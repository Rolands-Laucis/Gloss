use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::sync::OnceLock;

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
    p: Option<Vec<String>>, // pronoun
    n: Option<Vec<String>>, // noun
    u: Option<Vec<String>>, // NULL/unknown
    v: Option<Vec<String>>, // verb
    x: Option<Vec<String>>, // other
    a: Option<Vec<String>>, // adjective
    r: Option<Vec<String>>, // adverb
    s: Option<Vec<String>>, // adj satellite
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
                // Debug: Print the base score for this word
                // println!("Word: '{}' -> Base score: {}", word, score);

                // Process each part of speech
                let pos_entries = vec![
                    ("p", &entry.p), // pronoun
                    ("n", &entry.n), // noun
                    ("u", &entry.u), // NULL/unknown
                    ("v", &entry.v), // verb
                    ("x", &entry.x), // other
                    ("a", &entry.a), // adjective
                    ("r", &entry.r), // adverb
                    ("s", &entry.s), // adj satellite
                ];

                for (pos, synset_ids_opt) in pos_entries {
                    if let Some(synset_ids) = synset_ids_opt {
                        for synset_id in synset_ids {
                            if let Some(synset) = self.wordnet.synsets.get(synset_id) {
                                let synonyms = self.resolve_synset_ids_to_words(&synset.syns);
                                let antonyms = self.resolve_synset_ids_to_words(&synset.ants);

                                // Add some variation to score based on synset quality
                                let adjusted_score = score
                                    + (synset.defs.len() as i64 * 2)
                                    + (synset.ex.len() as i64);

                                results.push(WordResult {
                                    word: word.clone(),
                                    pos: pos.to_string(),
                                    definitions: synset.defs.clone(),
                                    examples: synset.ex.clone(),
                                    synonyms,
                                    antonyms,
                                    match_score: adjusted_score,
                                });
                            }
                        }
                    }
                }
            }
        }

        // Sort by: 1) exact matches first, 2) single words, 3) match score (descending), 4) word (ascending), 5) pos
        results.sort_by(|a, b| {
            let a_is_exact = a.word.to_lowercase() == query.to_lowercase();
            let b_is_exact = b.word.to_lowercase() == query.to_lowercase();
            let a_is_single = !a.word.contains(' ') && !a.word.contains('-');
            let b_is_single = !b.word.contains(' ') && !b.word.contains('-');
            
            b_is_exact.cmp(&a_is_exact)  // Exact matches first (true > false)
                .then_with(|| b_is_single.cmp(&a_is_single))  // Single words first (true > false)
                .then_with(|| b.match_score.cmp(&a.match_score))  // Higher scores first
                .then_with(|| a.word.cmp(&b.word))  // Alphabetical
                .then_with(|| a.pos.cmp(&b.pos))   // POS as final tiebreaker
        });

        // Limit results
        results.truncate(max_results);
        results
    }
}

pub fn init_wordnet(
    file_path: &str,
    language_code: &str,
) -> Result<(), Box<dyn std::error::Error>> {
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

/// Efficiently get the first definition for multiple words without fuzzy matching.
/// Returns a HashMap of word -> first definition (or empty string if not found).
#[tauri::command]
pub fn get_first_definitions(words: Vec<String>, language_code: &str) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();
    
    if let Some(searchers_mutex) = WORDNET_SEARCHERS.get() {
        if let Ok(searchers) = searchers_mutex.lock() {
            if let Some(searcher) = searchers.get(language_code) {
                for word in words {
                    if let Some(entry) = searcher.wordnet.words.get(&word) {
                        // Try to find the first definition from any POS
                        let first_def = get_first_def_from_entry(entry, &searcher.wordnet.synsets);
                        result.insert(word, first_def.unwrap_or_default());
                    } else {
                        result.insert(word, String::new());
                    }
                }
            }
        }
    }
    
    result
}

/// Helper function to get the first definition from a word entry
fn get_first_def_from_entry(entry: &WordEntry, synsets: &HashMap<String, Synset>) -> Option<String> {
    // Check each POS in order of common usage
    let pos_fields: [&Option<Vec<String>>; 8] = [
        &entry.n, // noun
        &entry.v, // verb
        &entry.a, // adjective
        &entry.r, // adverb
        &entry.s, // adj satellite
        &entry.p, // pronoun
        &entry.x, // other
        &entry.u, // unknown
    ];
    
    for synset_ids_opt in pos_fields {
        if let Some(synset_ids) = synset_ids_opt {
            for synset_id in synset_ids {
                if let Some(synset) = synsets.get(synset_id) {
                    if let Some(first_def) = synset.defs.first() {
                        return Some(first_def.clone());
                    }
                }
            }
        }
    }
    
    None
}

/// Add a new word entry with a single sense to the wordnet
#[tauri::command]
pub fn add_word_entry(
    word: String,
    pos: String,
    definition: String,
    language_code: &str,
    file_path: &str,
) -> Result<(), String> {
    if let Some(searchers_mutex) = WORDNET_SEARCHERS.get() {
        if let Ok(mut searchers) = searchers_mutex.lock() {
            if let Some(searcher) = searchers.get_mut(language_code) {
                // Generate a unique synset ID
                let synset_id = format!("custom-{}-{}", word.replace(' ', "_"), uuid_simple());
                
                // Create the new synset
                let new_synset = Synset {
                    defs: vec![definition],
                    ex: vec![],
                    syns: vec![],
                    ants: vec![],
                };
                
                // Add synset to wordnet
                searcher.wordnet.synsets.insert(synset_id.clone(), new_synset);
                
                // Add or update word entry
                let entry = searcher.wordnet.words.entry(word.clone()).or_insert_with(|| WordEntry {
                    p: None, n: None, u: None, v: None, x: None, a: None, r: None, s: None,
                });
                
                // Add synset ID to the appropriate POS field
                match pos.as_str() {
                    "p" => entry.p.get_or_insert_with(Vec::new).push(synset_id.clone()),
                    "n" => entry.n.get_or_insert_with(Vec::new).push(synset_id.clone()),
                    "u" => entry.u.get_or_insert_with(Vec::new).push(synset_id.clone()),
                    "v" => entry.v.get_or_insert_with(Vec::new).push(synset_id.clone()),
                    "x" => entry.x.get_or_insert_with(Vec::new).push(synset_id.clone()),
                    "a" => entry.a.get_or_insert_with(Vec::new).push(synset_id.clone()),
                    "r" => entry.r.get_or_insert_with(Vec::new).push(synset_id.clone()),
                    "s" => entry.s.get_or_insert_with(Vec::new).push(synset_id.clone()),
                    _ => return Err(format!("Invalid POS: {}", pos)),
                }
                
                // Update reverse mapping
                searcher.synset_to_words
                    .entry(synset_id)
                    .or_insert_with(Vec::new)
                    .push(word);
                
                // Persist to file
                let json = serde_json::to_string(&searcher.wordnet)
                    .map_err(|e| format!("Failed to serialize: {}", e))?;
                fs::write(file_path, json)
                    .map_err(|e| format!("Failed to write file: {}", e))?;
                
                return Ok(());
            }
        }
    }
    Err("WordNet not initialized".to_string())
}

/// Add a new sense (definition) to an existing word
#[tauri::command]
pub fn add_sense_to_word(
    word: String,
    pos: String,
    definition: String,
    language_code: &str,
    file_path: &str,
) -> Result<(), String> {
    if let Some(searchers_mutex) = WORDNET_SEARCHERS.get() {
        if let Ok(mut searchers) = searchers_mutex.lock() {
            if let Some(searcher) = searchers.get_mut(language_code) {
                // Check if word exists
                if !searcher.wordnet.words.contains_key(&word) {
                    return Err(format!("Word '{}' not found", word));
                }
                
                // Generate a unique synset ID
                let synset_id = format!("custom-{}-{}", word.replace(' ', "_"), uuid_simple());
                
                // Create the new synset
                let new_synset = Synset {
                    defs: vec![definition],
                    ex: vec![],
                    syns: vec![],
                    ants: vec![],
                };
                
                // Add synset to wordnet
                searcher.wordnet.synsets.insert(synset_id.clone(), new_synset);
                
                // Get the word entry and add synset ID to the appropriate POS field
                let entry = searcher.wordnet.words.get_mut(&word).unwrap();
                match pos.as_str() {
                    "p" => entry.p.get_or_insert_with(Vec::new).push(synset_id.clone()),
                    "n" => entry.n.get_or_insert_with(Vec::new).push(synset_id.clone()),
                    "u" => entry.u.get_or_insert_with(Vec::new).push(synset_id.clone()),
                    "v" => entry.v.get_or_insert_with(Vec::new).push(synset_id.clone()),
                    "x" => entry.x.get_or_insert_with(Vec::new).push(synset_id.clone()),
                    "a" => entry.a.get_or_insert_with(Vec::new).push(synset_id.clone()),
                    "r" => entry.r.get_or_insert_with(Vec::new).push(synset_id.clone()),
                    "s" => entry.s.get_or_insert_with(Vec::new).push(synset_id.clone()),
                    _ => return Err(format!("Invalid POS: {}", pos)),
                }
                
                // Update reverse mapping
                searcher.synset_to_words
                    .entry(synset_id)
                    .or_insert_with(Vec::new)
                    .push(word);
                
                // Persist to file
                let json = serde_json::to_string(&searcher.wordnet)
                    .map_err(|e| format!("Failed to serialize: {}", e))?;
                fs::write(file_path, json)
                    .map_err(|e| format!("Failed to write file: {}", e))?;
                
                return Ok(());
            }
        }
    }
    Err("WordNet not initialized".to_string())
}

/// Simple UUID generator for unique IDs
fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("{:x}{:x}", duration.as_secs(), duration.subsec_nanos())
}

/// Delete an entire word entry from the wordnet
#[tauri::command]
pub fn delete_word_entry(
    word: String,
    language_code: &str,
    file_path: &str,
) -> Result<(), String> {
    if let Some(searchers_mutex) = WORDNET_SEARCHERS.get() {
        if let Ok(mut searchers) = searchers_mutex.lock() {
            if let Some(searcher) = searchers.get_mut(language_code) {
                // Check if word exists
                if !searcher.wordnet.words.contains_key(&word) {
                    return Err(format!("Word '{}' not found", word));
                }
                
                // Get all synset IDs associated with this word
                let entry = searcher.wordnet.words.get(&word).unwrap();
                let mut synset_ids_to_check: Vec<String> = Vec::new();
                
                if let Some(ids) = &entry.p { synset_ids_to_check.extend(ids.clone()); }
                if let Some(ids) = &entry.n { synset_ids_to_check.extend(ids.clone()); }
                if let Some(ids) = &entry.u { synset_ids_to_check.extend(ids.clone()); }
                if let Some(ids) = &entry.v { synset_ids_to_check.extend(ids.clone()); }
                if let Some(ids) = &entry.x { synset_ids_to_check.extend(ids.clone()); }
                if let Some(ids) = &entry.a { synset_ids_to_check.extend(ids.clone()); }
                if let Some(ids) = &entry.r { synset_ids_to_check.extend(ids.clone()); }
                if let Some(ids) = &entry.s { synset_ids_to_check.extend(ids.clone()); }
                
                // Remove word from words map
                searcher.wordnet.words.remove(&word);
                
                // Update synset_to_words and remove orphaned synsets
                for synset_id in synset_ids_to_check {
                    if let Some(words) = searcher.synset_to_words.get_mut(&synset_id) {
                        words.retain(|w| w != &word);
                        // If no words reference this synset anymore, remove the synset
                        if words.is_empty() {
                            searcher.synset_to_words.remove(&synset_id);
                            searcher.wordnet.synsets.remove(&synset_id);
                        }
                    }
                }
                
                // Persist to file
                let json = serde_json::to_string(&searcher.wordnet)
                    .map_err(|e| format!("Failed to serialize: {}", e))?;
                fs::write(file_path, json)
                    .map_err(|e| format!("Failed to write file: {}", e))?;
                
                return Ok(());
            }
        }
    }
    Err("WordNet not initialized".to_string())
}

/// Delete a specific sense (synset) from a word
#[tauri::command]
pub fn delete_sense_from_word(
    word: String,
    pos: String,
    synset_index: usize,
    language_code: &str,
    file_path: &str,
) -> Result<(), String> {
    if let Some(searchers_mutex) = WORDNET_SEARCHERS.get() {
        if let Ok(mut searchers) = searchers_mutex.lock() {
            if let Some(searcher) = searchers.get_mut(language_code) {
                // Check if word exists
                let entry = searcher.wordnet.words.get_mut(&word)
                    .ok_or_else(|| format!("Word '{}' not found", word))?;
                
                // Get the synset ID to remove based on POS and index
                let synset_id = {
                    let synset_ids = match pos.as_str() {
                        "p" => entry.p.as_ref(),
                        "n" => entry.n.as_ref(),
                        "u" => entry.u.as_ref(),
                        "v" => entry.v.as_ref(),
                        "x" => entry.x.as_ref(),
                        "a" => entry.a.as_ref(),
                        "r" => entry.r.as_ref(),
                        "s" => entry.s.as_ref(),
                        _ => return Err(format!("Invalid POS: {}", pos)),
                    };
                    
                    synset_ids
                        .and_then(|ids| ids.get(synset_index))
                        .ok_or_else(|| format!("Sense index {} not found for POS '{}'", synset_index, pos))?
                        .clone()
                };
                
                // Remove the synset ID from the entry's POS field
                let synset_ids_mut = match pos.as_str() {
                    "p" => entry.p.as_mut(),
                    "n" => entry.n.as_mut(),
                    "u" => entry.u.as_mut(),
                    "v" => entry.v.as_mut(),
                    "x" => entry.x.as_mut(),
                    "a" => entry.a.as_mut(),
                    "r" => entry.r.as_mut(),
                    "s" => entry.s.as_mut(),
                    _ => return Err(format!("Invalid POS: {}", pos)),
                };
                
                if let Some(ids) = synset_ids_mut {
                    if synset_index < ids.len() {
                        ids.remove(synset_index);
                    }
                }
                
                // Update synset_to_words and potentially remove orphaned synset
                if let Some(words) = searcher.synset_to_words.get_mut(&synset_id) {
                    words.retain(|w| w != &word);
                    if words.is_empty() {
                        searcher.synset_to_words.remove(&synset_id);
                        searcher.wordnet.synsets.remove(&synset_id);
                    }
                }
                
                // Check if word entry is now empty and should be removed
                let entry = searcher.wordnet.words.get(&word).unwrap();
                let is_empty = entry.p.as_ref().map_or(true, |v| v.is_empty())
                    && entry.n.as_ref().map_or(true, |v| v.is_empty())
                    && entry.u.as_ref().map_or(true, |v| v.is_empty())
                    && entry.v.as_ref().map_or(true, |v| v.is_empty())
                    && entry.x.as_ref().map_or(true, |v| v.is_empty())
                    && entry.a.as_ref().map_or(true, |v| v.is_empty())
                    && entry.r.as_ref().map_or(true, |v| v.is_empty())
                    && entry.s.as_ref().map_or(true, |v| v.is_empty());
                
                if is_empty {
                    searcher.wordnet.words.remove(&word);
                }
                
                // Persist to file
                let json = serde_json::to_string(&searcher.wordnet)
                    .map_err(|e| format!("Failed to serialize: {}", e))?;
                fs::write(file_path, json)
                    .map_err(|e| format!("Failed to write file: {}", e))?;
                
                return Ok(());
            }
        }
    }
    Err("WordNet not initialized".to_string())
}
