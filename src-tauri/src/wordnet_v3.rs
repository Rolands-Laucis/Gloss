use std::collections::HashMap;
use std::fs;
use std::sync::OnceLock;
use serde::{Deserialize, Serialize};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

// Add to Cargo.toml:
// [dependencies]
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
// fuzzy-matcher = "0.3"

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Synset {
    defs: Vec<String>,
    ex: Vec<String>,
    syns: Vec<String>,
    ants: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WordEntry {
    p: Option<Vec<String>>,  // pronoun
    n: Option<Vec<String>>,  // noun
    u: Option<Vec<String>>,  // NULL/unknown
    v: Option<Vec<String>>,  // verb
    x: Option<Vec<String>>,  // other
    a: Option<Vec<String>>,  // adjective
    r: Option<Vec<String>>,  // adverb
    s: Option<Vec<String>>,  // adj satellite
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WordNet {
    synsets: HashMap<String, Synset>,
    words: HashMap<String, WordEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordResult {
    pub word: String,
    pub pos: String,
    pub definitions: Vec<String>,
    pub examples: Vec<String>,
    pub synonyms: Vec<String>,
    pub antonyms: Vec<String>,
    pub match_score: i64,
}

#[derive(Debug, Clone)]
pub struct WordNetSearcher {
    wordnet: WordNet,
    synset_to_words: HashMap<String, Vec<String>>,
}

use std::sync::Mutex;

static WORDNET_SEARCHERS: OnceLock<Mutex<HashMap<String, WordNetSearcher>>> = OnceLock::new();

impl WordNetSearcher {
    fn new(wordnet: WordNet) -> Self {
        let mut synset_to_words = HashMap::new();
        
        // Build reverse mapping from synset IDs to words
        for (word, entry) in &wordnet.words {
            let mut all_synsets = Vec::new();
            
            if let Some(pronouns) = &entry.p {
                all_synsets.extend(pronouns.iter().cloned());
            }
            if let Some(nouns) = &entry.n {
                all_synsets.extend(nouns.iter().cloned());
            }
            if let Some(unknowns) = &entry.u {
                all_synsets.extend(unknowns.iter().cloned());
            }
            if let Some(verbs) = &entry.v {
                all_synsets.extend(verbs.iter().cloned());
            }
            if let Some(others) = &entry.x {
                all_synsets.extend(others.iter().cloned());
            }
            if let Some(adjectives) = &entry.a {
                all_synsets.extend(adjectives.iter().cloned());
            }
            if let Some(adverbs) = &entry.r {
                all_synsets.extend(adverbs.iter().cloned());
            }
            if let Some(adj_satellites) = &entry.s {
                all_synsets.extend(adj_satellites.iter().cloned());
            }
            
            for synset_id in all_synsets {
                synset_to_words
                    .entry(synset_id)
                    .or_insert_with(Vec::new)
                    .push(word.clone());
            }
        }
        
        Self {
            wordnet,
            synset_to_words,
        }
    }
    
    fn get_pos_description(pos: &str) -> &'static str {
        match pos {
            "p" => "pronoun",
            "n" => "noun", 
            "u" => "NULL/unknown",
            "v" => "verb",
            "x" => "other",
            "a" => "adjective",
            "r" => "adverb",
            "s" => "adj satellite",
            _ => "unknown",
        }
    }
    
    fn resolve_synset_ids_to_words(&self, synset_ids: &[String]) -> Vec<String> {
        let mut words = Vec::new();
        for synset_id in synset_ids {
            if let Some(synset_words) = self.synset_to_words.get(synset_id) {
                words.extend(synset_words.iter().cloned());
            }
        }
        words.sort();
        words.dedup();
        words
    }
    
    pub fn search(&self, query: &str, max_results: usize) -> Vec<WordResult> {
        let matcher = SkimMatcherV2::default();
        let mut results = Vec::new();
        
        // Search through all words
        for (word, entry) in &self.wordnet.words {
            if let Some(score) = matcher.fuzzy_match(word, query) {
                // Process each part of speech
                let pos_entries = vec![
                    ("p", &entry.p),  // pronoun
                    ("n", &entry.n),  // noun
                    ("u", &entry.u),  // NULL/unknown
                    ("v", &entry.v),  // verb
                    ("x", &entry.x),  // other
                    ("a", &entry.a),  // adjective
                    ("r", &entry.r),  // adverb
                    ("s", &entry.s),  // adj satellite
                ];
                
                for (pos, synset_ids_opt) in pos_entries {
                    if let Some(synset_ids) = synset_ids_opt {
                        for synset_id in synset_ids {
                            if let Some(synset) = self.wordnet.synsets.get(synset_id) {
                                let synonyms = self.resolve_synset_ids_to_words(&synset.syns);
                                let antonyms = self.resolve_synset_ids_to_words(&synset.ants);
                                
                                results.push(WordResult {
                                    word: word.clone(),
                                    pos: pos.to_string(),
                                    definitions: synset.defs.clone(),
                                    examples: synset.ex.clone(),
                                    synonyms,
                                    antonyms,
                                    match_score: score,
                                });
                            }
                        }
                    }
                }
            }
        }
        
        // Sort by match score (descending) and then by word (ascending)
        results.sort_by(|a, b| {
            b.match_score.cmp(&a.match_score)
                .then_with(|| a.word.cmp(&b.word))
                .then_with(|| a.pos.cmp(&b.pos))
        });
        
        // Limit results
        results.truncate(max_results);
        results
    }
}

pub fn init_wordnet(file_path: &str, language_code: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json_content = fs::read_to_string(file_path)?;
    let wordnet: WordNet = serde_json::from_str(&json_content)?;
    
    let searcher = WordNetSearcher::new(wordnet);
    
    // Get or initialize the global HashMap with Mutex
    let searchers_mutex = WORDNET_SEARCHERS.get_or_init(|| Mutex::new(HashMap::new()));
    let mut searchers = searchers_mutex.lock().unwrap();
    searchers.insert(language_code.to_string(), searcher);
    
    Ok(())
}

#[tauri::command]
pub fn search_wordnet(query: &str, language_code: &str, max_results: usize) -> Vec<WordResult> {
    if let Some(searchers_mutex) = WORDNET_SEARCHERS.get() {
        if let Ok(searchers) = searchers_mutex.lock() {
            if let Some(searcher) = searchers.get(language_code) {
                return searcher.search(query, max_results);
            }
        }
    }
    Vec::new()
}