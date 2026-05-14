$env:SLINT_LIVE_PREVIEW = 1

while ($true) {
    cargo build --features slint/live-preview

    if (-not $?) {
        Remove-Item .\target\debug\slint_android_demo.pdb -ErrorAction SilentlyContinue
    }
    else {
        break;
    }
}

./target/debug/slint-android-demo.exe
