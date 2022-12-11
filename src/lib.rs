use crate::schema::sample_user::dsl;
use diesel::prelude::*;

mod models;
mod pool;
mod schema;

pub use models::SampleUser;
pub use pool::MultipleConnectionPool;
pub use schema::sample_user;

/// Sample logic for using database
///
/// 1. Get the aggregate from datastore
/// 2. Print its
pub async fn sample_logic<T>(ctx: &T, tenant_id: i32) -> eyre::Result<()>
where
    T: ProvideSampleUserRepository,
{
    let users = ctx.provide().get(tenant_id).await;
    println!("tenant_id: {tenant_id}");
    println!("{users:#?}");
    Ok(())
}

/// Interface of SampleUser aggregate
#[async_trait::async_trait]
pub trait SampleUserRepository {
    /// Get the SampleUser aggregates
    async fn get(&self, tenant_id: i32) -> eyre::Result<Vec<models::SampleUser>>;
}

#[async_trait::async_trait]
impl SampleUserRepository for MultipleConnectionPool {
    async fn get(&self, tenant_id: i32) -> eyre::Result<Vec<models::SampleUser>> {
        let conn = self.conn(tenant_id).await?;
        let r = conn
            .interact(|conn| dsl::sample_user.load::<SampleUser>(conn))
            .await??;
        Ok(r)
    }
}

/// Provider of SampleUserRepository
pub trait ProvideSampleUserRepository {
    type Repository: SampleUserRepository;

    fn provide(&self) -> &Self::Repository;
}

impl ProvideSampleUserRepository for MultipleConnectionPool {
    type Repository = MultipleConnectionPool;

    fn provide(&self) -> &Self::Repository {
        self
    }
}
