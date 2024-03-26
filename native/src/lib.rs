use std::time::Instant;
use std::{sync::Arc, thread};

mod dtls_default;
use coap::{client::CoAPClient, dtls::DtlsConnection};
use coap_lite::RequestType;
use coap_lite::{CoapOption, CoapRequest, RequestType as Method};
use log::debug;
use num_traits::FromPrimitive;
use protobuf::{Message, ProtobufError, ProtobufResult};

#[cfg(target_os = "android")]
use {android_logger::Config, log::Level};

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub fn initialize_logging() {
    #[cfg(target_os = "android")]
    {
        android_logger::init_once(Config::default().with_min_level(Level::Debug));
    }
    debug!("logging initialized");
}

pub mod greeter {
    tonic::include_proto!("com.google.greeting");
}

use greeter::{
    greeter_server::{Greeter, GreeterServer},
    HelloRequest, HelloResponse,
};

use tokio::runtime::Runtime;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
pub struct MyGreeter {
    //client: Arc<Mutex<CoAPClient<DtlsConnection>>>,
}
//impl MyGreeter {
//    pub fn new(client: CoAPClient<DtlsConnection>) -> Self {
//        Self {
//            client: Arc::new(Mutex::new(client)),
//        }
//    }
//}

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

#[no_mangle]
pub extern "C" fn device_lib_start() {
    initialize_logging();
    thread::spawn(move || {
        debug!("spawning thread");

        let Ok(rt) = Runtime::new() else {
            debug!("could not spawn tokio runtime");
            return;
        };
        let _device_lib = rt.block_on(device_lib_main());
    });
}

use std::default::Default;
use tracing::{span, Level as TraceLevel};

use crate::dtls_default::get_client;
async fn do_dtls_request_through_localhost() {
    //dtls_default::client_get().await;
    let domain = format!("10.0.2.2:{}", 7777);
    let mut client = get_client().await;
    let start_time = Instant::now();
    let resp = client
        .request_path("/block", Method::Get, None, None, Some(domain.to_string()))
        .await
        .unwrap();
    let end_time = Instant::now();
    let duration = end_time - start_time;

    debug!("response len: {:?}", resp.message.payload.len());
    debug!("duration: {} ms", duration.as_millis());
    debug!(
        "throughput: {}B/s",
        1000.0 * resp.message.payload.len() as f32 / duration.as_millis() as f32
    );
    //let greeter = MyGreeter::new(get_client().await);
    debug!("starting greeter");
}

async fn device_lib_main() {
    let addr = "127.0.0.1:50051".parse().expect("not an address");
    Server::builder()
        .trace_fn(|arg| {
            debug!("recv: {:?}", arg);
            span!(TraceLevel::DEBUG, "trace", ?arg)
        })
        .add_service(GreeterServer::new(MyGreeter::default()))
        .serve(addr)
        .await
        .expect("failed to start server");
    debug!("oh no server failed");
}
