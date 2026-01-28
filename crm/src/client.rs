use crm::pb::{CreateUserRequest, user_service_client::UserServiceClient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = UserServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(CreateUserRequest {
        name: "John Doe".into(),
        email: "john.doe@example.com".into(),
    });

    let response = client.create_user(request).await?;

    println!("RESPONSE={response:?}");

    Ok(())
}
