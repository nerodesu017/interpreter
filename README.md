# Lox in Rust

## Context

This project is done following the guide at https://craftinginterpreters.com/ but written in rust

## Project structure

  - main.rs     - entrypoint - here we decide if usage is correct and if we are running a file or doing REPL - the source file is transferred to a Lox class instance
  - lox.rs      - the global Lox class that should be a singleton since it's one instance per run - here is where we give a green light to the runner. We also implement the ScannerError trait here so that we can report errors
  - scanner.rs  - the scanner of the language - here
  - token.rs    - the implementation of the Token and TokenType, also the implementation of the Display trait for them both
  - literal.rs  - implementation of the Literal enum and the Display trait for it
  - error.rs    - definition of the ScannerError trait to be implemented by the Lox struct