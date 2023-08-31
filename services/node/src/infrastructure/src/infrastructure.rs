use std::sync::Arc;

use sqlx::MySqlPool;

use crate::{
    config::{config::Config, config_factory::ConfigFactory},
    user::user_repository::UserRepository,
    world::world_repository::WorldRepository,
};

pub struct Infrastructure {
    pub config: Arc<Config>,
    pub user_repository: Arc<UserRepository>,
    pub world_repository: Arc<WorldRepository>,
}

impl Infrastructure {
    pub async fn new() -> Self {
        let config = ConfigFactory::build();
        let pool = Self::create_connection_pool_new(&config).await;

        let user_repository = Arc::new(UserRepository::new(pool.clone()));
        let world_repository = Arc::new(WorldRepository::new(pool.clone()));

        Self {
            user_repository,
            world_repository,
            config: Arc::new(config),
        }
    }

    async fn create_connection_pool_new(config: &Config) -> MySqlPool {
        MySqlPool::connect(&config.mysql_connection_url)
            .await
            .unwrap()
    }
}
