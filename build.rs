use std::env;

#[cfg(not(target_os = "android"))]
use slint_build::{CompilerConfiguration, EmbedResourcesKind};

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("android") {
        slint_build::compile("ui/app.slint").unwrap();
    } else {
        slint_build::compile_with_config(
            "ui/desktop.slint",
            CompilerConfiguration::new().embed_resources(EmbedResourcesKind::AsAbsolutePath),
        )
        .unwrap();
    }
}
