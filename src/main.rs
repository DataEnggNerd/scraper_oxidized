use std::collections::HashMap;
use reqwest::get;
use scraper::{Selector, Html};

#[tokio::main]
async fn main(){
    let url = "https://en.wikipedia.org/wiki/Jersey_Act";
    let body = get(url).await.unwrap().text().await.unwrap();
    println!("body = {body:?}");

    let document = Html::parse_document(&body);

    let anchor_selector = Selector::parse("a").unwrap();

    let mut links: HashMap<String, String> = HashMap::new();

    for element in document.select(&anchor_selector){
        if let Some(href) = element.value().attr("href"){
            if let Some(text) = element.text().next(){
                links.insert(text.trim().to_string(), href.to_string());
            }
        }
    }

    for (text, href) in &links{
        println!("Text -> {}, Href -> {}", text, href);
    }
}

