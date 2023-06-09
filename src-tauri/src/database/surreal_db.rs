use std::time::{Instant, Duration};


use crate::models::PackageData;

use super::{DbActions, DbResponse};
use anyhow::Result;
use async_trait::async_trait;
use surrealdb::{Surreal, engine::remote::ws::{Ws, Client}, opt::auth::Root, Response};

pub struct SurrealDbClient {
    db: Surreal<Client>,
}

impl SurrealDbClient {
    pub async fn try_new() -> Result<Self> {
        let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await?;

        db.use_ns("aur").use_db("packages").await?;

        Ok(Self { db })
    }
}

#[async_trait]
impl DbActions for SurrealDbClient {
    async fn get_custom_query_time(&self, query: &str) -> Result<Duration> {
        let start = Instant::now();
        self.db.query(query).await?;
        Ok(start.elapsed())
    }

    async fn run_custom_query(&self, query: &str) -> Result<DbResponse<String>> {
        let start = Instant::now();
        let mut response: Response = self.db.query(query).await?;
        let duration = start.elapsed();

        let result: Option<PackageData> = response.take(0)?;
        if let Some(data) = result {
            let result = serde_json::to_string(&data)?;
            return Ok(DbResponse { result, duration });
        }
        Ok(DbResponse { result: "No data found".to_owned(), duration })
    }

    async fn sort_pkgs_by_field_with_limit(&self, field: &str, limit_start: u32, limit_end: u32) -> Result<DbResponse<Vec<String>>> {
        let query = 
            format!("SELECT VALUE name FROM (SELECT basic.name as name, basic.{} as key 
                FROM pkgs ORDER BY key DESC LIMIT BY {} START AT {})",
                field,
                limit_end.to_string(),
                limit_start.to_string()
            );

        let start = Instant::now();
        let result: Vec<String> = self.db.query(query).await?.take(0)?;
        let duration = start.elapsed();
        Ok(DbResponse { result, duration })
    }
}

#[cfg(test)]
mod test {
    use super::SurrealDbClient;
    use anyhow::{Result, Ok};
    use super::DbActions;

    #[tokio::test]
    async fn ss() -> Result<()> {
        let db = SurrealDbClient::try_new().await?;
        db.run_custom_query("SELECT VALUE basic.name as name FROM pkgs LIMIT BY 1").await?;
        Ok(())
    }
}