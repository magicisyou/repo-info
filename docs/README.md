# Repo lang stat

A small cli tool to print the statistics of programming languages used in a github repository

## About

This is a small tool which fetches the programming languages used in a repository and print the percentage and number of characters in the terminal. Since no token is used there would be limits in using github api (about 60 request per hour).

## Screenshot

![Example](/docs/screenshot.webp)

## Build

Clone the directory

```
git clone https://github.com/youaremagic/repo-lang-stat
```

Change directory

```
cd repo-lang-stat
```

Run

```
cargo run --release -- <OWNER> <REPO>
```
