use std::collections::HashSet;
use crate::DICTIONARY;
use crate::trie_node::TrieNode;

/// All valid directions for locating adjacent characters.
const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),           (0, 1),
    (1, -1),  (1, 0),  (1, 1),
];

pub struct Solver {
    word_trie: TrieNode
}

impl Solver {
    /// Creates a new solver instance.
    /// Resolves words from the global dictionary.
    pub fn new() -> Self {
        let dictionary = DICTIONARY.read().unwrap();
        Self { word_trie: dictionary.clone() }
    }

    /// Finds all valid words in a 2D board.
    /// board: The game board.
    pub fn find_all_words(&self, board: &[Vec<String>]) -> Vec<String> {
        let mut words = HashSet::new();
        let rows = board.len();
        let cols = board[0].len();
        let mut visited = vec![vec![false; cols]; rows];
        let mut current_word = String::new();

        for row in 0..rows {
            for col in 0..cols {
                self.visit(
                    board,
                    row,
                    col,
                    &mut visited,
                    &mut current_word,
                    &mut words
                );
            }
        }

        let mut result: Vec<String> = words.into_iter()
            .filter(|word| word.len() >= 3)
            .collect();

        result.sort_by(|a, b| {
            b.len().cmp(&a.len()).then(a.cmp(b))
        });

        result
    }

    /// Visits a position on the game board.
    /// board: The game board.
    /// row: The row index.
    /// col: The column index.
    /// visited: The visited positions.
    /// current_word: The current word.
    /// words: The set of valid words.
    fn visit(
        &self,
        board: &[Vec<String>],
        row: usize,
        col: usize,
        visited: &mut Vec<Vec<bool>>,
        current_word: &mut String,
        words: &mut HashSet<String>
    ) {
        if !self.in_bounds(board, row, col) || visited[row][col] {
            return;
        }

        visited[row][col] = true;
        current_word.push_str(&board[row][col]);

        if self.word_trie.has_prefix(current_word) {
            if self.word_trie.is_word(current_word) {
                words.insert(current_word.clone());
            }

            for &(dx, dy) in &DIRECTIONS {
                let new_row = row as i32 + dx;
                let new_col = col as i32 + dy;

                if new_row >= 0 && new_col >= 0 {
                    self.visit(
                        board,
                        new_row as usize,
                        new_col as usize,
                        visited,
                        current_word,
                        words
                    );
                }
            }
        }

        visited[row][col] = false;
        current_word.truncate(current_word.len() - board[row][col].len());
    }

    /// Checks if a position is within the boundaries of a game board.
    /// board: The game board.
    /// row: The row index.
    /// col: The column index.
    fn in_bounds(&self, board: &[Vec<String>], row: usize, col: usize) -> bool {
        row < board.len() && col < board[0].len()
    }
}