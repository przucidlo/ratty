use std::sync::Arc;

use sea_orm::{Database, DatabaseConnection};

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
        let db = Self::create_connection_pool(&config).await;

        let user_repository = Arc::new(UserRepository::new(db.clone()));
        let world_repository = Arc::new(WorldRepository::new(db.clone()));

        Self {
            user_repository,
            world_repository,
            config: Arc::new(config),
        }
    }

    async fn create_connection_pool(config: &Config) -> DatabaseConnection {
        println!("Connecting to database...");

        match Database::connect(&config.mysql_connection_url).await {
            Ok(db) => {
                println!("Connected to database.");

                db
            }
            Err(_) => panic!("Could not connect to database."),
        }
    }
}
