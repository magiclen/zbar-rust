use std::{env, path::PathBuf};

const MIN_VERSION: &str = "0.22";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // These are printed up front so that a change is noticed even when the variable is never read on this run.
    for name in [
        "ZBAR_LIBS",
        "ZBAR_LIB_DIRS",
        "ZBAR_INCLUDE_DIRS",
        "ZBAR_DIR",
        "ZBAR_STATIC",
        "ZBAR_DYLIB_STDCPP",
    ] {
        println!("cargo:rerun-if-env-changed={name}");
    }

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let prefix = env::var_os("ZBAR_DIR").map(PathBuf::from);
    let lib_dirs = env::var_os("ZBAR_LIB_DIRS")
        .map(|dirs| env::split_paths(&dirs).collect::<Vec<_>>())
        .or_else(|| prefix.as_ref().map(|dir| vec![dir.join("lib")]))
        .or_else(|| (target_os == "freebsd").then(|| vec![PathBuf::from("/usr/lib")]));
    let include_dirs = env::var_os("ZBAR_INCLUDE_DIRS")
        .map(|dirs| env::split_paths(&dirs).collect::<Vec<_>>())
        .or_else(|| prefix.as_ref().map(|dir| vec![dir.join("include")]))
        .or_else(|| (target_os == "freebsd").then(|| vec![PathBuf::from("/usr/include")]));
    let libs = env::var("ZBAR_LIBS")
        .ok()
        .map(|libs| libs.split(':').map(str::to_string).collect::<Vec<_>>());
    let forced_static = env::var_os("ZBAR_STATIC").map(|value| value != "0");

    let include_dirs = if lib_dirs.is_none() && libs.is_none() {
        let statik = forced_static.unwrap_or_else(|| {
            let library = run_pkg_config(false);

            if library.libs.iter().any(|lib| lib == "zbar") {
                has_static(&library.link_paths, "zbar") && !has_dylib(&library.link_paths, "zbar")
            } else {
                library.link_files.iter().any(|file| is_archive(&file.to_string_lossy()))
                    || library.libs.iter().any(|lib| lib.strip_prefix(':').is_some_and(is_archive))
            }
        });

        let library = run_pkg_config(statik);
        link_pkg_config(&library, statik);

        include_dirs.unwrap_or(library.include_paths)
    } else {
        let mut library = None;
        let lib_dirs = lib_dirs.unwrap_or_else(|| {
            library
                .get_or_insert_with(|| run_pkg_config(forced_static.unwrap_or(false)))
                .link_paths
                .clone()
        });
        let libs = libs.unwrap_or_else(|| vec!["zbar".to_string()]);

        for dir in &lib_dirs {
            assert!(dir.is_dir(), "ZBar library directory does not exist: {}", dir.display());
            println!("cargo:rustc-link-search=native={}", dir.display());
        }

        let statik = forced_static.unwrap_or_else(|| determine_static(&lib_dirs, &libs));
        let kind = if statik { "static" } else { "dylib" };

        for lib in &libs {
            println!("cargo:rustc-link-lib={kind}={lib}");
        }

        include_dirs.unwrap_or_else(|| {
            library.get_or_insert_with(|| run_pkg_config(statik)).include_paths.clone()
        })
    };

    for dir in &include_dirs {
        assert!(dir.is_dir(), "ZBar include directory does not exist: {}", dir.display());
        println!("cargo:include={}", dir.display());
    }

    match env::var("ZBAR_DYLIB_STDCPP").as_deref() {
        Ok("0") | Err(_) => (),
        Ok(_) => println!("cargo:rustc-link-lib=dylib=stdc++"),
    }
}

fn has_static(dirs: &[PathBuf], lib: &str) -> bool {
    dirs.iter().any(|dir| {
        dir.join(format!("lib{lib}.a")).is_file() || dir.join(format!("{lib}.lib")).is_file()
    })
}

fn has_dylib(dirs: &[PathBuf], lib: &str) -> bool {
    dirs.iter().any(|dir| {
        dir.join(format!("lib{lib}.so")).is_file()
            || dir.join(format!("{lib}.dll")).is_file()
            || dir.join(format!("lib{lib}.dylib")).is_file()
    })
}

fn determine_static(dirs: &[PathBuf], libs: &[String]) -> bool {
    let can_static = libs.iter().all(|lib| has_static(dirs, lib));
    let can_dylib = libs.iter().all(|lib| has_dylib(dirs, lib));

    match (can_static, can_dylib) {
        (_, true) => false,
        (true, false) => true,
        (false, false) => {
            panic!(
                "ZBar libdirs at `{dirs:?}` do not contain the required files to either \
                 statically or dynamically link ZBar"
            );
        },
    }
}

fn link_pkg_config(library: &pkg_config::Library, statik: bool) {
    for dir in &library.link_paths {
        println!("cargo:rustc-link-search=native={}", dir.display());
    }

    for dir in &library.framework_paths {
        println!("cargo:rustc-link-search=framework={}", dir.display());
    }

    for lib in &library.libs {
        if let Some(name) = lib.strip_prefix(':') {
            link_file_name(name, statik);
            continue;
        }

        let kind = if statik
            && (lib == "zbar"
                || (has_static(&library.link_paths, lib) && !has_dylib(&library.link_paths, lib)))
        {
            "static"
        } else {
            "dylib"
        };

        println!("cargo:rustc-link-lib={kind}={lib}");
    }

    for framework in &library.frameworks {
        println!("cargo:rustc-link-lib=framework={framework}");
    }

    for file in &library.link_files {
        let dir = file.parent().unwrap();
        let name = file.file_name().unwrap().to_string_lossy();

        // Keep the exact file name and pass the library on to downstream Rust crates.
        println!("cargo:rustc-link-search=native={}", dir.display());
        link_file_name(&name, statik);
    }

    for args in &library.ld_args {
        if !args.is_empty() {
            println!("cargo:rustc-link-arg=-Wl,{}", args.join(","));
        }
    }
}

fn is_archive(name: &str) -> bool {
    name.ends_with(".a") && !name.ends_with(".dll.a")
}

fn link_file_name(name: &str, statik: bool) {
    let kind =
        if is_archive(name) || (statik && name.ends_with(".lib")) { "static" } else { "dylib" };

    println!("cargo:rustc-link-lib={kind}:+verbatim={name}");
}

fn run_pkg_config(statik: bool) -> pkg_config::Library {
    pkg_config::Config::new()
        .statik(statik)
        .cargo_metadata(false)
        .atleast_version(MIN_VERSION)
        .probe("zbar")
        .expect("Couldn't find ZBar 0.22 or later with pkg-config")
}
