use spider::tokio;
use spider::website::Website;
use spider::page::Page;
use scraper::Html;
use std::collections::HashMap;

async fn crawl_web() -> Option<Box<Vec<Page>>>{
    let mut website: Website = Website::new("https://spider.cloud");

    // Start the crawling process
    website.scrape().await;

    // Process the results
    let pages = website.get_pages();
    pages.cloned()
}

async fn parsing_web(pages: Option<Box<Vec<Page>>>) -> Option<String>{
    if let Some(pages) = pages {
        let mut combined_text = String::new();
        for page in pages.iter() {
            let html = page.get_html();

            // Parse the HTML content
            let document = Html::parse_document(&html);

            // Extract all text content from the HTML
            let mut text_content = String::new();
            for text in document.root_element().text() {
                text_content.push_str(text);
                text_content.push(' ');
            }

            // Remove all symbols from the text content
            let cleaned_text: String = text_content
                .chars()
                .filter(|c| c.is_alphabetic() || c.is_whitespace())
                .collect();

                combined_text.push_str(&cleaned_text);
                combined_text.push(' ');
        }
        Some(combined_text)
    } else {
        None
    }
}

async fn analyze_text(text: &str) -> HashMap<String, usize> {
    let mut word_freq = HashMap::new();
    for word in text.split_whitespace() {
        *word_freq.entry(word.to_string()).or_insert(0) += 1;
    }
    word_freq
}

#[tokio::main]
async fn main() {
    let pages = crawl_web().await;
    if let Some(combined_text) = parsing_web(pages).await {
        // Perform basic data analysis on the extracted text content
        let word_freq = analyze_text(&combined_text).await;
        println!("Word Frequency: {:?}", word_freq);
    }
}

