use std::collections::HashMap;

use serde::Serialize;

pub struct AppState {
    pub max_payload: i64,
    pub object_hash_cache: HashMap<String, ObjectContent>,
    pub repo_hash_cache: HashMap<String, Vec<RepoBranchFile>>,
}

#[derive(Serialize, Debug)]
pub struct LastCommit {
    pub hash: String,
    pub date: String,
    pub msg: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Repo {
    pub objects: Vec<RepoBranchFile>,
    pub read_me: Option<String>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RepoBranchFile {
    pub name: String,
    pub file_type: String,
    pub object_hash: String,
    pub last_commit: LastCommit,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RepoData {
    pub name: String,
    pub description: String,
    pub last_commit: LastCommit,
}

#[derive(Serialize, Debug)]
pub struct ObjectContent {
    pub name: String,
    pub content: String,
    pub size: String,
    pub ext: String,
}
