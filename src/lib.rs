#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) -> Result<(), Box<dyn std::error::Error>> {
    use android_logger::Config;

    slint::android::init(app).unwrap();

    android_logger::init_once(Config::default().with_max_level(log::LevelFilter::Debug));

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
