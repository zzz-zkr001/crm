use tonic::transport::Server;
use user_stats::{UserStatsServer, user_stats_service_server::UserStatsServiceServer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "[::1]:50051".parse().unwrap();
    let server = UserStatsServiceServer::from(UserStatsServer::default());

    println!("UserServer listening on {addr}");

    Server::builder().add_service(server).serve(addr).await?;

    Ok(())
}
