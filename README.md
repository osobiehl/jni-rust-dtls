# RUST interop GRPC minimal example

## this current example does NOT use DTLS, only GRPC, since you need a DTLS client to use DTLS over CoAP

## how to setup tools to build

install rustup
install rust and cargo
have ndk installed

run `rustup target add aarch64-linux-android` to add android as a compilation target

cd into `native` and edit the `.cargo` file 


```toml
[env]
CC_aarch64-linux-android="/Users/osobiehl/Library/Android/sdk/ndk/26.1.10909125/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android30-clang"
AR_aarch64-linux-android="/Users/osobiehl/Library/Android/sdk/ndk/26.1.10909125/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android-ar"
```
**NOTE**
IF you want to build for other targets (like x86 android), you need to set the appropriate environment variables for this target!


set these environment variables to the necessary C compiler of your android NDK. DO NOT USE SHELL VARIABLES. You can also automate this later by setting ENV vars using whatever build tool you want


## how to build native library

`cd native`
`cargo build --target=aarch64-linux-android --profile=release`

## making sure library is linked
the release binary should be symlinked to the cd android/app/src/main/jniLibs/arm64-v8a/ folder. 

you should see something like the following
```shell
ls -l  android/app/src/main/jniLibs/arm64-v8a 
total 0
lrwxr-xr-x  1 osobiehl  staff  74 Mar 22 19:51 libnative.so -> ../../../../../../native/target/aarch64-linux-android/release/libnative.so

```

# Running the example

once the native lib is built, start up your ANDROID emulator and run main.dart. You should see a "hello from Rust" in the app console!


A few resources to get you started if this is your first Flutter project:

- [Lab: Write your first Flutter app](https://docs.flutter.dev/get-started/codelab)
- [Cookbook: Useful Flutter samples](https://docs.flutter.dev/cookbook)

For help getting started with Flutter development, view the
[online documentation](https://docs.flutter.dev/), which offers tutorials,
samples, guidance on mobile development, and a full API reference.
