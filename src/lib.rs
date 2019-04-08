#[macro_use]
extern crate lazy_static;

use std::collections::HashSet;
use std::error::Error;
use std::cmp::Reverse;
use std::path::PathBuf;
use wasm_bindgen::prelude::*;

mod types;
use types::{Filters, Storage, Score};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn load_filters() -> Result<Filters, Box<Error>> {
    let bytes = include_bytes!("../storage");
    Ok(Storage::from_bytes(bytes)?.filters)
}

lazy_static! {
    static ref FILTERS: Filters = load_filters().unwrap();
}

#[wasm_bindgen]
pub fn search(query: String, num_results: usize) -> JsValue {
    let search_terms: HashSet<String> =
        query.split_whitespace().map(|s| s.to_lowercase()).collect();

    let mut matches: Vec<(&PathBuf, u32)> = FILTERS
        .iter()
        .map(|(name, filter)| (name, filter.score(&search_terms)))
        .filter(|(_, score)| *score > 0)
        .collect();

    matches.sort_by_key(|k| Reverse(k.1));
    
    let results: Vec<&PathBuf> = matches.iter()
        .map(|(name, _)| name.to_owned())
        .take(num_results)
        .collect();

    JsValue::from_serde(&results).unwrap()
}
