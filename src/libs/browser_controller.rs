use urlencoding::encode;

pub fn search_in_browser(query: &str, search_engine: &str) {
    let encoded_query = encode(query);
    let search_url = match search_engine {
        "Google" => format!("https://www.google.com/search?q={}", encoded_query),
        "DuckDuckGo" => format!("https://duckduckgo.com/?q={}", encoded_query),
        "Bing" => format!("https://www.bing.com/search?q={}", encoded_query),
        _ => panic!("Invalid search engine"),
    };

    match open::that(search_url) {
        Ok(_) => (),
        Err(_) => panic!("Failed to open browser"),
    }
}
