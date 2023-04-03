use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub usage: Usage,
    pub choices: Vec<Choice>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    #[serde(rename = "prompt_tokens")]
    pub prompt_tokens: i64,
    #[serde(rename = "completion_tokens")]
    pub completion_tokens: i64,
    #[serde(rename = "total_tokens")]
    pub total_tokens: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Choice {
    pub message: Message,
    #[serde(rename = "finish_reason")]
    pub finish_reason: String,
    pub index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // This line loads the environment variables from the ".env" file.

    let auth_token = std::env::var("AUTH_TOKEN").expect("export AUTH_TOKEN");
    let bearer_auth = format!("Bearer {}", auth_token);
    println!("Welcome to RustGPT");

    loop {
        println!("====================================");
        println!("Please enter a question or [exit] to stop:");
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        // validate if userInput is equal to the string "exit"
        if line.trim_end() == "exit" {
            println!("Exiting...");
            break;
        }

        println!("You: {}", line);

        let mut data: String = r#"
        {
            "model": "gpt-3.5-turbo",
            "messages": [
              {
                "role": "system",
                "content": "You are a helpful assistant."
              },
              {
                "role": "user",
                "content": "{}"
              }
            ]
          }"#
        .to_string();
        data = format! {"{}", data.replace("{}", line.trim_end())};

        let url = "https://api.openai.com/v1/chat/completions".to_string();
        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header(ACCEPT, "*/*")
            .header(AUTHORIZATION, &bearer_auth)
            .header(CONTENT_TYPE, "application/json")
            .body(data)
            .send()
            .await
            .unwrap();
        match response.status() {
            reqwest::StatusCode::OK => {
                match response.json::<Root>().await {
                    Ok(parsed) => {
                        println!("RustGPT: {}", parsed.choices[0].message.content);
                    }
                    Err(_) => println!("🛑 Hm, the response didn't match the shape we expected."),
                };
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                println!("🛑 Status: UNAUTHORIZED - Need to grab a new token");
            }
            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                println!("🛑 Status: 429 - Too many requests");
            }
            other => {
                panic!("🛑 Uh oh! Something unexpected happened: [{:#?}]", other);
            }
        };
    }
}
