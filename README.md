# Hangman (Rust)

A letter-guessing word game written in Rust. Guess the hidden word one
letter at a time — 6 wrong guesses and the hangman is complete.

## Play

```bash
cargo run --release
# or run the compiled binary:
./hangman.exe
```

Windows binary in **Releases**.

## Features

- ~160-word built-in vocabulary
- ASCII hangman gallows that fills in as you miss
- Live word display (`d _ _ c _`), guessed-letter tracking
- Color output (ANSI), replay loop, final score
- Pure std — zero dependencies

## Build

```bash
cargo build --release --target x86_64-pc-windows-gnu
# exe at target/x86_64-pc-windows-gnu/release/hangman.exe
```

MIT licensed.
