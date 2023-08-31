use std::collections::HashMap;
use std::collections::HashSet;

use uuid::Uuid;
use serde::{Serialize, Deserialize};


pub struct AppState {
    pub users: Users,
    pub max_payload: i64,
    pub object_hash_cache: HashMap<String, ObjectContent>,
    pub repo_hash_cache: HashMap<String, Vec<RepoBranchFile>>,
}

#[derive(PartialEq, Eq, Hash)]
pub enum Privileges {
    AntonioHickey,
    SwWorks,
    Omni,
    Flux,
    Geode,
    Pleb,
}

pub struct User {
    pub id: Uuid,
    privileges: HashSet<Privileges>,
}
impl Default for User {
    fn default() -> User {
        return User {
            id: Uuid::new_v4(), 
            privileges: HashSet::from([Privileges::Pleb])
        }
    }
}
impl User {
    fn add_privilege(&mut self, privilege: Privileges) -> bool {
        self.privileges.insert(privilege)
    }
    fn remove_privilege(&mut self, privilege: Privileges) -> bool {
        self.privileges.remove(&privilege)
    }
    fn make_pleb(&mut self) -> () {
        self.privileges = HashSet::from([Privileges::Pleb])
    }
}

pub struct Users {
    pub state_hash: String,
    pub hashmap: HashMap::<String, User>,
}
impl Default for Users {
    fn default() -> Users {
        /* TODO:
            - Local file to replicate
            - ? Use signature for integrity check on local file ?
            - Default to reading local file and replicating it
        */ 
        let antonio_hickey = User {
            id: Uuid::new_v4(),
            privileges: HashSet::from([Privileges::AntonioHickey])
        };

        Users {
            state_hash: Uuid::new_v4().as_simple().to_string(),
            hashmap: HashMap::<String, User>::from([
                ("67e5504410b1426f9247bb680e5fe0c8".to_string(), antonio_hickey),
            ])
        }
    }
}
impl Users {}

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

#[derive(Serialize, Debug)]
pub struct ObjectContent {
    pub name: String,
    pub content: String,
    pub size: String,
    pub ext: String,
}
