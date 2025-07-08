use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use once_cell::sync::Lazy;
use serde::Deserialize;
use tauri::command;

#[derive(Debug, Deserialize)]
struct WordNet {
    synset: HashMap<String, Synset>,
    lemma: HashMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct Pointer {
    symbol: String,
    synset: String,
    source: Option<i32>, // Changed to Option<i32> to handle missing or null values
    target: Option<i32>, // Changed to Option<i32> to handle missing or null values
}

#[derive(Debug, Deserialize)]
struct Synset {
    offset: i64,
    pos: String,
    word: Vec<String>,
    gloss: String, // Gloss is confirmed to always be a string
    example: Option<Vec<String>>,
    pointer: Option<Vec<Pointer>>, // Changed to Option<Vec<Pointer>> to handle missing or null values
    // other fields omitted for simplicity
}

#[derive(Debug, Clone, serde::Serialize)]
struct Entry {
    word: String,
    definitions: Vec<String>,
    pos: String,
    synonyms: Vec<String>,
    antonyms: Vec<String>,
    example: Option<String>,
}

// Global state
static DICTIONARY: Lazy<Mutex<Vec<Entry>>> = Lazy::new(|| Mutex::new(vec![]));
static WORDS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(vec![]));
static MATCHER: Lazy<SkimMatcherV2> = Lazy::new(SkimMatcherV2::default);

#[command]
fn search(term: String, max_results: usize) -> Vec<Entry> {
    let words = WORDS.lock().unwrap();
    let dict = DICTIONARY.lock().unwrap();

    let mut scored: Vec<(i64, usize)> = words
        .iter()
        .enumerate()
        .filter_map(|(i, word)| MATCHER.fuzzy_match(word, &term).map(|score| (score, i)))
        .collect();

    scored.sort_by(|a, b| b.0.cmp(&a.0));

    scored
        .into_iter()
        .take(max_results)
        .map(|(_, idx)| {
            let entry = &dict[idx];
            let word = entry.word.split('.').last().unwrap_or(&entry.word).to_string();
            let synonyms = entry.synonyms.clone();
            let antonyms = entry.antonyms.clone();
            let example = entry.example.clone();
            Entry {
                word,
                definitions: entry.definitions.clone(), // Fixed type mismatch
                pos: entry.pos.clone(),
                synonyms,
                antonyms,
                example,
            }
        })
        .collect()
}

fn init_wordnet(json_file_path: &str){
    let json = fs::read_to_string("resources/wordnet.json").expect("failed to read json");
    let wordnet: WordNet = serde_json::from_str(&json).expect("failed to parse");

    let mut entries = vec![];
    let mut words = vec![];

    for (lemma_key, synset_ids) in &wordnet.lemma {
        if let Some(first_synset_id) = synset_ids.first() {
            if let Some(synset) = wordnet.synset.get(first_synset_id) {
                let antonyms = synset.pointer.as_ref().map_or(vec![], |pointers| {
                    pointers.iter()
                        .filter(|p| p.symbol == "!" && wordnet.synset.contains_key(&p.synset))
                        .flat_map(|p| wordnet.synset[&p.synset].word.clone())
                        .collect()
                });

                let entry = Entry {
                    word: lemma_key.clone(),
                    definitions: vec![synset.gloss.clone()],
                    pos: synset.pos.clone(),
                    synonyms: synset.word.clone(),
                    antonyms,
                    example: synset.example.as_ref().and_then(|examples| examples.first().cloned()),
                };
                words.push(lemma_key.clone());
                entries.push(entry);
            }
        }
    }

    {
        let mut dict_lock = DICTIONARY.lock().unwrap();
        let mut words_lock = WORDS.lock().unwrap();
        *dict_lock = entries;
        *words_lock = words;
    }
}