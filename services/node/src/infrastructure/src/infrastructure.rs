use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::sync::Arc;

use crate::{
    config::{config::Config, config_factory::ConfigFactory},
    user::user_repository::UserRepository,
};

pub struct Infrastructure {
    pub config: Arc<Config>,
    pub user_repository: Arc<UserRepository>,
}

impl Infrastructure {
    pub async fn new() -> Self {
        let config = ConfigFactory::build();
        let pool = Self::create_connection_pool(&config).await;

        let user_repository = Arc::new(UserRepository::new(pool.clone()));

        Self {
            user_repository,
            config: Arc::new(config),
        }
    }

    async fn create_connection_pool(config: &Config) -> MySqlPool {
        println!("Connecting to database...");

        let pool = MySqlPoolOptions::new()
            .connect(&config.mysql_connection_url)
            .await
            .unwrap();

        println!("Connected to database.");

        pool
    }
}
