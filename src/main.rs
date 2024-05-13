use reqwest;
use serde_json::json;
use std::io;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let api_key = "api_key"; // replace with your actual API key
    let url = "https://api.groq.com/openai/v1/chat/completions";
    
    let client = reqwest::Client::new();

    println!("Enter your message:");
    let mut message = String::new();
    io::stdin().read_line(&mut message).expect("Failed to read line");
    let message = message.trim();

    let payload = json!({
        "messages": [{"role": "user", "content": message}],
        "model": "llama3-70b-8192"
    });

    let res = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .body(payload.to_string())
        .send()
        .await?;

    let response_text = res.text().await?;

    let response_json: serde_json::Value = serde_json::from_str(&response_text).unwrap();

    let content = response_json["choices"][0]["message"]["content"].as_str().unwrap();
    
    println!();
    println!("{content}");
    println!();

    Ok(())
}
