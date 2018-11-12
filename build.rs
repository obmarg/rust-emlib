extern crate bindgen;
extern crate cc;
extern crate walkdir;

use std::env;
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    //println!("cargo:rustc-link-lib=bz2");
    for entry in WalkDir::new("vendor").into_iter().filter_map(|e| e.ok()) {
        println!("cargo:rerun-if-changed={}", entry.path().display());
    }

    build_emlib_sources();
    build_emlib_bindings();
}

fn build_emlib_sources() {
    cc::Build::new()
        .target("arm-none-eabihf")
        .compiler("arm-none-eabi-gcc")
        .include("vendor/emlib/inc")
        .include(board_include_path())
        .include("vendor/CMSIS/CMSIS/Include")
        .define(
            &board_define().expect("You must use one of the features to define a board"),
            "1",
        ).flag("-ffunction-sections")
        .warnings(true)
        .opt_level(2)
        .debug(true)
        .flag("-mthumb")
        .flag("-mcpu=cortex-m4")
        .flag("-fomit-frame-pointer")
        .flag("-fno-short-enums")
        .flag("-std=c99")
        .flag("-mfpu=fpv4-sp-d16")
        .flag("-mfloat-abi=hard")
        .files(source_files())
        .file(board_system_file())
        .compile("emlib");

    println!("cargo:rustc-link-lib=static=emlib");
}

fn build_emlib_bindings() {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .clang_arg("-Ivendor/emlib/inc")
        .clang_arg(format!("-I{}", board_include_path()))
        .clang_arg("-Ivendor/CMSIS/CMSIS/Include")
        .clang_arg(
            format!("-D{}=1", board_define().expect("You must use one of the features to define a board"))
        )
        .clang_arg("--target=thumbv7em-none-eabihf")
        .clang_arg("-mcpu=cortex-m4")
        .clang_arg("-mthumb")
        .clang_arg("-mfloat-abi=hard")
    // The input header we would like to generate
    // bindings for.
        .header("src/wrapper.h")
    // We need to use this in no_std projects, so don't use
    // anything in core.
        .use_core()
    // We also need to configure a custom ctypes library as otherwise bindgen
    // tries to use std for that.
        .ctypes_prefix("super::ctypes")
    // Finish the builder and generate the bindings.
        .generate()
    // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn board_define() -> Option<String> {
    fn rv(s: &str) -> Option<String> {
        Some(s.to_string())
    }

    #[cfg(feature = "efr32bg1p232f256gj43")]
    return rv("EFR32BG1P232F256GJ43");
    #[cfg(feature = "efr32bg1p232f256gm32")]
    return rv("EFR32BG1P232F256GM32");
    #[cfg(feature = "efr32bg1p232f256gm48")]
    return rv("EFR32BG1P232F256GM48");
    #[cfg(feature = "efr32bg1p233f256gm48")]
    return rv("EFR32BG1P233F256GM48");
    #[cfg(feature = "efr32bg1p332f256gj43")]
    return rv("EFR32BG1P332F256GJ43");
    #[cfg(feature = "efr32bg1p332f256gm32")]
    return rv("EFR32BG1P332F256GM32");
    #[cfg(feature = "efr32bg1p332f256gm48")]
    return rv("EFR32BG1P332F256GM48");
    #[cfg(feature = "efr32bg1p333f256gm48")]
    return rv("EFR32BG1P333F256GM48");
    #[cfg(feature = "efr32bg1p333f256im48")]
    return rv("EFR32BG1P333F256IM48");
}

fn board_system_file() -> String {
    let device_fam = device_family().expect("You must use one of the features to define a board");
    format!(
        "vendor/device/{}/Source/system_{}.c",
        device_fam,
        device_fam.to_lowercase()
    )
}

fn board_include_path() -> String {
    let device_fam = device_family().expect("You must use one of the features to define a board");
    format!("vendor/device/{}/Include", device_fam)
}

fn device_family() -> Option<String> {
    if cfg!(feature = "efr32bg1p232f256gj43")
        || cfg!(feature = "efr32bg1p232f256gm32")
        || cfg!(feature = "efr32bg1p232f256gm48")
        || cfg!(feature = "efr32bg1p233f256gm48")
        || cfg!(feature = "efr32bg1p332f256gj43")
        || cfg!(feature = "efr32bg1p332f256gm32")
        || cfg!(feature = "efr32bg1p332f256gm48")
        || cfg!(feature = "efr32bg1p333f256gm48")
        || cfg!(feature = "efr32bg1p333f256im48")
    {
        return Some("EFR32BG1P".to_string());
    }

    return None;
}

fn source_files() -> impl Iterator<Item = String> {
    [
        "em_gpio.c",
        "em_adc.c",
        "em_cmu.c",
        "em_msc.c",
        "em_cryotimer.c",
        "em_rtcc.c",
        "em_system.c",
        "em_leuart.c",
        "em_usart.c",
        "em_ldma.c",
        "em_letimer.c",
        "em_wdog.c",
        "em_rmu.c",
    ]
        .iter()
        .map(|p| format!("vendor/emlib/src/{}", p))
}
