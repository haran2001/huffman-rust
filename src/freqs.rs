pub fn count_words(text: &str) -> HashMap<String, u32> {
    let mut word_counts = HashMap::new();
    for word in text.split_whitespace() {
        *word_counts.entry(word.to_string()).or_insert(0) += 1;
    }
    word_counts
}

pub fn count_characters(text: &str) -> HashMap<String, u32> {
    let mut char_counts = HashMap::new();
    for char in text.chars() {
        *char_counts.entry(char.to_string()).or_insert(0) += 1;
    }
    char_counts
}

