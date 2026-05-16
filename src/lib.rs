use std::{
    fs::{OpenOptions, create_dir_all},
    io::Write,
    path::Path,
};

slint::include_modules!();

pub fn place_file(bytes: impl AsRef<[u8]>, path: impl AsRef<Path>) {
    let path = path.as_ref();
    if let Some(dir) = path.parent() {
        _ = create_dir_all(dir);
    }

    let Ok(mut file) = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
    else {
        return;
    };

    _ = file.write_all(bytes.as_ref());
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) -> Result<(), Box<dyn std::error::Error>> {
    use std::{env, path::PathBuf};

    use android_logger::Config;

    let data_dir = PathBuf::from("/data/user")
        .join(unsafe { libc::getuid() / 100000 }.to_string())
        .join("com.example.slint")
        .join("files");
    let android_root = data_dir.to_string_lossy().into_owned();

    {
        unsafe {
            env::set_var("ANDROID_ROOT", android_root);
        }

        place_file(
            include_bytes!("../test/ColorOS/SysFont-Regular.ttf"),
            data_dir.join("fonts").join("SysFont-Regular.ttf"),
        );
        place_file(
            include_bytes!("../test/ColorOS/SysSans-Hans-Regular.ttf"),
            data_dir.join("fonts").join("SysSans-Hans-Regular.ttf"),
        );
    }

    android_logger::init_once(Config::default().with_max_level(log::LevelFilter::Info));
    slint::android::init(app).unwrap();

    AppWindow::new()?.run()?;
    Ok(())
}
