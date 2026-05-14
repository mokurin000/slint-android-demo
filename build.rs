#[cfg(not(target_os = "android"))]
use slint_build::{CompilerConfiguration, EmbedResourcesKind};

fn main() {
    #[cfg(target_os = "android")]
    slint_build::compile("ui/app.slint").unwrap();

    #[cfg(not(target_os = "android"))]
    slint_build::compile_with_config(
        "ui/desktop.slint",
        CompilerConfiguration::new().embed_resources(EmbedResourcesKind::AsAbsolutePath),
    )
    .unwrap();
}
