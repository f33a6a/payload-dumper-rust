#[cfg(windows)]
fn find_libzip(use_static: bool) {
    let mut config = vcpkg::Config::new();

    if use_static {
        config.r#static(true);
        println!("cargo:rustc-link-lib=static=zip");
    } else {
        println!("cargo:rustc-link-lib=zip");
    }

    config
        .find_package("libzip")
        .expect("Failed to find libzip via vcpkg");
}

#[cfg(not(windows))]
fn find_libzip(use_static: bool) {
    let mut config = pkg_config::Config::new();

    if use_static {
        config.statik(true);
        println!("cargo:rustc-link-lib=static=zip");
    } else {
        println!("cargo:rustc-link-lib=zip");
    }

    config
        .probe("libzip")
        .expect("Failed to find libzip via pkg-config");
}
