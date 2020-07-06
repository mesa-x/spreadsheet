use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://localhost:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "dpp".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response.get_ref().message);

    Ok(())
}
