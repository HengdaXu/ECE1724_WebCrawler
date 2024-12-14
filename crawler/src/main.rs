mod crawler;
mod parser;
mod analyzer;
mod ui;

use crate::ui::init_ui;

#[tokio::main]
async fn main() {
    init_ui().await;
}
