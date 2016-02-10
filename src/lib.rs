extern crate stemmer;

use self::stemmer::Stemmer;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::{HashMap};

pub fn filter_unwanted_chars(doc: &String) -> String {
    let unwanted_chars: Vec<char> = ".?!',()[];{}:”“".chars().collect();
    doc.chars().filter(|c| !unwanted_chars.contains(&c) ).collect::<String>()
}

pub fn filter_stop_words(document: &String) -> Vec<&str> {
    // We open the stopwords file...
    let path = Path::new("stopwords.txt");
    let mut file = match File::open(&path) {
        Err(why) => panic!(println!("Erreur ouverture fichier: {}", why)),
        Ok(file) => file,
    };

    // ... and we put its content into the new string
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Counldn't open file: {}", Error::description(&why)),
        Ok(_) => (),
    }

    // From the "file string", we construct a Set of stopwords
    let stop_words = s.split_whitespace().collect::<Vec<&str>>();

    // Build the Set of words in the document
    let mut term_set = document.split_whitespace().collect::<Vec<&str>>();

    // Build the final Set of term in the document, filtered of all the stopwords
    term_set.retain(|&term| !stop_words.contains(&term));

    term_set
}

// Make it the more pluggeable possible, so that everyone can choose
// what operation to operate
pub fn clean_doc(content: &String, lang: &str) -> Vec<String> {
    let mut langs = HashMap::new();
    langs.insert("en", "english".to_string());
    langs.insert("fr", "french".to_string());

    // Handle gracefully unknown languages
    let mut stemmer = Stemmer::new(langs.get(&lang).expect("Language not found.")).unwrap();

    // TODO 
    // Benchmark lowercase a big string vs map.lowercase Vec
    let lowercase_doc = &content.to_lowercase();

    // Filter unwanted Chars
    // TODO 
    // Benchmark filter a big string vs map.lowercase Vec
    let clean_doc = filter_unwanted_chars(&lowercase_doc);

    // Filter Stop Words
    // TODO 
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

