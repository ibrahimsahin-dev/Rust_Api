use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ApiResponse {
    hdurl: String,
    
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    
    let api_key="DEMO_KEY".to_string();
    let url = format!(
        "https://api.nasa.gov/planetary/apod?api_key={}",
        api_key
    );

    let response: ApiResponse = reqwest::get(url)
        .await?
        .json()
        .await?;

    println!("ID: {}", response.hdurl);
   

    Ok(())
}
