use std::collections::HashMap;
use reqwest::get;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let url = "https://en.wikipedia.org/wiki/Jersey_Act";
    let body = get(url).await?.text().await?;
    // println!("body = {body:?}");

    let document = Html::parse_document(&body);

    let anchor_selector = Selector::parse("a").expect("No anchor tags found");

    let elements: Vec<_> = document.select(&anchor_selector).collect();

    if elements.is_empty() {
        return Err("No linkes found".into());
    }
    
    let mut links: HashMap<String, String> = HashMap::new();

    for element in elements{
        if let Some(href) = element.value().attr("href"){
            if let Some(text) = element.text().next(){
                links.insert(text.trim().to_string(), href.to_string());
            }
        }
    }

    for (text, href) in &links{
        println!("Text -> {}, Href -> {}", text, href);
    }

    Ok(())
}

