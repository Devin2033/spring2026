fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect();

    // To store unique words and counts
    let mut unique_words: Vec<&str> = Vec::new();
    let mut counts: Vec<usize> = Vec::new();

    for word in &words {
        let mut found = false;
        for i in 0..unique_words.len() {
            if unique_words[i] == *word {
                counts[i] += 1;
                found = true;
                break;
            }
        }
        if !found {
            unique_words.push(word);
            counts.push(1);
        }
    }

    // To find the index of max count
    let mut max_index = 0;
    for i in 1..counts.len() {
        if counts[i] > counts[max_index] {
            max_index = 1;
        }
    }

    (unique_words[max_index].to_string(), counts[max_index])
}


fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}