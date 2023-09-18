use clap::Parser;
use colored::Colorize;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};
use std::collections::HashMap;

/// Get Info about languages used in a github repository
#[derive(Parser)]
#[command(version)]
struct Details {
    /// Owner of the repository
    owner: String,
    /// Name of the repository
    repo: String,
}

impl Details {
    fn fetch(&self) -> Result<HashMap<String, u64>, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/languages",
            self.owner, self.repo
        );

        println!("Fetching info!");

        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/vnd.github+json"),
        );
        headers.insert(USER_AGENT, HeaderValue::from_static("repo_lang_stat"));

        let client = reqwest::blocking::Client::new();
        let response = client
            .get(url)
            .headers(headers)
            .send()?
            .json::<HashMap<String, u64>>()?;

        Ok(response)
    }
}

fn main() {
    let details = Details::parse();
    match details.fetch() {
        Ok(response) => print_stat(&response),
        Err(e) => eprintln!("Sorry, Can't find what you are looking for.\nError : {e}"),
    }
}

fn print_stat(data: &HashMap<String, u64>) {
    if data.is_empty() {
        println!("No languages found");
    } else {
        let mut sum = 0;
        for (_, count) in data.iter() {
            sum += count;
        }
        println!();
        for (lang, count) in data.iter() {
            println!(
                "{:12} : {:6} : {count} chars",
                lang.blue(),
                percentage(sum, *count).green()
            );
        }
    }
}

fn percentage(sum: u64, count: u64) -> String {
    let perc = (count as f64 / sum as f64) * 100.0;
    format!("{:.2}%", perc)
}
