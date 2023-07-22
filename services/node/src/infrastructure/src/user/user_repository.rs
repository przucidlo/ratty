use domain::user::user::User as DomainUser;
use sea_query::{enum_def, Asterisk, Expr, MysqlQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::MySqlPool;

use super::user_errors::UserInfrastructureError;

#[enum_def]
#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn to_domain(&self) -> DomainUser {
        DomainUser::new_existing(&self.id, &self.username, &self.password)
    }
}

pub struct UserRepository {
    pool: MySqlPool,
}

impl UserRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_username(
        &self,
        username: &str,
    ) -> Result<DomainUser, UserInfrastructureError> {
        let (query, values) = Query::select()
            .column(Asterisk)
            .from(UserIden::Table)
            .and_where(Expr::col(UserIden::Username).eq(username))
            .build_sqlx(MysqlQueryBuilder);

        let user = sqlx::query_as_with::<_, User, _>(&query, values.clone())
            .fetch_one(&self.pool)
            .await;

        match user {
            Ok(user) => Ok(user.to_domain()),
            Err(_) => Err(UserInfrastructureError::UserNotFoundError),
        }
    }

    pub async fn find_by_id(&self, id: u64) -> Result<DomainUser, UserInfrastructureError> {
        let (query, values) = Query::select()
            .column(Asterisk)
            .from(UserIden::Table)
            .and_where(Expr::col(UserIden::Id).eq(id))
            .build_sqlx(MysqlQueryBuilder);

        let user = sqlx::query_as_with::<_, User, _>(&query, values.clone())
            .fetch_one(&self.pool)
            .await;

        match user {
            Ok(user) => Ok(user.to_domain()),
            Err(_) => Err(UserInfrastructureError::UserNotFoundError),
        }
    }

    pub async fn insert(&self, user: DomainUser) -> Result<DomainUser, UserInfrastructureError> {
        let (sql, values) = Query::insert()
            .into_table(UserIden::Table)
            .columns([UserIden::Username, UserIden::Password])
            .values_panic([user.username().into(), user.password().into()])
            .build_sqlx(MysqlQueryBuilder);

        let result = sqlx::query_with(&sql, values).execute(&self.pool).await;

        match result {
            Ok(result) => {
                let last_insert_id = result.last_insert_id();

                return self.find_by_id(last_insert_id).await;
            }
            Err(_) => todo!(),
        }
    }
}
