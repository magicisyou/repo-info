use clap::Parser;
use colored::Colorize;
use indicatif::ProgressBar;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};
use std::{collections::HashMap, fmt};
use term_size::dimensions;

#[derive(serde::Deserialize)]
struct RepositoryInfo {
    name: String,
    full_name: String,
    description: Option<String>,
    language: Option<String>,
    stargazers_count: u32,
    size: u32,
    forks: u32,
    open_issues: u32,
    default_branch: String,
    updated_at: String,
    html_url: String,
    #[serde(flatten)]
    _ignore: serde_json::Value,
}

struct Repository {
    info: RepositoryInfo,
    languages: HashMap<String, u64>,
}

impl Repository {
    fn from(info: RepositoryInfo, languages: HashMap<String, u64>) -> Self {
        Self { info, languages }
    }
}

/// Get Info about languages used in a github repository
#[derive(Parser)]
#[command(version)]
struct Details {
    /// Repository in format owner/repository
    repository: String,
}

impl Details {
    fn fetch_repository_details(&self) -> Result<Repository, Box<dyn std::error::Error>> {
        let progress_bar = ProgressBar::new(2);
        progress_bar.inc(0);

        let info_url = format!("https://api.github.com/repos/{}", self.repository);
        let lang_url = format!("https://api.github.com/repos/{}/languages", self.repository);

        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/vnd.github+json"),
        );
        headers.insert(USER_AGENT, HeaderValue::from_static("silentdusk_repo_info"));

        let client = reqwest::blocking::Client::new();

        let repository_info = client
            .get(info_url)
            .headers(headers.clone())
            .send()?
            .json::<RepositoryInfo>()?;

        progress_bar.inc(1);

        let repository_languages = client
            .get(lang_url)
            .headers(headers)
            .send()?
            .json::<HashMap<String, u64>>()?;

        progress_bar.inc(1);
        progress_bar.finish_and_clear();

        Ok(Repository::from(repository_info, repository_languages))
    }
}

impl fmt::Display for RepositoryInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let term_width = match dimensions() {
            Some((width, _)) => width,
            None => 65,
        };
        let language = match &self.language {
            Some(lang) => lang.into(),
            None => "".to_string(),
        };
        let description = match &self.description {
            Some(desc) => desc.into(),
            None => "".to_string(),
        };
        writeln!(f, "{:term_width$}", self.name.red().on_white())?;
        writeln!(f, "Full Name: {}", self.full_name.red())?;
        writeln!(f, "Description: {}", description.red())?;
        writeln!(f, "Language: {}", language.red())?;
        writeln!(f, "Default branch: {}", self.default_branch.red())?;
        writeln!(f, "Stars: {}", self.stargazers_count.to_string().red())?;
        writeln!(f, "Size: {}", self.size.to_string().red())?;
        writeln!(f, "Forks: {}", self.forks.to_string().red())?;
        writeln!(f, "Open issues: {}", self.open_issues.to_string().red())?;
        writeln!(f, "Last updated: {}", self.updated_at.red())?;
        write!(f, "Link: {}", self.html_url.red())?;
        Ok(())
    }
}

fn main() {
    let details = Details::parse();

    match details.fetch_repository_details() {
        Ok(response) => {
            println!("{}", &response.info);
            print_stat(&response.languages);
        }
        Err(e) => eprintln!("Sorry, Can't find what you are looking for.\nError : {e}"),
    }
}

fn print_stat(data: &HashMap<String, u64>) {
    if data.is_empty() {
        println!("No languages found");
    } else {
        println!("Languages:");
        let mut sum = 0;
        for (_, count) in data.iter() {
            sum += count;
        }
        for (lang, count) in data.iter() {
            let bar = get_bar(sum, *count);
            println!(
                "{:15}  {:6}  {}",
                lang,
                percentage(sum, *count).red(),
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
