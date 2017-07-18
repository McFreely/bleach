extern crate stemmer;

use self::stemmer::Stemmer;
use std::collections::{HashMap};

pub fn filter_unwanted_chars(doc: &String) -> String {
    let unwanted_chars: Vec<char> = ".?!',()[];{}:”“".chars().collect();
    doc.chars().filter(|c| !unwanted_chars.contains(&c) ).collect::<String>()
}

pub fn filter_stop_words(document: &String) -> Vec<&str> {
    static STOP_WORDS: &'static str = include_str!("stopwords.txt");

    // From the "file string", we construct a Set of stopwords
    let stop_words = STOP_WORDS.split_whitespace().collect::<Vec<&str>>();

    // Build the Set of words in the document
    let mut term_set = document.split_whitespace().collect::<Vec<&str>>();

    // Build the final Set of term in the document, filtered of all the stopwords
    term_set.retain(|&term| !stop_words.contains(&term));

    term_set
}

// Make it the more pluggeable possible, so that everyone can choose
// what operation to operate
pub fn clean_article(content: &String, lang: &str) -> Vec<String> {
    // Handle gracefully unknown languages
    let language = match lang {
        "fr" => "french".to_string(),
        _ => "english".to_string(),
    };

    let mut stemmer = Stemmer::new(&language).unwrap();

    // TODO
    // Benchmark lowercase a big string vs map.lowercase Vec
    let lowercase_doc = &content.to_lowercase();

    // Filter unwanted Chars
    // TODO
    // Benchmark filter a big string vs map.lowercase Vec
    let clean_doc = filter_unwanted_chars(&lowercase_doc);

    // Filter Stop Words
    // TODO
    // Add stop word list for other languages
    // benchmark actual process versus operations on Vec
    let filtered_doc = filter_stop_words(&clean_doc);

    // Stemmification
    // Look for better stemmers ?
    let mut stemmed_doc = Vec::new();
    for word in filtered_doc.into_iter() {
        if word != "" {
            stemmed_doc.push(stemmer.stem(&word));
        }
    }

    stemmed_doc
}

#[cfg(test)]
mod tests {
    use super::filter_unwanted_chars;
    use super::filter_stop_words;

    #[test]
    fn it_filters_unwanted_chars() {
        let doc = "this is, awesome!".to_string();
        let filtered_doc = filter_unwanted_chars(&doc);

        assert_eq!(filtered_doc, "this is awesome");
    }

    #[test]
    fn it_removes_stop_words() {
        let doc = "this is awesome".to_string();
        let filtered_doc = filter_stop_words(&doc);

        assert_eq!(filtered_doc, vec!["awesome"]);
    }
}

