use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Repo {
    pub name: String,
	pub stargazers_count: i32,
    pub forks: i32,
    pub open_issues: i32,
}

