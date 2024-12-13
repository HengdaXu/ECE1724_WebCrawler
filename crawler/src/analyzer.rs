use std::collections::HashMap;

pub async fn analyze_text(text: &str) -> HashMap<String, usize> {
    let mut word_freq = HashMap::new();
    for word in text.split_whitespace() {
        *word_freq.entry(word.to_string()).or_insert(0) += 1;
    }
    word_freq
}