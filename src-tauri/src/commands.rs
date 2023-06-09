use core::fmt;
use std::{error::Error, time::Duration, collections::HashMap};
use serde::{Serialize, Deserialize};
use anyhow::Result;

use crate::{database::{DbResponse, RedisDb, DbActions, SkytableClient, SurrealDbClient}, models::{BasicPackageData, PackageData}};

#[derive(Serialize, Deserialize, Debug)]
pub enum Db {
    SurrealDb,
    Redis,
    Skytable
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryCommand {
    target_db: Db,
    query: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FrontendError {
    message: String
}

impl Error for FrontendError {}

impl fmt::Display for FrontendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An unknown error occured")
    }
}

impl From<anyhow::Error> for FrontendError {
    fn from(error: anyhow::Error) -> Self {
        FrontendError { message: error.to_string() }
    }
}

// TODO: fix trait object with async
#[tauri::command]
pub async fn get_query_time(query_command: QueryCommand) -> Result<Duration, FrontendError> {
    let response = match query_command.target_db {
        Db::Redis => RedisDb::try_new()?.get_custom_query_time(&query_command.query).await?,
        Db::Skytable => SkytableClient::try_new()?.get_custom_query_time(&query_command.query).await?,
        Db::SurrealDb => SurrealDbClient::try_new().await?.get_custom_query_time(&query_command.query).await?
    };
    Ok(response)
}

#[tauri::command]
pub async fn run_query(query_command: QueryCommand) -> Result<DbResponse<String>, FrontendError> {
    let response = match query_command.target_db {
        Db::Redis => RedisDb::try_new()?.run_custom_query(&query_command.query).await?,
        Db::Skytable => SkytableClient::try_new()?.run_custom_query(&query_command.query).await?,
        Db::SurrealDb => SurrealDbClient::try_new().await?.run_custom_query(&query_command.query).await?
    };
    Ok(response)
}

#[tauri::command]
pub async fn sort_pkgs_by_field_with_limit(target_db: Db, field: &str, limit_start: u32, limit_end: u32) -> Result<DbResponse<Vec<String>>, FrontendError> {
    let response = match target_db {
        Db::Redis => RedisDb::try_new()?.sort_pkgs_by_field_with_limit(field, limit_start, limit_end).await?,
        Db::Skytable => SkytableClient::try_new()?.sort_pkgs_by_field_with_limit(field, limit_start, limit_end).await?,
        Db::SurrealDb => SurrealDbClient::try_new().await?.sort_pkgs_by_field_with_limit(field, limit_start, limit_end).await?
    };
    Ok(response)
}

#[tauri::command]
pub async fn get_most_voted_pkgs(target_db: Db, number: u32) -> Result<DbResponse<Vec<BasicPackageData>>, FrontendError> {
    let response = match target_db {
        Db::Redis => RedisDb::try_new()?.get_most_voted_pkgs(number).await?,
        Db::Skytable => SkytableClient::try_new()?.get_most_voted_pkgs(number).await?,
        Db::SurrealDb => SurrealDbClient::try_new().await?.get_most_voted_pkgs(number).await?
    };
    Ok(response)
}

#[tauri::command]
pub async fn insert_pkg(target_db: Db, pkg: PackageData) -> Result<DbResponse<()>, FrontendError> {
    let response = match target_db {
        Db::Redis => RedisDb::try_new()?.insert_pkg(&pkg).await?,
        Db::Skytable => SkytableClient::try_new()?.insert_pkg(&pkg).await?,
        Db::SurrealDb => SurrealDbClient::try_new().await?.insert_pkg(&pkg).await?,
    };
    Ok(response)
}

#[tauri::command]
pub async fn get_pkg(target_db: Db, name: &str) -> Result<DbResponse<PackageData>, FrontendError> {
    let response = match target_db {
        Db::Redis => RedisDb::try_new()?.get_pkg(name).await?,
        Db::Skytable => SkytableClient::try_new()?.get_pkg(name).await?,
        Db::SurrealDb => SurrealDbClient::try_new().await?.get_pkg(name).await?,
    };
    Ok(response)
}

#[tauri::command]
pub async fn remove_comments(target_db: Db, pkg_name: &str) -> Result<DbResponse<()>, FrontendError> {
    let response = match target_db {
        Db::Redis => RedisDb::try_new()?.remove_comments(pkg_name).await?,
        Db::Skytable => SkytableClient::try_new()?.remove_comments(pkg_name).await?,
        Db::SurrealDb => SurrealDbClient::try_new().await?.remove_comments(pkg_name).await?,
    };
    Ok(response)
}

#[tauri::command]
pub async fn get_packages_occurences_in_deps(target_db: Db, pkg_names: Vec<String>) -> Result<DbResponse<HashMap<String, u32>>, FrontendError> {
    let response = match target_db {
        Db::Redis => RedisDb::try_new()?.get_packages_occurences_in_deps(&pkg_names).await?,
        Db::Skytable => SkytableClient::try_new()?.get_packages_occurences_in_deps(&pkg_names).await?,
        Db::SurrealDb => SurrealDbClient::try_new().await?.get_packages_occurences_in_deps(&pkg_names).await?,
    };
    Ok(response)
}