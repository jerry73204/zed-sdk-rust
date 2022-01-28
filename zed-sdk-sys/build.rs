use anyhow::{ensure, Result};
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
    #[cfg(feature = "generate-bindings")]
    {
        use anyhow::anyhow;
        use std::fs;

        let include_dir = zed_dir.join("include");
        let mut builder = bindgen::builder()
            .header(format!("{}/sl/c_api/types_c.h", include_dir.display()))
            .header(format!(
                "{}/sl/c_api/zed_interface.h",
                include_dir.display()
            ))
            .clang_arg(format!("-I{}", include_dir.display()));

        let enum_types =
            fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/config/enums.txt"))?;
        for name in enum_types.lines() {
            builder = builder.rustified_non_exhaustive_enum(name);
        }

        let bindings = builder
            .generate()
            .map_err(|()| anyhow!("unable to run bindgen"))?;

        bindings.write_to_file(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bindings.rs"))?;
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
