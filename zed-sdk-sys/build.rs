use anyhow::{anyhow, ensure, Result};
use std::{env, path::PathBuf};

const ZED_DIR_ENV: &str = "ZED_DIR";

fn main() -> Result<()> {
    // guess ZED_DIR
    let zed_dir: PathBuf = match env::var_os(ZED_DIR_ENV) {
        Some(dir) => dir.into(),
        None => {
            let guess = PathBuf::from("/usr/local/zed");
            ensure!(
                guess.exists(),
                "unable to find ZED directory. Is it installed?"
            );
            guess
        }
    };

    // code generation
    {
        let include_dir = zed_dir.join("include");
        let bindings = bindgen::builder()
            .header(format!("{}/sl/c_api/types_c.h", include_dir.display()))
            .header(format!(
                "{}/sl/c_api/zed_interface.h",
                include_dir.display()
            ))
            .clang_arg(format!("-I{}", include_dir.display()))
            .generate()
            .map_err(|()| anyhow!("unable to run bindgen"))?;

        let out_dir: PathBuf = env::var_os("OUT_DIR").unwrap().into();
        bindings.write_to_file(out_dir.join("bindings.rs"))?;
    }

    // linking
    {
        let lib_dir = zed_dir.join("lib");
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
        println!("cargo:rustc-link-lib=dylib=sl_zed");
        println!("cargo:rustc-link-lib=dylib=sl_zed_c");
    }

    Ok(())
}
