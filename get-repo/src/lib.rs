use suborbital::runnable::*;
use suborbital::{cache, http, file, req, resp, util};
use serde_json;
use model;

struct GetRepo{}

impl Runnable for GetRepo {
    fn run(&self, _: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        let repo_name = req::state("repo").unwrap_or(req::url_param("repo"));

        let org = match file::get_static("organization") {
            Some(f) => f,
            None => return Err(RunErr::new(500, "application not configured correctly"))
        };

        let mut repo_json: Vec<u8> = Vec::new();

        let _ = cache::get(repo_name.as_str())
            .map_err(|_| {
                let repo_details = http::get(format!("https://api.github.com/repos/{}/{}", util::to_string(org), repo_name).as_str(), None).unwrap_or_default();

                let repo: model::Repo = match serde_json::from_slice(repo_details.as_slice()) {
                    Ok(r) => r,
                    Err(_) => model::Repo::default(),
                };

                repo_json = serde_json::to_vec(&repo).unwrap_or_default();

                cache::set(repo_name.as_str(), repo_json.clone(), 5*60);

                RunErr::new(404, "failed to get repo from cache")
            })
            .map(|json| {
                repo_json = json;
            });

        resp::content_type("application/json");
        Ok(repo_json)
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &GetRepo = &GetRepo{};

#[no_mangle]
pub extern fn init() {
    use_runnable(RUNNABLE);
}
