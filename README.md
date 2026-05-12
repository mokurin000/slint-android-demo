# slint-android-demo

```bash
# Install Android platform 30
sdkmanager "--sdk_root=$ANDROID_HOME" --install "platforms;android-30"
# Build APK
cargo apk run --target aarch64-linux-android --lib
```
