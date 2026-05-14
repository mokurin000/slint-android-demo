#[cfg(target_os = "android")]
use std::path::Path;

slint::include_modules!();

#[cfg(target_os = "android")]
fn place_file(path: impl AsRef<Path>, bytes: impl AsRef<[u8]>) {
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(false)
        .truncate(true)
        .write(true)
        .open(path)
    {
        use std::io::Write;

        _ = f.write_all(bytes.as_ref());
    }
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) -> Result<(), Box<dyn std::error::Error>> {
    use std::{fs::create_dir_all, path::PathBuf};

    use android_logger::Config;

    let base_path = PathBuf::from(format!("/data/user/{}/com.example.slint/fonts", unsafe {
        libc::getuid() / 100000
    }));
    _ = create_dir_all("");
    place_file(
        base_path.join("oppo.ttf"),
        include_bytes!("../SysSans-Hans-Regular.ttf"),
    );
    place_file(
        base_path.join("noto.ttf"),
        include_bytes!("../NotoSansCJK-Regular.ttc"),
    );

    android_logger::init_once(Config::default().with_max_level(log::LevelFilter::Debug));
    slint::android::init(app).unwrap();

    AppWindow::new()?.run()?;
    Ok(())
}
