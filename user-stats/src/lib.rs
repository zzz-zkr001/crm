pub mod config;
pub mod pb;

pub use pb::*;
use sqlx::PgPool;

use std::{pin::Pin, sync::Arc};

use futures::Stream;
use tonic::{Request, Response, Status};
use user_stats::{
    QueryRequest, RawQueryRequest, User, user_stats_service_server::UserStatsService,
};

use crate::{config::AppConfig, pb::user_stats_service_server::UserStatsServiceServer};

pub struct UserStatsServer {
    inner: Arc<UserStatsServerInner>,
}

pub struct UserStatsServerInner {
    config: AppConfig,
    pool: PgPool,
}

type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<User, Status>> + Send>>;

#[tonic::async_trait]
impl UserStatsService for UserStatsServer {
    type QueryStream = ResponseStream;
    type RawQueryStream = ResponseStream;

    async fn query(&self, request: Request<QueryRequest>) -> ServiceResult<Self::QueryStream> {
        todo!()
    }

    async fn raw_query(
        &self,
        request: Request<RawQueryRequest>,
    ) -> ServiceResult<Self::RawQueryStream> {
        todo!()
    }
}

impl UserStatsServer {
    pub async fn new() -> Self {
        let config = AppConfig::load().await.();
        let pool = PgPool::connect(&config.database_url).await.unwrap();
        Self {
            inner: Arc::new(UserStatsServerInner { config, pool }),
        }
    }
}
