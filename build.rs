extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    //println!("cargo:rustc-link-lib=bz2");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .clang_arg("-Ivendor/emlib/inc")
        .clang_arg("-Ivendor/device/EFR32BG1P/Include/")
        .clang_arg("-Ivendor/CMSIS/CMSIS/Include")
        .clang_arg(board_define().expect("You must use one of the features to define a board"))
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
        .ctypes_prefix("ctypes")
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
    return rv("-DEFR32BG1P232F256GJ43");
    #[cfg(feature = "efr32bg1p232f256gm32")]
    return rv("-DEFR32BG1P232F256GM32");
    #[cfg(feature = "efr32bg1p232f256gm48")]
    return rv("-DEFR32BG1P232F256GM48");
    #[cfg(feature = "efr32bg1p233f256gm48")]
    return rv("-DEFR32BG1P233F256GM48");
    #[cfg(feature = "efr32bg1p332f256gj43")]
    return rv("-DEFR32BG1P332F256GJ43");
    #[cfg(feature = "efr32bg1p332f256gm32")]
    return rv("-DEFR32BG1P332F256GM32");
    #[cfg(feature = "efr32bg1p332f256gm48")]
    return rv("-DEFR32BG1P332F256GM48");
    #[cfg(feature = "efr32bg1p333f256gm48")]
    return rv("-DEFR32BG1P333F256GM48");
    #[cfg(feature = "efr32bg1p333f256im48")]
    return rv("-DEFR32BG1P333F256IM48");

    return None;
}
