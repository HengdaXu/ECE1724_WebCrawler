use spider::website::Website;
use spider::page::Page;

pub async fn crawl_web(url: String) -> Option<Box<Vec<Page>>>{
    let mut website: Website = Website::new(&url);

    // Start the crawling process
    website.scrape().await;

    // Process the results
    let pages = website.get_pages();
    pages.cloned()
}




