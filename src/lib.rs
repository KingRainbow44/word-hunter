// This flag is required because Java names follow camelCase.
#![allow(non_snake_case)]

mod solver;
mod trie_node;

use std::sync::RwLock;
use jni::JNIEnv;
use jni::objects::{JClass, JObjectArray, JString};
use jni::sys::{jint, jobjectArray};
use lazy_static::lazy_static;
use crate::solver::Solver;
use crate::trie_node::TrieNode;

lazy_static! {
    pub static ref DICTIONARY: RwLock<TrieNode> = RwLock::new(TrieNode::new());
}

/// Loads a dictionary file.
/// path: The path to the dictionary file.
pub fn load_dictionary(path: String) {
    // Lock the dictionary.
    let mut dictionary = DICTIONARY.write().unwrap();

    // Check if the file exists.
    if !std::fs::exists(&path)
        .expect("Couldn't check if the dictionary file exists.") {
        return;
    }

    // Read the dictionary file.
    let contents = std::fs::read_to_string(&path)
        .expect("Couldn't read the dictionary file.");

    // Split the contents by newlines.
    for word in contents.lines() {
        dictionary.insert(word.to_lowercase().to_string());
    }

    // Unlock the dictionary.
    drop(dictionary);
}

/// Finds all words on a 2D board.
/// board: A 2D vector of strings.
pub fn solve_words(board: Vec<Vec<String>>) -> Vec<String> {
    let solver = Solver::new();
    solver.find_all_words(&board)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_all_words() {
        // Load the dictionary.
        load_dictionary("words.txt".to_string());

        let board = vec![
            vec!["o".to_string(), "e".to_string(), "t".to_string(), "w".to_string()],
            vec!["i".to_string(), "a".to_string(), "r".to_string(), "s".to_string()],
            vec!["y".to_string(), "t".to_string(), "o".to_string(), "p".to_string()],
            vec!["r".to_string(), "w".to_string(), "b".to_string(), "s".to_string()]
        ];

        // Find all words on the board.
        let words = solve_words(board);

        // Log all words.
        println!("Found {} words:", words.len());
        for word in &words {
            println!("{}", word);
        }

        // Check if the words are correct.
        assert!(words.contains(&"ear".to_string()));
        assert!(words.contains(&"ears".to_string()));
        assert!(words.contains(&"tar".to_string()));
        assert!(words.contains(&"tars".to_string()));
        assert!(words.contains(&"tear".to_string()));
        assert!(words.contains(&"tears".to_string()));
        assert!(words.contains(&"top".to_string()));
        assert!(words.contains(&"tops".to_string()));
    }
}

/// Loads all Scrabble! words.
/// env: The JNI environment.
/// class: The Java class calling this method.
/// dictionary_path: The path to the dictionary.
#[no_mangle]
pub extern "system" fn Java_moe_seikimo_magixbot_MagixBot_loadWords(
    mut env: JNIEnv, _class: JClass,
    dictionary_path: JString
) {
    // Read the dictionary path.
    let dictionary_path: String = env.get_string(&dictionary_path)
        .expect("Couldn't get the dictionary path.")
        .into();

    // Load the dictionary.
    load_dictionary(dictionary_path);
}

/// Native method to find all valid Scrabble! words in a 2D board.
/// Requires the dictionary to be initialized.
/// env: The JNI environment.
/// class: The Java class calling this method.
/// board: A 2D array of characters.
#[no_mangle]
pub extern "system" fn Java_moe_seikimo_magixbot_features_game_type_WordHunt_findWords(
    mut env: JNIEnv, _class: JClass,
    java_board: JObjectArray
) -> jobjectArray {
    // Convert Java 2D array to Rust Vec<Vec<String>>
    let rows = env.get_array_length(&java_board).unwrap() as usize;
    let mut board = Vec::with_capacity(rows);

    for i in 0..rows {
        let row = env.get_object_array_element(&java_board, i as jint)
            .expect("Failed to get row");
        let row_array = JObjectArray::from(row);
        let cols = env.get_array_length(&row_array).unwrap() as usize;
        let mut row_vec = Vec::with_capacity(cols);

        for j in 0..cols {
            let cell = env.get_object_array_element(&row_array, j as jint)
                .expect("Failed to get cell");
            let cell_str = env.get_string(&JString::from(cell))
                .expect("Failed to get string")
                .into();
            row_vec.push(cell_str);
        }

        board.push(row_vec);
    }

    // Create WordHunt instance and find words
    let words = solve_words(board);

    // Convert result back to Java String array
    let string_class = env.find_class("java/lang/String")
        .expect("Failed to find String class");
    let result_array = env.new_object_array(
        words.len() as jint,
        string_class,
        JString::default()
    ).expect("Failed to create result array");

    for (i, word) in words.iter().enumerate() {
        let j_string = env.new_string(word)
            .expect("Failed to create Java string");
        env.set_object_array_element(
            &result_array,
            i as jint,
            j_string
        ).expect("Failed to set array element");
    }

    result_array.into_raw()
}