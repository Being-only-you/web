#[cfg(feature = "ssr")]
use std::sync::Arc;
#[cfg(feature = "ssr")]
use crate::error::AppError;
#[cfg(feature = "ssr")]
use surrealdb::engine::remote::ws::{Client, Ws};
#[cfg(feature = "ssr")]
use surrealdb::opt::auth::Root;
#[cfg(feature = "ssr")]
use surrealdb::sql::Value;
#[cfg(feature = "ssr")]
use surrealdb::Surreal;

#[cfg(feature = "ssr")]
pub struct DB;

#[cfg(feature = "ssr")]
pub struct ConnectionOptions<'a> {
    pub namespace: &'a str,
    pub database: &'a str,
    pub credentials: Root<'a>,
}

#[cfg(feature = "ssr")]
impl DB {
    pub async fn connect<'a>(
        endpoint: &'static str,
        options: &ConnectionOptions<'a>,
    ) -> Result<Surreal<Client>, AppError> {
        let db = Surreal::new::<Ws>(endpoint).await?;

        db.signin(options.credentials).await?;

        db.use_ns(options.namespace)
            .use_db(options.database)
            .await?;

        Ok(db)
    }
}

#[derive(Clone)]
#[cfg(feature = "ssr")]
pub struct SurrealDBRepo {
    pub db: Arc<Surreal<Client>>,
}

#[cfg(feature = "ssr")]
impl SurrealDBRepo {
    pub async fn new(endpoint: &'static str, options: &ConnectionOptions<'_>) -> Self {
        let db = DB::connect(endpoint, options).await.unwrap();
        SurrealDBRepo {
            db: Arc::new(db),
        }
    }
}

#[cfg(feature = "ssr")]
pub trait Creatable: Into<Value> {}

#[cfg(feature = "ssr")]
pub trait Updatable: Into<Value> {}
