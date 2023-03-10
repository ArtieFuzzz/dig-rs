pub mod types;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};

pub async fn dig(
    domain: impl Into<String>,
    record: DnsRecord,
) -> Result<types::CloudFlareResponse, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!(
        "https://cloudflare-dns.com/query?name={}&type={record:?}",
        domain.into()
    );

    let mut headers = HeaderMap::new();
    headers.insert("Accept", HeaderValue::from_static("application/dns-json"));

    let response = Client::builder()
        .default_headers(headers)
        .build()?
        .get(url)
        .send()
        .await?
        .json::<types::CloudFlareResponse>()
        .await?;

    Ok(response)
}

#[derive(Debug)]
pub enum DnsRecord {
    A = 1,
    AAAA = 28,
    NS = 2,
    CNAME = 5,
    MX = 15,
    TXT = 16,
}
