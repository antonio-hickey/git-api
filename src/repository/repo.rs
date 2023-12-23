use crate::{
    application::{GitApiError, REPOS_PATH},
    utils::{
        commands::{change_directory, get_filename_from_hash, run_git_command},
        commits::Commit,
        dates::parse_string_to_date,
    },
};
use serde::Serialize;
use std::{
    fs::{self, ReadDir},
    result::Result,
};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A model representing a repository
pub struct Repo {
    pub objects: Vec<RepoBranchFile>,
    pub read_me: Option<String>,
}
impl Repo {
    /// Get all repositories on the server.
    pub async fn get_all() -> Result<Vec<RepoData>, GitApiError> {
        // Start at the file path holding all the repositories
        change_directory(REPOS_PATH)?;

        // Read all the files in the directory and map them into
        // repository data (`RepoData`)
        let repos = fs::read_dir(REPOS_PATH).map(|files_in_dir| {
            let mut repos_in_dir = Self::into_repos_in_dir(files_in_dir);

            // Sort the repositories by date and reverse the order
            // (most recent, ..., oldest)
            repos_in_dir.sort_by_key(|a| parse_string_to_date(&a.last_commit.date));
            repos_in_dir.reverse();
            repos_in_dir
        })?;

        Ok(repos)
    }

    /// Get a repo at a specified state using a given repo name and hash
    ///
    /// Called when a user clicks a directory in a repo branch, so it's meant
    /// to basically treat directories in a repository as sub repositories.
    pub async fn by_hash(repo: &str, hash: &str) -> Result<Repo, GitApiError> {
        // Start by navigating to the repository directory
        let repo_path = format!("{}{}.git/", REPOS_PATH, &repo);
        change_directory(&repo_path)?;

        let parent_path = get_filename_from_hash(hash)?;

        // Grab all the objects in the repo using the "git ls-tree {HASH}" command
        // and trying to parse out the command output into repo objects
        let objects_in_repo = run_git_command(&["ls-tree", hash], false)?
            .lines()
            .filter_map(
                |object| match Self::parse_object(object, Some(&parent_path)) {
                    Ok(object) => Some(object),
                    Err(e) => {
                        eprintln!("{e:?}");
                        None
                    }
                },
            )
            .collect::<Vec<RepoBranchFile>>();

        Ok(Repo {
            objects: objects_in_repo,
            read_me: None,
        })
    }

    /// Get a repository at a specified state using a given repo name and branch.
    ///
    /// Called when a user clicks a repo from the list of repos on /git/ which default
    /// to the master branch for now, but looking to add UI for branch selection soon.
    pub async fn by_branch(repo: &str, branch: &str) -> Result<Repo, GitApiError> {
        // Start by navigating to the repository's directory
        let path = format!("{}{}.git/", REPOS_PATH, &repo);
        change_directory(&path)?;

        // Initiate a mutable variable to store README.md content
        // as a string if the repo has one else default to None.
        let mut read_me: Option<String> = None;

        // Get all the objects in the repository by running
        // the "git ls-tree {BRANCH}" command and parsing
        // through the commands output
        let objects = run_git_command(&["ls-tree", branch], false)?
            .lines()
            .filter_map(|object| {
                if let Ok(object) = Self::parse_object(object, None) {
                    // Check the object name to find a read me
                    if object.name == "README.md" {
                        // Try to read the "README.md" file and mutate
                        // the `read_me` variable to it's content
                        let branch_filename = format!("{}:README.md", &branch);
                        match run_git_command(&["show", &branch_filename], false) {
                            Ok(content) => {
                                read_me = Some(content);
                            }
                            Err(e) => {
                                eprintln!("{e:?}");
                            }
                        }
                    };

                    Some(object)
                } else {
                    None
                }
            })
            .collect::<Vec<RepoBranchFile>>();

        Ok(Repo { objects, read_me })
    }

    /// Try to get the commit log of a specified repo at specified branch
    ///
    /// Called when a user clicks the `[ updates ]` button in a repo tree
    /// the branch is hard coded to master for now, but looking to build
    /// UI for the user to select different branches soon.
    pub async fn get_commit_log(repo: &str, branch: &str) -> Result<Vec<Commit>, GitApiError> {
        // Start by navigating to the repository's directory
        let path = format!("{}{}.git/", REPOS_PATH, &repo);
        change_directory(&path)?;

        // Get all the commit history using the "git log --no-merges {BRANCH}" command
        // and parsing out commits from the output of the command
        let commits: Vec<Commit> = run_git_command(&["log", "--no-merges", branch], false)?
            .lines()
            .collect::<Vec<&str>>()
            .chunks(6)
            .filter_map(|commit_chunk| {
                if commit_chunk.is_empty() {
                    None
                } else {
                    Some(Commit::from_commit_block(commit_chunk.to_vec()))
                }
            })
            .collect();

        Ok(commits)
    }

    /// Converts files in a directory (`ReadDir`) into repositories (`Vec<RepoData>`)
    fn into_repos_in_dir(files_in_dir: ReadDir) -> Vec<RepoData> {
        // filter out any files that got an error trying to read and map the ok ones
        // into repository data `RepoData`
        files_in_dir
            .filter_map(|file_in_dir| {
                // Convert file in directory into repo data
                if let Ok(file_in_dir) = file_in_dir {
                    change_directory(file_in_dir.path().to_str().expect("some path string"))
                        .unwrap();

                    // Only if file in directory is a directory holding other
                    // files itself as a repository will ALWAYS be a directory
                    if file_in_dir
                        .file_type()
                        .map(|file_type| file_type.is_dir())
                        .unwrap_or(false)
                    {
                        // Get the name of the repository from file path
                        // and then replace the REPOS_PATH and ".git"
                        // leaving only the project name
                        let name = file_in_dir
                            .path()
                            .display()
                            .to_string()
                            .replacen(REPOS_PATH, "", 1)
                            .replacen(".git", "", 1);

                        // Get the description of the repository
                        let description =
                            fs::read_to_string(file_in_dir.path().join("description"))
                                .unwrap_or(String::new());

                        // Get the last commit to the repository
                        let commit_log = run_git_command(&["log", "--no-merges"], false).unwrap();
                        let last_commit_block: Vec<&str> = commit_log.lines().take(6).collect();
                        let last_commit = Commit::from_commit_block(last_commit_block);

                        return Some(RepoData {
                            name,
                            description,
                            last_commit,
                        });
                    }
                }
                None
            })
            .collect::<Vec<RepoData>>()
    }

    /// Try to parse a unparsed object string into a `RepoBranchFile`
    fn parse_object(
        unparsed_object: &str,
        parent_path: Option<&str>,
    ) -> Result<RepoBranchFile, GitApiError> {
        // Split up the object line by spaces
        let object_data: Vec<&str> = unparsed_object.split(' ').collect();

        // Parse out a object name, hash, and file type
        let hash_and_name: Vec<&str> = object_data[2].split('\t').collect();
        let object_hash = hash_and_name[0].to_string();
        let name = if let Some(parent_path) = parent_path {
            format!("{}/{}", parent_path, hash_and_name[1])
        } else {
            hash_and_name[1].to_string()
        };
        let file_type = object_data[1].to_string();

        // Parse out the last commit from the commit log of the object
        let commit_log = run_git_command(&["log", "--no-merges", "--", &name], false)?;
        let last_commit_block: Vec<&str> = commit_log.lines().take(6).collect();
        let last_commit = Commit::from_commit_block(last_commit_block);

        Ok(RepoBranchFile {
            name: hash_and_name[1].to_string(),
            object_hash,
            file_type,
            last_commit,
        })
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A model representing "metadata" for a repo
pub struct RepoData {
    pub name: String,
    pub description: String,
    pub last_commit: Commit,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A model representing an object in a repo
/// examples: code file, folder, image file, etc.
pub struct RepoBranchFile {
    pub name: String,
    pub file_type: String,
    pub object_hash: String,
    pub last_commit: Commit,
}
