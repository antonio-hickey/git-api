use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct LastCommit {
    pub hash: String,
    pub date: String,
    pub msg: String,
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
