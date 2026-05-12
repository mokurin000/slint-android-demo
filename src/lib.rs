#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) -> Result<(), Box<dyn std::error::Error>> {
    use android_logger::Config;

    android_logger::init_once(Config::default().with_max_level(log::LevelFilter::Trace));
    slint::android::init(app).unwrap();

    slint::slint!(
        export component AppWindow inherits Window {
            Text {
                text: "测试";
            }
        }
    );

    AppWindow::new()?.run()?;
    Ok(())
}
