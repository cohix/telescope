use reqwest;

pub async fn get_repos() -> Result<String, reqwest::Error> {
	get("/api/v1/repos").await
}

async fn get(url: &str) -> Result<String, reqwest::Error> {
	let full_url = format!("http://localhost:8080{}", url);

	let body: String = match reqwest::get(full_url).await {
		Ok(resp) => resp.text().await.unwrap(),
		Err(e) => return Err(e)
	};

	Ok(body)
}