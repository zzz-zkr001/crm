use crm::pb::{
    CreateUserRequest, CreateUserResponse, GetUserRequest, GetUserResponse, User,
    user_service_server::{UserService, UserServiceServer},
};
use tonic::{Request, Response, Status, transport::Server};

#[derive(Default)]
pub struct UserServer;

#[tonic::async_trait]
impl UserService for UserServer {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> anyhow::Result<Response<GetUserResponse>, Status> {
        let user = request.into_inner();
        println!("get_user: {:?}", user);
        Ok(Response::new(GetUserResponse {
            user: Some(User::default()),
        }))
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let input = request.into_inner();
        let user = User::new(1 as u64, input.name.as_str(), input.email.as_str());
        println!("create_user: {:?}", user);
        Ok(Response::new(CreateUserResponse { user: Some(user) }))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "[::1]:50051".parse().unwrap();
    let server = UserServer::default();

    println!("UserServer listening on {addr}");

    Server::builder()
        .add_service(UserServiceServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
