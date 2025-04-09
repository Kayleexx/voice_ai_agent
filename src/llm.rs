use std::env;
use reqwest::Client;
use serde_json::json;

pub async fn ask_together_ai(prompt: &str) -> anyhow::Result<String> {
    let api_key = env::var("TOGETHER_AI_KEY")?;
    let client = Client::new();

    let res = client
        .post("https://api.together.xyz/inference")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": "mistral-7b-instruct",
            "prompt": prompt,
            "temperature": 0.8,
            "max_tokens": 100
        }))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(res["output"].as_str().unwrap_or("No clue fam 🤷‍♂️").to_string())
}
