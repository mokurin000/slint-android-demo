slint::include_modules!();

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) -> Result<(), Box<dyn std::error::Error>> {
    use std::{fs, path::PathBuf};

    use android_logger::Config;

    let fonts = if option_env!("FIXED_OPPO_SANS").is_some() {
        vec![(
            PathBuf::from("/data/data/com.example.slint/SysSans-Hans-Regular-400-instanced.ttf"),
            include_bytes!("../SysSans-Hans-Regular-400-instanced.ttf").as_slice(),
        )]
    } else {
        vec![
            (
                PathBuf::from("/data/data/com.example.slint/SarasaUiSC-Regular.ttf"),
                include_bytes!("../SarasaUiSC-Regular.ttf").as_slice(),
            ),
            (
                PathBuf::from("/data/data/com.example.slint/SysSans-Hans-Regular.ttf"),
                include_bytes!("../SysSans-Hans-Regular.ttf"),
            ),
            (
                PathBuf::from("/data/data/com.example.slint/NotoSansCJK-Regular.ttc"),
                include_bytes!("../NotoSansCJK-Regular.ttc"),
            ),
        ]
    };
    for (path, bytes) in fonts {
        if !path.exists()
            && let Ok(mut file) = fs::OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(path)
        {
            use std::io::Write;

            _ = file.write_all(bytes);
        }
    }

    slint::android::init(app).unwrap();
    android_logger::init_once(Config::default().with_max_level(log::LevelFilter::Debug));

    AppWindow::new()?.run()?;
    Ok(())
}
