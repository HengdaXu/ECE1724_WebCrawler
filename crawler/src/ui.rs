use crate::crawler::crawl_web;
use crate::parser::parsing_web;
use crate::analyzer::analyze_text;


use std::collections::HashMap;
use cursive::Cursive;
use cursive::views::{Button, Dialog, EditView, LinearLayout, TextView};
use cursive::traits::*;
use cursive::style::{BorderStyle, Palette};
use cursive::view::Nameable;
use url::{Url, ParseError};

pub async fn init_ui() {
    let mut siv = cursive::default();

    // Start with a nicer theme than default
    siv.set_theme(cursive::theme::Theme {
        shadow: true,
        borders: BorderStyle::Simple,
        palette: Palette::retro().with(|palette| {
            use cursive::style::BaseColor::*;

            {
                // First, override some colors from the base palette.
                use cursive::style::Color::TerminalDefault;
                use cursive::style::PaletteColor::*;

                palette[Background] = TerminalDefault;
                palette[View] = TerminalDefault;
                palette[Primary] = White.dark();
                palette[TitlePrimary] = Blue.light();
                palette[Secondary] = Blue.light();
                palette[Highlight] = Blue.dark();
            }

            {
                // Then override some styles.
                use cursive::style::Effect::*;
                use cursive::style::PaletteStyle::*;
                use cursive::style::Style;
                palette[Highlight] = Style::from(Blue.light()).combine(Bold);
                palette[EditableTextCursor] = Style::secondary().combine(Reverse).combine(Underline)
            }
        }),
    });
    
    let url_input = EditView::new()
        .with_name("url_input")
        .fixed_width(40);

    let layout = LinearLayout::vertical()
        .child(TextView::new("Enter a URL to crawl"))
        .child(url_input)
        .child(Button::new("Start Crawling", move |s| {
            // Get the URL from user input
            let crawl_url = s
                .call_on_name("url_input", |view: &mut EditView| view.get_content())
                .unwrap();
            
            let crawl_url = crawl_url.clone();
            let cb_sink = s.cb_sink().clone();
            tokio::spawn(async move {
                let pages = crawl_web(crawl_url.to_string()).await;
                let combined_text = parsing_web(pages).await;
                let word_freq: HashMap<String, usize> = analyze_text(&combined_text.clone().unwrap()).await;
                cb_sink.send(Box::new(move |s| {
                    if let Some(combined_text) = combined_text {
                        s.add_layer(
                            Dialog::around(TextView::new(combined_text)).title("Crawled Text")
                                .title("Entered URL")
                                .button("Back", |s| {
                                    s.pop_layer();
                                })
                                .button("Frequency Analysis", move |s|{
                                    let results = word_freq.clone();
                                    word_freq_results(s, results);
                                }),
                        );
                    } else {
                        error_message(s, "Error: Failed to crawl or parse the pages.");
                    }
                })).unwrap();
            });

        }))
        .child(Button::new("Check", check_url))
        .child(Button::new("Quit", |s| {
            s.quit();
        }));

    siv.add_layer(
        Dialog::around(layout)
            .title("Web Crawler UI")
    );

    siv.run();
}

fn word_freq_results(s: &mut Cursive, word_freq: HashMap<String, usize>) {
    let mut result_layout = LinearLayout::vertical();
    for (word, count) in &word_freq {
        result_layout.add_child(TextView::new(format!("{}: {}", word, count)));
    }

    
    // Add a new layer with the result layout
    s.add_layer(
        Dialog::around(result_layout)
            .title("Word Frequency Results")
            .button("Top 5", move |s|{
                let mut results = HashMap::new();
                for (word, count) in &word_freq {
                    results.insert(word.clone(), *count);
                }
                top_results(s, results);
            }) 
            .button("Close", |s| {
                s.pop_layer();
            }),
    );
}

fn top_results(s: &mut Cursive, word_freq: HashMap<String, usize>) {
    let mut sorted: Vec<_> = word_freq.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1)); // Sort by frequency (descending)
    let top_five: Vec<_> = sorted.into_iter().take(5).collect();

    let mut top_layout = LinearLayout::vertical();
    for (word, count) in top_five {
        top_layout.add_child(TextView::new(format!("{}: {}", word, count)));
    }

    s.add_layer(
        Dialog::around(top_layout)
            .title("Top 5 Word Frequencies")
            .button("Close", |s| {
                s.pop_layer();
            }),
    );
}

fn check_url(s: &mut Cursive) {
    // Get the URL from user input
    let crawl_url = s
        .call_on_name("url_input", |view: &mut EditView| view.get_content())
        .unwrap();

    // Create the new layer to check the url
    // check URL
    match Url::parse(&crawl_url) {
        Ok(_) => {
            s.add_layer(
                Dialog::around(TextView::new(format!("You entered: {}", crawl_url)))
                    .title("Entered URL")
                    .button("OK", |s| {
                        s.pop_layer();
                    }),
            );
        }
        Err(ParseError::InvalidPort) => {
            error_message(s, "Error: The URL contains an invalid port number.");
        }
        Err(ParseError::RelativeUrlWithoutBase) => {
            error_message(s, "Error: The URL does not have a valid base protocol.");
        }
        Err(ParseError::InvalidIpv4Address) => {
            error_message(s, "Error: The URL contains an invalid IPv4 address.");
        }
        Err(ParseError::InvalidIpv6Address) => {
            error_message(s, "Error: The URL contains an invalid IPv6 address.");
        }
        Err(err) => {
            error_message(s, &format!("Error parsing URL: {}", err));
        }
    }
}

fn error_message(s: &mut Cursive, message: &str) {
    s.add_layer(
        Dialog::around(TextView::new(message))
            .title("Invalid URL")
            .button("OK", |s| {
                s.pop_layer();
            }),
    );
}

