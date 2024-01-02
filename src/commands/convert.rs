use std::process::exit;

use lazy_static::lazy_static;
use regex::Regex;
use scraper::{Html, Selector};

use crate::{currency_code::CurrencyCode, errors::ApplicationError};

lazy_static! {
    static ref PRICE_REGEX: Regex = Regex::new(r"\d+[,.]\d+").unwrap();
    static ref PRICE_SELECTOR: Selector =
        Selector::parse("#main > div > div > div + div + div").unwrap();
}

async fn make_request(amount: f64, from: &str, to: &str) -> Result<String, ApplicationError> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://www.google.com/search?q={}-{}+to+{}",
        amount, from, to
    );
    let res = client.get(&url).send().await?;
    let html = res.text().await?;

    Ok(html)
}

fn parse_request(html: &str) -> Result<f64, ApplicationError> {
    let fragment = Html::parse_document(html);
    let price = fragment
        .select(&PRICE_SELECTOR)
        .next()
        .ok_or(ApplicationError::ParseError(
            "no price element found in html".to_string(),
        ))?;
    let price = price
        .text()
        .find(|s| PRICE_REGEX.is_match(s))
        .ok_or(ApplicationError::ParseError(
            "no price found in element content".to_string(),
        ))?
        .trim()
        .replace(',', ".")
        .split(' ')
        .next()
        .ok_or(ApplicationError::ParseError(
            "price format unexpected".to_string(),
        ))?
        .to_string();
    let price = price
        .parse::<f64>()
        .map_err(|_| ApplicationError::ParseError(format!("failed to parse price: {}", price)))?;

    Ok(price)
}

pub async fn convert(amount: f64, from: CurrencyCode, to: CurrencyCode) {
    let from_str = from.into();
    let to_str = to.into();

    if from == to {
        println!("{} {}", amount, from_str);
        return;
    }

    let price = make_request(amount, from_str, to_str)
        .await
        .unwrap_or_else(|e| {
            eprintln!("{}", e);
            exit(1);
        });
    let price = parse_request(&price).unwrap_or_else(|e| {
        eprintln!("{}", e);
        exit(1);
    });

    println!("{} {}", price, to_str);
}
