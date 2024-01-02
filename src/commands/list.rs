use strum::IntoEnumIterator;

use crate::currency_code::CurrencyCode;

pub fn list() {
    println!(
        "{}",
        CurrencyCode::iter()
            .map(|c| c.into())
            .collect::<Vec<&'static str>>()
            .join(", ")
    );
}
