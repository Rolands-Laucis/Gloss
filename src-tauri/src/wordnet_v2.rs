use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::sync::OnceLock;
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcher};

// WordNet JSON Schema Structures
#[derive(Debug, Deserialize, Clone)]
struct WordNet {
    synset: HashMap<String, Synset>,
    lemma: HashMap<String, Vec<String>>,
    #[serde(rename = "lemmaRanked")]
    lemma_ranked: Option<HashMap<String, Vec<String>>>,
    exception: Option<HashMap<String, Vec<String>>>,
    example: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Clone)]
struct Synset {
    offset: i32,
    pos: String,
    word: Vec<String>,
    pointer: Option<Vec<Pointer>>,
    frame: Option<Vec<Frame>>,
    gloss: String,
    example: Option<Vec<Example>>,
}

#[derive(Debug, Deserialize, Clone)]
struct Pointer {
    symbol: String,
    synset: String,
    source: i32,
    target: i32,
}

#[derive(Debug, Deserialize, Clone)]
struct Frame {
    #[serde(rename = "wordNumber")]
    word_number: i32,
    #[serde(rename = "frameNumber")]
    frame_number: i32,
}

#[derive(Debug, Deserialize, Clone)]
struct Example {
    // Example structure varies, keeping it flexible
    #[serde(flatten)]
    data: HashMap<String, serde_json::Value>,
}

// Search Result Structure for Tauri (includes score)
#[derive(Debug, Clone, Serialize)]
pub struct SearchResult {
    pub entry: WordEntry,
    pub score: i64,
}

// Search Result Structure
#[derive(Debug, Clone, Serialize)]
pub struct WordEntry {
    pub word: String,
    pub definition: String,
    pub pos: String,
    pub synonyms: Vec<String>,
    pub example: Option<String>,
    pub score: i64, // Add score field
}

// Fuzzy Search Index
// #[derive(Debug)]
pub struct FuzzySearchIndex {
    entries: Vec<WordEntry>,
    matcher: SkimMatcher,
}

impl FuzzySearchIndex {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
            matcher: SkimMatcher::default(),
        }
    }

    fn add_entry(&mut self, entry: WordEntry) {
        self.entries.push(entry);
    }

    pub fn search(&self, query: &str, limit: usize) -> Vec<(WordEntry, i64)> {
        let mut results: Vec<(WordEntry, i64)> = self
            .entries
            .iter()
            .filter_map(|entry| {
                self.matcher
                    .fuzzy_match(&entry.word, query)
                    .map(|score| (entry.clone(), score))
            })
            .collect();

        // Sort by score (descending)
        results.sort_by(|a, b| b.1.cmp(&a.1));
        results.truncate(limit);
        results
    }

    pub fn get_all_entries(&self) -> &Vec<WordEntry> {
        &self.entries
    }
}

// Global instance
static WORDNET_INDEX: OnceLock<FuzzySearchIndex> = OnceLock::new();

// POS code mapping
fn pos_code_to_full(pos: &str) -> String {
    match pos {
        "a" => "adjective".to_string(),
        "n" => "noun".to_string(),
        "r" => "adverb".to_string(),
        "s" => "satellite adjective".to_string(),
        "v" => "verb".to_string(),
        _ => pos.to_string(),
    }
}

// Extract synonyms and antonyms from pointers
fn extract_relations(
    synset: &Synset,
    wordnet: &WordNet,
) -> Vec<String> {
    let mut synonyms = Vec::new();

    if let Some(pointers) = &synset.pointer {
        for pointer in pointers {
            match pointer.symbol.as_str() {
                "~" | "@" | "^" | "=" => {
                    // Various synonym-like relations (hypernym, hyponym, etc.)
                    if let Some(target_synset) = wordnet.synset.get(&pointer.synset) {
                        synonyms.extend(target_synset.word.clone());
                    }
                }
                _ => {}
            }
        }
    }

    // Add words from the same synset as synonyms (excluding the word itself)
    for word in &synset.word {
        if !synonyms.contains(word) {
            synonyms.push(word.clone());
        }
    }

    // Remove duplicates
    synonyms.sort();
    synonyms.dedup();

    synonyms
}

// Extract example sentence
fn extract_example(synset: &Synset, wordnet: &WordNet) -> Option<String> {
    // Try to get from synset examples first
    if let Some(examples) = &synset.example {
        if let Some(first_example) = examples.first() {
            // This is a simplified extraction - actual structure may vary
            if let Some(text) = first_example.data.get("text") {
                if let Some(text_str) = text.as_str() {
                    return Some(text_str.to_string());
                }
            }
        }
    }

    // Try to extract from gloss (often contains examples in quotes)
    let gloss = &synset.gloss;
    if let Some(quote_start) = gloss.find('"') {
        if let Some(quote_end) = gloss[quote_start + 1..].find('"') {
            let example = &gloss[quote_start + 1..quote_start + 1 + quote_end];
            return Some(example.to_string());
        }
    }

    None
}

// Build the fuzzy search index from WordNet data
fn build_index(wordnet: WordNet) -> FuzzySearchIndex {
    let mut index = FuzzySearchIndex::new();

    for (synset_id, synset) in &wordnet.synset {
        let synonyms = extract_relations(synset, &wordnet);
        let example = extract_example(synset, &wordnet);
        let pos_full = pos_code_to_full(&synset.pos);

        for word in &synset.word {
            // Skip words with more than one space character
            if word.chars().filter(|&c| c == '_').count() > 1 {
                continue;
            }

            // Clean word (remove POS tags if present)
            let clean_word = word.split('.').next().unwrap_or(word).to_string();
            
            // Filter out the current word from synonyms
            let filtered_synonyms: Vec<String> = synonyms
                .iter()
                .filter(|&s| s != word && s != &clean_word)
                .cloned()
                .collect();

            let entry = WordEntry {
                word: clean_word,
                definition: synset.gloss.clone(),
                pos: pos_full.clone(),
                synonyms: filtered_synonyms,
                example: example.clone(),
                score: 0, // Initialize score
            };

            index.add_entry(entry);
        }
    }

    index
}

// Tauri command to initialize the WordNet index
#[tauri::command]
pub fn initialize_wordnet() -> Result<String, String> {
    initialize_wordnet_index("resources/wordnet.json")
        .map(|_| "WordNet index initialized successfully!".to_string())
        .map_err(|e| format!("Failed to initialize WordNet index: {}", e))
}

// Main function to read and parse the WordNet JSON file
pub fn initialize_wordnet_index(json_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read the JSON file
    let json_content = fs::read_to_string(json_file_path)?;
    
    // Parse the JSON into WordNet structure
    let wordnet: WordNet = serde_json::from_str(&json_content)?;
    
    // Build the fuzzy search index
    let index = build_index(wordnet);
    
    // Set the global instance
    WORDNET_INDEX.set(index).map_err(|_| "Failed to set global WordNet index")?;
    
    println!("WordNet index initialized successfully!");
    Ok(())
}

// Tauri command for fuzzy search
#[tauri::command]
pub fn search_words(query: String, limit: Option<usize>) -> Vec<WordEntry> {
    let limit = limit.unwrap_or(10); // Default to 10 results
    
    if let Some(index) = WORDNET_INDEX.get() {
        index.search(&query, limit)
            .into_iter()
            .map(|(mut entry, score)| {
                entry.score = score; // Add score directly to the entry
                entry
            })
            .collect()
    } else {
        Vec::new()
    }
}

// Internal search function (for non-Tauri usage)
pub fn search_words_internal(query: &str, limit: usize) -> Vec<(WordEntry, i64)> {
    if let Some(index) = WORDNET_INDEX.get() {
        index.search(query, limit)
    } else {
        Vec::new()
    }
}

// Tauri command to get index status
#[tauri::command]
pub fn get_index_status() -> IndexStatus {
    IndexStatus {
        is_initialized: WORDNET_INDEX.get().is_some(),
        entry_count: WORDNET_INDEX.get().map(|index| index.entries.len()).unwrap_or(0),
    }
}

// Index status structure
#[derive(Debug, Clone, Serialize)]
pub struct IndexStatus {
    pub is_initialized: bool,
    pub entry_count: usize,
}

// Example usage and testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_functionality() {
        // This test assumes you have a wordnet.json file
        // initialize_wordnet_index("wordnet.json").unwrap();
        
        // let results = search_words("happy", 5);
        // println!("Search results for 'happy': {:#?}", results);
    }
}

// Get all entries (for debugging or full access)
pub fn get_all_entries() -> Option<Vec<WordEntry>> {
    WORDNET_INDEX.get().map(|index| index.get_all_entries().clone())
}