use tonic::{Request, Response, Status, transport::Server};
use user_stats::{
    QueryRequest, QueryResponse, RawQueryRequest, RawQueryResponse,
    pb::user_stats_service_server::UserStatsService,
    user_stats_service_server::UserStatsServiceServer,
};

#[derive(Default)]
pub struct UserStatsServer;

#[tonic::async_trait]
impl UserStatsService for UserStatsServer {
    async fn query(
        &self,
        request: Request<QueryRequest>,
    ) -> Result<Response<QueryResponse>, Status> {
        todo!()
    }

    async fn raw_query(
        &self,
        request: Request<RawQueryRequest>,
    ) -> Result<Response<RawQueryResponse>, Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "[::1]:50051".parse().unwrap();
    let server = UserStatsServer::default();

    println!("UserServer listening on {addr}");

    Server::builder()
        .add_service(UserStatsServiceServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
