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


# Adding protobuf code
to add protobuf code, first add your `.proto` file in `protos`
then, modify `native/build.rs` to compile the new proto file
```rust
tonic_build::compile_protos("../protos/your_proto.proto")?;
```

then, in your rust code, add the following
```rust
pub mod greeter {
    tonic::include_proto!("you_proto_name");
}
```
note that if you gave your proto file a package name, it will have that name instead

afterwards, declare a struct and implement the methods on the struct
```rust
#[derive(Debug, Default)]
pub struct MyGreeter {
}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloResponse>, Status> {
        // Return an instance of type HelloResponse
        debug!("Got a request: {:?}", request);

        let reply = HelloResponse {
            message: format!("Hello {}!", request.into_inner().name), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
    async fn say_hello_again(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloResponse>, Status> {
        debug!("hello again");
        self.say_hello(request).await
    }
}
```

# Running the example

once the native lib is built, start up your ANDROID emulator and run main.dart. You should see a "hello from Rust" in the app console!


A few resources to get you started if this is your first Flutter project:

- [Lab: Write your first Flutter app](https://docs.flutter.dev/get-started/codelab)
- [Cookbook: Useful Flutter samples](https://docs.flutter.dev/cookbook)

For help getting started with Flutter development, view the
[online documentation](https://docs.flutter.dev/), which offers tutorials,
samples, guidance on mobile development, and a full API reference.
