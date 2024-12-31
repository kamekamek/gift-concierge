use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref GIFT_CACHE: Mutex<HashMap<String, Vec<GiftInfo>>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Clone)]
struct GiftInfo {
    name: String,
    image_url: Option<String>,
    price: f32,
    description: String,
}

impl GiftInfo {
    fn new(name: String, image_url: Option<String>, price: f32, description: String) -> Self {
        GiftInfo {
            name,
            image_url,
            price,
            description,
        }
    }
}

fn cache_gift_info(query: &str, gifts: Vec<GiftInfo>) {
    let mut cache = GIFT_CACHE.lock().unwrap();
    cache.insert(query.to_string(), gifts);
}

fn get_cached_gift_info(query: &str) -> Option<Vec<GiftInfo>> {
    let cache = GIFT_CACHE.lock().unwrap();
    cache.get(query).cloned()
}

fn main() {
    // Example usage
    let gifts = vec![
        GiftInfo::new(
            "Elegant Pen".to_string(),
            Some("https://example.com/pen.jpg".to_string()),
            29.99,
            "A beautifully crafted pen.".to_string(),
        ),
        GiftInfo::new(
            "Leather Wallet".to_string(),
            Some("https://example.com/wallet.jpg".to_string()),
            49.99,
            "A premium leather wallet.".to_string(),
        ),
    ];

    let query = "executive gifts";
    cache_gift_info(query, gifts.clone());

    if let Some(cached_gifts) = get_cached_gift_info(query) {
        println!("Cached gifts for '{}': {:?}", query, cached_gifts);
    } else {
        println!("No cached gifts found for '{}'", query);
    }
};