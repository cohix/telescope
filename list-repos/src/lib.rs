use suborbital::runnable::*;
use suborbital::{file, http, util};

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

        Ok(repo_resp)
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &ListRepos = &ListRepos{};

#[no_mangle]
pub extern fn init() {
    use_runnable(RUNNABLE);
}
