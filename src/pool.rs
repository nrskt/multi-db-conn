use std::collections::HashMap;

use deadpool_diesel::postgres::{Manager, Object, Pool, Runtime};
use diesel::PgConnection;
use maplit::hashmap;

/// PostgreSQL Database connection
///
/// It wrap the deadpool object because solving error handling issues.
/// `deadpool_sync::InteractError` does not implement Sync trait. but Error type of eyre
/// require Sync trait. Therefore, it has `interact()` method.
pub struct PgConn(Object);

impl PgConn {
    pub async fn interact<F, R>(&self, f: F) -> eyre::Result<R>
    where
        F: FnOnce(&mut PgConnection) -> R + Send + 'static,
        R: Send + 'static,
    {
        match self.0.interact(f).await {
            Err(e) => eyre::bail!("{:?}", e),
            Ok(r) => Ok(r),
        }
    }
}

impl std::ops::Deref for PgConn {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for PgConn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Connection pool for connectiong multiple database instances.
#[derive(Clone)]
pub struct MultipleConnectionPool {
    pool: HashMap<i32, Pool>,
}

impl MultipleConnectionPool {
    /// Construct the multiple database connection pool
    ///
    /// If there are many tenants, we should fix to the arguments
    pub fn new(tenant_1: &str, tenant_2: &str) -> eyre::Result<Self> {
        let tenant_1_mgr = Manager::new(tenant_1, Runtime::Tokio1);
        let tenant_2_mgr = Manager::new(tenant_2, Runtime::Tokio1);
        let tenant_1_pool = Pool::builder(tenant_1_mgr).max_size(2).build()?;
        let tenant_2_pool = Pool::builder(tenant_2_mgr).max_size(2).build()?;

        Ok(Self {
            pool: hashmap! {
            1 => tenant_1_pool,
            2 => tenant_2_pool,
                        },
        })
    }

    /// Get the database conncetion for the specific tenant according to the argument.
    pub async fn conn(&self, tenant_id: i32) -> eyre::Result<PgConn> {
        let pool = match self.pool.get(&tenant_id) {
            Some(val) => val,
            None => eyre::bail!("failed"),
        };
        let conn = pool.get().await?;
        Ok(PgConn(conn))
    }
}
