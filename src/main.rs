use reqwest::header::HeaderMap;
use soup::{Soup, QueryBuilderExt, NodeExt};

fn main() {
    let user_agent_desktop = include_str!("../resources/useragent.txt");
    let base_url = "https://www.sslproxies.org";

    let mut headers = HeaderMap::new();
    headers.append("User-Agent", user_agent_desktop.parse().unwrap());

    let client = reqwest::blocking::ClientBuilder::new()
        .cookie_store(true)
        .default_headers(headers)
        .build().expect("error creating client");

    let response_text = client
        .get(base_url)
        .send()
        .expect("error getting response")
        .text()
        .expect("error getting response text");

    let soup = Soup::new(response_text.as_str());

    let tables = soup.tag("table")
        .filter(|tag| return
            tag.get("id").unwrap_or("".into()).eq("proxylisttable")
        );

    println!("{}", tables.len());
}
