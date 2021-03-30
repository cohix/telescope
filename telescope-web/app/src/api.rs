use anyhow::{anyhow, Error};
// use reqwest;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::prelude::*;
use model;

pub fn get_repos(callback: Callback<Result<Vec<model::Repo>, Error>>) -> FetchTask {
	let handler = move |response: Response<Json<Result<Vec<model::Repo>, Error>>>| {
		let (meta, Json(data)) = response.into_parts();
		if meta.status.is_success() {
			callback.emit(data)
		} else {
			callback.emit(Err(anyhow!(
				"{}: error getting repos",
				meta.status
			)))
		}
	};

	let request = Request::get(url("/api/v1/repos").as_str()).body(Nothing).unwrap();
	FetchService::fetch(request, handler.into()).unwrap()
}

fn url(path: &str) -> String {
	format!("http://localhost:8080{}", path)
}