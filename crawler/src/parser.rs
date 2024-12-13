use spider::page::Page;
use scraper::Html;

pub async fn parsing_web(pages: Option<Box<Vec<Page>>>) -> Option<String>{
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