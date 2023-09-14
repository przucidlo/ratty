use std::sync::Arc;

use domain::world::{
    world_member_repository::WorldMemberRepository, world_repository::WorldRepository,
};
use sqlx::MySqlPool;

use crate::{
    config::{config::Config, config_factory::ConfigFactory},
    database::world::mysql_world_member_repository::MySqlWorldMemberRepository,
    user::user_repository::UserRepository,
    world::mysql_world_repository::MySqlWorldRepository,
};

pub struct Infrastructure {
    pub config: Arc<Config>,
    pub user_repository: Arc<UserRepository>,
    pub world_repository: Arc<dyn WorldRepository>,
    pub world_member_repository: Arc<dyn WorldMemberRepository>,
}

impl Infrastructure {
    pub async fn new() -> Self {
        let config = ConfigFactory::build();
        let pool = Self::create_connection_pool_new(&config).await;

        let user_repository = Arc::new(UserRepository::new(pool.clone()));
        let world_repository = Arc::new(MySqlWorldRepository::new(pool.clone()));
        let world_member_repository = Arc::new(MySqlWorldMemberRepository::new(pool.clone()));

        Self {
            user_repository,
            world_repository,
            world_member_repository,
            config: Arc::new(config),
        }
    }

    async fn create_connection_pool_new(config: &Config) -> MySqlPool {
        MySqlPool::connect(&config.mysql_connection_url)
            .await
            .unwrap()
    }
}
