use clap::Parser;
use colored::Colorize;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};
use std::collections::HashMap;
use term_size::dimensions;

/// Get Info about languages used in a github repository
#[derive(Parser)]
#[command(version)]
struct Details {
    /// Repository in format owner/repository
    repository: String,
}

impl Details {
    fn fetch(&self) -> Result<HashMap<String, u64>, Box<dyn std::error::Error>> {
        let url = format!("https://api.github.com/repos/{}/languages", self.repository);

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
            let bar = get_bar(sum, *count);
            println!(
                "{:15}  {:6}  {}",
                lang.blue(),
                percentage(sum, *count).green(),
                bar.green(),
            );
        }
    }
}

fn percentage(sum: u64, count: u64) -> String {
    let perc = (count as f64 / sum as f64) * 100.0;
    format!("{:.2}%", perc)
}

fn get_bar(sum: u64, count: u64) -> String {
    let text_width = 30;
    let fallback_width = 60;
    let term_width = match dimensions() {
        Some((width, _)) => width as u64 - text_width,
        None => fallback_width - text_width,
    };
    let width = (count * term_width / sum) as usize;
    let mut bar = String::from("");
    for _i in 0..width {
        bar.push('■');
    }
    bar
}
