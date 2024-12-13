mod crawler;
mod parser;
mod analyzer;

use crate::crawler::crawl_web;
use crate::parser::parsing_web;
use crate::analyzer::analyze_text;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    println!("Starting the web crawler...");

    // Step 1: Crawl the web to get pages
    let pages = crawl_web().await;
    if pages.is_none() {
        eprintln!("No pages were crawled. Exiting...");
        return;
    }

    // Step 2: Parse the crawled pages to extract text content
    let combined_text = parsing_web(pages).await;
    if combined_text.is_none() {
        eprintln!("Failed to parse the crawled pages. Exiting...");
        return;
    }

    // Step 3: Analyze the parsed text content
    let word_freq: HashMap<String, usize> = analyze_text(&combined_text.unwrap()).await;

    // Step 4: Display results
    println!("Web Crawler Completed Successfully!");
    println!("Word Frequency Analysis: {:?}", word_freq);
}
