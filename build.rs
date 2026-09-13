use std::{collections::HashSet, env, path::PathBuf, sync::OnceLock};

const MIN_VERSION: &str = "0.10";

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

    let lib_dirs = find_zbar_lib_dirs(&target_os);

    for d in &lib_dirs {
        if !d.exists() {
            panic!("ZBar library directory does not exist: {}", d.display());
        }
        println!("cargo:rustc-link-search=native={}", d.display());
    }

    let include_dirs = find_zbar_include_dirs(&target_os);

    for d in &include_dirs {
        if !d.exists() {
            panic!("ZBar include directory does not exist: {}", d.display());
        }
        println!("cargo:include={}", d.display());
    }

    let libs = find_zbar_libs(&target_os);

    let kind = determine_mode(&lib_dirs, libs.as_slice());

    for lib in libs {
        println!("cargo:rustc-link-lib={kind}={lib}");
    }

    match env::var("ZBAR_DYLIB_STDCPP").as_deref() {
        Ok("0") | Err(_) => (),
        Ok(_) => println!("cargo:rustc-link-lib=dylib=stdc++"),
    }
}

fn split_dirs(dirs: &str) -> Vec<PathBuf> {
    dirs.split(':').map(PathBuf::from).collect()
}

fn find_zbar_lib_dirs(target_os: &str) -> Vec<PathBuf> {
    if let Ok(dirs) = env::var("ZBAR_LIB_DIRS") {
        return split_dirs(&dirs);
    }

    if let Ok(dir) = env::var("ZBAR_DIR") {
        return vec![PathBuf::from(dir).join("lib")];
    }

    if target_os == "freebsd" {
        return vec![PathBuf::from("/usr/lib")];
    }

    run_pkg_config().link_paths.clone()
}

fn find_zbar_include_dirs(target_os: &str) -> Vec<PathBuf> {
    if let Ok(dirs) = env::var("ZBAR_INCLUDE_DIRS") {
        return split_dirs(&dirs);
    }

    if let Ok(dir) = env::var("ZBAR_DIR") {
        return vec![PathBuf::from(dir).join("include")];
    }

    if target_os == "freebsd" {
        return vec![PathBuf::from("/usr/include")];
    }

    run_pkg_config().include_paths.clone()
}

fn find_zbar_libs(target_os: &str) -> Vec<String> {
    if let Ok(libs) = env::var("ZBAR_LIBS") {
        return libs.split(':').map(str::to_string).collect();
    }

    // pkg-config is not usually available for these targets.
    if target_os == "windows" || target_os == "freebsd" {
        return vec!["zbar".to_string()];
    }

    run_pkg_config().libs.clone()
}

fn determine_mode<T: AsRef<str>>(libdirs: &[PathBuf], libs: &[T]) -> &'static str {
    match env::var("ZBAR_STATIC").as_deref() {
        Ok("0") => return "dylib",
        Ok(_) => return "static",
        Err(_) => (),
    }

    let files = libdirs
        .iter()
        .flat_map(|d| {
            d.read_dir().unwrap_or_else(|error| {
                panic!("Couldn't read the ZBar library directory {}: {error}", d.display())
            })
        })
        .filter_map(|e| e.ok())
        .map(|e| e.file_name())
        .filter_map(|e| e.into_string().ok())
        .collect::<HashSet<_>>();

    let can_static = libs.iter().all(|l| {
        files.contains(&format!("lib{}.a", l.as_ref()))
            || files.contains(&format!("{}.lib", l.as_ref()))
    });
    let can_dylib = libs.iter().all(|l| {
        files.contains(&format!("lib{}.so", l.as_ref()))
            || files.contains(&format!("{}.dll", l.as_ref()))
            || files.contains(&format!("lib{}.dylib", l.as_ref()))
    });

    match (can_static, can_dylib) {
        (true, false) => "static",
        (false, true) => "dylib",
        (false, false) => {
            panic!(
                "ZBar libdirs at `{libdirs:?}` do not contain the required files to either \
                 statically or dynamically link ZBar"
            );
        },
        (true, true) => "dylib",
    }
}

fn run_pkg_config() -> &'static pkg_config::Library {
    static LIBRARY: OnceLock<pkg_config::Library> = OnceLock::new();

    LIBRARY.get_or_init(|| {
        pkg_config::Config::new()
            .cargo_metadata(false)
            .atleast_version(MIN_VERSION)
            .probe("zbar")
            .expect("Couldn't find ZBar with pkg-config")
    })
}
