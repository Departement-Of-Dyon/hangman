use std::collections::HashSet;
use std::io::{self, Write};

const WORDS: &[&str] = &[
    "apple", "beach", "brain", "chair", "cloud", "dance", "eagle", "flame",
    "grape", "heart", "index", "judge", "knife", "lemon", "magic", "noble",
    "ocean", "peace", "queen", "river", "stone", "tiger", "umbra", "vivid",
    "whale", "youth", "zebra", "brave", "crane", "dream", "earth", "frost",
    "ghost", "hotel", "ivory", "joker", "karma", "light", "metal", "noble",
    "onion", "piano", "quill", "robin", "solar", "tulip", "ultra", "velvet",
    "wagon", "xenon", "yield", "zesty", "alarm", "baker", "camel", "delta",
    "ember", "flint", "grove", "haven", "input", "jewel", "kayak", "lunar",
    "maple", "ninja", "orbit", "pearl", "quest", "raven", "saber", "tempo",
    "unity", "vapor", "wharf", "xylem", "yearn", "zinc", "actor", "bloom",
    "coral", "dodge", "elite", "fable", "giant", "harbor", "inferno", "jaguar",
    "kernel", "lagoon", "magnet", "nectar", "onyx", "prairie", "quartz",
    "rocket", "summit", "thunder", "urgent", "vector", "willow", "xenial",
    "zephyr", "anchor", "bronze", "cipher", "dragon", "engine", "falcon",
    "garden", "helmet", "island", "jungle", "knight", "lantern", "marble",
    "nebula", "orchid", "phoenix", "quiver", "ranger", "saffron", "tundra",
    "umbriel", "vertex", "walnut", "xanadu", "yonder", "zircon",
    "absorb", "ballad", "citadel", "dynamo", "eclipse", "fractal", "gravity",
    "horizon", "icicle", "juniper", "krypton", "labyrinth", "monolith",
    "nemesis", "oblivion", "paradox", "quantum", "renaissance", "synergy",
    "tapestry", "umbra", "vortex", "warden", "xenotype", "yggdrasil", "zenith",
];

const MAX_WRONG: usize = 6;

fn pick_word() -> &'static str {
    let idx = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as usize)
        .unwrap_or(0))
        % WORDS.len();
    WORDS[idx]
}

fn display_word(word: &str, guessed: &HashSet<char>) -> String {
    word.chars()
        .map(|c| if guessed.contains(&c) { c.to_string() } else { "_".to_string() })
        .collect::<Vec<_>>()
        .join(" ")
}

fn draw_hangman(wrong: usize) {
    let stages = [
        "  +---+\n  |   |\n      |\n      |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n      |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n  |   |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|   |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|\\  |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|\\  |\n /    |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|\\  |\n / \\  |\n=========",
    ];
    println!("{}", stages[wrong.min(MAX_WRONG)]);
}

fn play_round(round: usize) -> bool {
    let word = pick_word();
    let mut guessed: HashSet<char> = HashSet::new();
    let mut wrong = 0usize;

    println!("\n\x1b[1;36m=== ROUND {} ===\x1b[0m", round);
    println!("The word has {} letters.", word.len());

    loop {
        draw_hangman(wrong);
        println!("\nWord: \x1b[33m{}\x1b[0m", display_word(word, &guessed));
        let mut used: Vec<char> = guessed.iter().cloned().collect();
        used.sort();
        if !used.is_empty() {
            println!("Guessed: {}", used.iter().collect::<String>());
        }
        println!("Wrong guesses left: {}", MAX_WRONG - wrong);

        if wrong >= MAX_WRONG {
            println!("\n\x1b[31mGame over!\x1b[0m The word was: \x1b[1;32m{}\x1b[0m", word);
            return false;
        }
        if word.chars().all(|c| guessed.contains(&c)) {
            println!("\n\x1b[1;32mYou win!\x1b[0m The word was: {}", word);
            return true;
        }

        print!("Guess a letter: ");
        io::stdout().flush().ok();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) | Err(_) => return false,   // EOF / error -> quit
            _ => {}
        }
        let ch = input.trim().chars().next().unwrap_or('\0').to_ascii_lowercase();

        if !ch.is_ascii_alphabetic() {
            println!("Please enter a single letter a-z.");
            continue;
        }
        if guessed.contains(&ch) {
            println!("You already tried '{}'.", ch);
            continue;
        }
        guessed.insert(ch);

        if !word.contains(ch) {
            wrong += 1;
            println!("\x1b[31m'{}' is not in the word.\x1b[0m", ch);
        } else {
            println!("\x1b[32m'{}' is in the word!\x1b[0m", ch);
        }
    }
}

fn main() {
    println!("\x1b[1;35m");
    println!("  _   _                      _");
    println!(" | | | |__ _ _ __ _ __ ___  | |");
    println!(" | |_| / _` | '__| '_ ` _ \\ | |");
    println!(" |  _ | (_| | |  | | | | | || |_|");
    println!(" |_| |_|\\__,_|_|  |_| |_| |_|\\___/");
    println!("\x1b[0m  Guess the word, one letter at a time. 6 wrong = hangman.");

    let mut wins = 0usize;
    let mut round = 0usize;
    loop {
        round += 1;
        if play_round(round) { wins += 1; }
        print!("\nPlay again? [y/n]: ");
        io::stdout().flush().ok();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() { break; }
        match input.trim().to_ascii_lowercase().as_str() {
            "y" | "yes" => continue,
            _ => break,
        }
    }
    println!("\nFinal score: {} win(s) in {} round(s). Good luck!", wins, round);
}
