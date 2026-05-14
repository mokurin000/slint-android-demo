fn main() {
    if option_env!("FIXED_OPPO_SANS").is_some() {
        slint_build::compile_with_config(
            "ui/fixed-weight.slint",
            slint_build::CompilerConfiguration::new()
                .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer),
        )
    } else {
        slint_build::compile_with_config(
            "ui/app.slint",
            slint_build::CompilerConfiguration::new()
                .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer),
        )
    }
    .unwrap()
}
