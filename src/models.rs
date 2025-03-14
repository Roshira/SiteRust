use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CryptoCurrency {
    pub id: u32,
    pub name: String,
    pub symbol: String,
    pub quote: Quote,
}

#[derive(Deserialize, Debug)]
pub struct Quote {
    #[serde(rename = "USD")]
    pub usd: UsdData,
}

#[derive(Deserialize, Debug)]
pub struct UsdData {
    pub price: f64,
    pub volume_24h: f64,
}