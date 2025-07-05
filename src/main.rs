use std::env;
use anyhow::{anyhow, Result};
use base64::prelude::*;
use reqwest::blocking::Client;
use serde_json::json;

fn main() -> Result<()> {
    let title = env::var("ISSUE_TITLE")??;
    let body = env::var("ISSUE_BODY")?;

    let jira_url = format!("https://{}.atlassian.net/", env::var("JIRA_ORG_NAME")?);

    let project_key = env::var("JIRA_PROJECT_KEY")?;

    let email = env::var("JIRA_EMAIL")?;

    let token = env::var("JIRA_API_TOKEN")?;

    let auth = BASE64_STANDARD.encode(format!("{}:{}", email, token));
    println!("{}", auth);
    let client = Client::new();

    // 1. Jira 이슈 생성
    let create_res = client.post(format!("{}/rest/api/3/issue", jira_url))
        .header("Authorization", format!("Basic {}", auth))
        .header("Content-Type", "application/json")
        .json(&json!({
            "fields": {
                "summary": title,
                "description": {
                      "content": [
                        {
                          "content": [
                            {
                              "text": body,
                              "type": "text"
                            }
                          ],
                          "type": "paragraph"
                        }
                      ],
                      "type": "doc",
                      "version": 1
                  },
                "project": { "key": project_key },
                "issuetype": { "name": "Task" }
            }
        }))
        .send()?;

    if !create_res.status().is_success() {
        return Err(anyhow!("Issue creation failed: {:?}", create_res.text()?));
    }

    let issue_key = create_res.json::<serde_json::Value>()?["key"]
        .as_str()
        .ok_or(anyhow!("Missing issue key"))?
        .to_string();

    println!("✅ Jira issue `{}` created", issue_key);
    Ok(())
}
