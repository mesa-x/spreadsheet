//! An example gRPC server using Tonic
//!
//!

use tonic::{transport::Server, Request, Response, Status};

// use async_timer::oneshot::{Oneshot, Timer};
use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
use rand::{thread_rng, Rng};
use std::pin::Pin;
use tokio_stream::Stream;

///
pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let ra = request.remote_addr();
        println!("Got a request from {:?}", ra);

        let reply = hello_world::HelloReply {
            message: format!(
                "Hello {}! This is from Rust-land!! From address {:?}.",
                request.into_inner().name,
                ra
            ),
            count: 0,
        };
        Ok(Response::new(reply))
    }

    type HelloOverAgainStream =
        Pin<Box<dyn Stream<Item = Result<HelloReply, Status>> + Send + Sync /*+ 'static*/>>;

    async fn hello_over_again(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<Self::HelloOverAgainStream>, Status> {
        let ra = request.remote_addr();
        let name = request.into_inner().name;
        let mut cnt = 0;

        let mut rng = thread_rng();

        let total_cnt: u32 = rng.gen_range(2..20);

        let output = async_stream::try_stream! {
            while cnt < total_cnt {
                cnt += 1;
                yield hello_world::HelloReply{
                    message: format!("Hello {}! Count {}. ra {:?}", name, cnt, ra),
                    count: cnt as i32,
                };
            }

            println!("Done writing for {:?}, total count {}", ra, total_cnt);
        };

        Ok(Response::new(Box::pin(output) as Self::HelloOverAgainStream))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
