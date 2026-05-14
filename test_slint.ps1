$env:SLINT_LIVE_PREVIEW = 1

while ($true) {
    cargo build --features slint/live-preview

    if ($?) {
        break;
    }
}

./target/debug/slint-android-demo.exe
