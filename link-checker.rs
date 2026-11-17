#!/usr/bin/env cargo
---
[dependencies]
reqwest = { version = "0.12", features = ["rustls-tls"] }
tokio = { version = "1", features = ["full"] }
regex = "1"
futures = "0.3"
anyhow = "1"
csv = "1"
---


use futures::{ stream, StreamExt };
use regex::Regex;
use reqwest::{ Client, StatusCode };
use std::{ fs, sync::Arc };
use tokio::sync::Mutex;

const CONCURRENCY: usize = 50;

#[derive(Debug)]
struct LinkResult {
    url: String,
    status: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("src/include/links020/wgsl.md")?;

    let regex = Regex::new(r"https://[^\s]+")?;

    let mut urls: Vec<String> = regex
        .find_iter(&input)
        .map(|m| m.as_str().to_string())
        .collect();

    urls.sort();
    urls.dedup();

    println!("Found {} unique URLs", urls.len());

    let client = Client::builder()
        .user_agent("docsrs-link-checker")
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let results: Arc<Mutex<Vec<LinkResult>>> = Arc::new(Mutex::new(Vec::new()));

    stream::iter(urls).for_each_concurrent(CONCURRENCY, |url| {
        let client = client.clone();
        let results = results.clone();

        async move {
            let result = check_url(&client, &url).await;

            println!("{:<8} {}", result.status, result.url);

            results.lock().await.push(result);
        }
    }).await;

    write_csv(&results.lock().await)?;

    println!("\nReport written to link_report.csv");

    Ok(())
}

async fn check_url(client: &Client, url: &str) -> LinkResult {
    let status = match client.head(url).send().await {
        Ok(resp) => classify_status(resp.status()),
        Err(_) =>
            match client.get(url).send().await {
                Ok(resp) => classify_status(resp.status()),
                Err(e) => format!("ERROR ({})", e),
            }
    };

    LinkResult {
        url: url.to_string(),
        status,
    }
}

fn classify_status(status: StatusCode) -> String {
    match status {
        s if s.is_success() => format!("OK ({})", s.as_u16()),
        s if s.is_redirection() => format!("REDIRECT ({})", s.as_u16()),
        s if s.is_client_error() => format!("CLIENT_ERROR ({})", s.as_u16()),
        s if s.is_server_error() => format!("SERVER_ERROR ({})", s.as_u16()),
        s => format!("OTHER ({})", s.as_u16()),
    }
}

fn write_csv(results: &[LinkResult]) -> anyhow::Result<()> {
    let mut writer = csv::Writer::from_path("link_report.csv")?;

    writer.write_record(["status", "url"])?;

    for r in results {
        writer.write_record([&r.status, &r.url])?;
    }

    writer.flush()?;

    Ok(())
}
