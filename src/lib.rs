slint::include_modules!();

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) -> Result<(), Box<dyn std::error::Error>> {
    use std::{fs, path::PathBuf};

    use android_logger::Config;

    let path = PathBuf::from("/data/data/com.example.slint/SarasaUiSC-Regular.ttf");

    if !path.exists()
        && let Ok(mut file) = fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)
    {
        use std::io::Write;

        _ = file.write_all(include_bytes!("../SarasaUiSC-Regular.ttf"));
    }

    slint::android::init(app).unwrap();
    android_logger::init_once(Config::default().with_max_level(log::LevelFilter::Debug));

    AppWindow::new()?.run()?;
    Ok(())
}
