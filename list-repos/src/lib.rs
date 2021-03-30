use suborbital::runnable::*;
use suborbital::{cache, file, http, log, resp, util};
use serde_json;
use model;

struct ListRepos{}

impl Runnable for ListRepos {
    fn run(&self, _: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        let org = match file::get_static("organization") {
            Some(f) => f,
            None => return Err(RunErr::new(500, "application not configured correctly"))
        };

        let url = format!("https://api.github.com/orgs/{}/repos", util::to_string(org));

        let repo_resp = match http::get(url.as_str(), None) {
            Ok(resp) => resp,
            Err(_) => return Err(RunErr::new(500, "failed to fetch repos"))
        };

        let repos: Vec<model::Repo> = serde_json::from_slice(repo_resp.as_slice()).map_err(|e| {
            log::error(format!("{}", e).as_str());
            RunErr::new(500, "failed to parse JSON")
        })?;

        // cache each repo
        repos.iter().for_each(|r| {
            cache::set(r.name.as_str(), serde_json::to_vec(r).unwrap_or_default(), 5*60);
        });

        let json = serde_json::to_vec(&repos).map_err(|e| {
            log::error(format!("{}", e).as_str());
            RunErr::new(500, format!("{}", e).as_str())
        })?;
        
        resp::content_type("application/json");
        Ok(json)
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &ListRepos = &ListRepos{};

#[no_mangle]
pub extern fn init() {
    use_runnable(RUNNABLE);
}
