use suborbital::runnable::*;
use suborbital::{http, req, file, log, util};
use std::collections::BTreeMap;
use serde::{Serialize};
use serde_json;

struct SendReport{}

#[derive(Serialize)]
struct WebhookContents {
    content: String
}

impl Runnable for SendReport {
    fn run(&self, _: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        let repo_json = req::state("__elem").unwrap_or_default();
        let repo: model::Repo = serde_json::from_slice(util::to_vec(repo_json.clone()).as_slice()).map_err(|_| {
            RunErr::new(500, "failed to parse JSON")
        })?;

        if repo.stargazers_count < 5 {
            return Ok(util::to_vec(repo_json))
        }

        let url = file::get_static("./webhook").unwrap_or_default();
        let url_str = util::to_string(url);

        let content = WebhookContents{
            content: format!("{}: {} stargazers", repo.name, repo.stargazers_count)
        };

        let mut headers = BTreeMap::new();
        headers.insert("Content-Type", "application/json");

        let body = serde_json::to_vec(&content).unwrap_or_default();

        http::post(url_str.as_str(), Some(body), Some(headers)).map_err(|e| {
            log::error(format!("{}", e.message).as_str());
            e
        })?;

        Ok(util::to_vec(repo_json))
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &SendReport = &SendReport{};

#[no_mangle]
pub extern fn init() {
    use_runnable(RUNNABLE);
}
