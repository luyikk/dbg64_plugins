use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=pluginsdk/x64bridge");
    println!("cargo:rustc-link-lib=pluginsdk/x64dbg");
    println!("cargo:rustc-link-lib=pluginsdk/capstone/capstone_x64");
    println!("cargo:rustc-link-lib=pluginsdk/DeviceNameResolver/DeviceNameResolver_x64");
    println!("cargo:rustc-link-lib=pluginsdk/jansson/jansson_x64");
    println!("cargo:rustc-link-lib=pluginsdk/lz4/lz4_x64");
    println!("cargo:rustc-link-lib=pluginsdk/TitanEngine/TitanEngine_x64");
    println!("cargo:rustc-link-lib=pluginsdk/XEDParse/XEDParse_x64");
    println!("cargo:rustc-link-lib=pluginsdk/yara/yara_x64");

    println!("cargo:rerun-if-changed=pluginsdk/bridgemain.h");
    println!("cargo:rerun-if-changed=pluginsdk/_plugins.h");
    println!("cargo:rerun-if-changed=pluginsdk/_scriptapi_argument.h");
    println!("cargo:rerun-if-changed=pluginsdk/_scriptapi_assembler.h");
    println!("cargo:rerun-if-changed=pluginsdk/_scriptapi_bookmark.h");
    println!("cargo:rerun-if-changed=pluginsdk/_scriptapi_comment.h");
    println!("cargo:rerun-if-changed=pluginsdk/_scriptapi_debug.h");
    println!("cargo:rerun-if-changed=pluginsdk/_scriptapi_flag.h");
    println!("cargo:rerun-if-changed=pluginsdk/_scriptapi_function.h");
    println!("cargo:rerun-if-changed=pluginsdk/_scriptapi_gui.h");
    println!("cargo:rerun-if-changed=pluginsdk/_scriptapi_label.h");
    println!("cargo:rerun-if-changed=pluginsdk/_scriptapi_memory.h");
    println!("cargo:rerun-if-changed=pluginsdk/_scriptapi_misc.h");
    println!("cargo:rerun-if-changed=pluginsdk/_scriptapi_module.h");
    println!("cargo:rerun-if-changed=pluginsdk/_scriptapi_pattern.h");
    println!("cargo:rerun-if-changed=pluginsdk/_scriptapi_register.h");
    println!("cargo:rerun-if-changed=pluginsdk/_scriptapi_stack.h");
    println!("cargo:rerun-if-changed=pluginsdk/_scriptapi_symbol.h");
    println!("cargo:rerun-if-changed=pluginsdk/capstone/capstone.h");
    println!("cargo:rerun-if-changed=pluginsdk/DeviceNameResolver/DeviceNameResolver.h");
    println!("cargo:rerun-if-changed=pluginsdk/jansson/jansson.h");
    println!("cargo:rerun-if-changed=pluginsdk/lz4/lz4file.h");
    println!("cargo:rerun-if-changed=pluginsdk/TitanEngine/TitanEngine.h");
    println!("cargo:rerun-if-changed=pluginsdk/XEDParse/XEDParse.h");
    println!("cargo:rerun-if-changed=pluginsdk/yara/yara.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = autocxx_bindgen::Builder::default()
        .clang_args(&["-x", "c++","-std=c++14","-stdlib=libc++"])
        .opaque_type("std::.*")
        .opaque_type(".*YR_AC_MATCH.*")
        .opaque_type(".*IMAGE_TLS_DIRECTORY.*")
        .no_debug("__BindgenBitfieldUnit")
        .no_debug("_MINIDUMP_MEMORY64_LIST")
        .layout_tests(false)
        .derive_hash(false)
        .derive_copy(true)
        .derive_partialeq(false)
        .derive_ord(false)
        .derive_partialord(false)
        .derive_default(false)
        .derive_eq(false)
        // The input header we would like to generate
        // bindings for.
        .header("pluginsdk/bridgemain.h")
        .header("pluginsdk/_plugins.h")
        .header("pluginsdk/_scriptapi_argument.h")
        .header("pluginsdk/_scriptapi_assembler.h")
        .header("pluginsdk/_scriptapi_bookmark.h")
        .header("pluginsdk/_scriptapi_comment.h")
        .header("pluginsdk/_scriptapi_debug.h")
        .header("pluginsdk/_scriptapi_flag.h")
        .header("pluginsdk/_scriptapi_function.h")
        .header("pluginsdk/_scriptapi_gui.h")
        .header("pluginsdk/_scriptapi_label.h")
        .header("pluginsdk/_scriptapi_memory.h")
        .header("pluginsdk/_scriptapi_misc.h")
        .header("pluginsdk/_scriptapi_module.h")
        .header("pluginsdk/_scriptapi_pattern.h")
        .header("pluginsdk/_scriptapi_register.h")
        .header("pluginsdk/_scriptapi_stack.h")
        .header("pluginsdk/_scriptapi_symbol.h")
        .header("pluginsdk/capstone/capstone.h")
        .header("pluginsdk/DeviceNameResolver/DeviceNameResolver.h")
        .header("pluginsdk/jansson/jansson.h")
        .header("pluginsdk/lz4/lz4file.h")
        .header("pluginsdk/TitanEngine/TitanEngine.h")
        .header("pluginsdk/XEDParse/XEDParse.h")
        .header("pluginsdk/yara/yara.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(autocxx_bindgen::CargoCallbacks))
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
