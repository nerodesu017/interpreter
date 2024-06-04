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

  - expr.rs     - defines the `Expr` enum and exports all the expressions (which are in turn generated using [nerodesu017/interpreter-code-gen](https://github.com/nerodesu017/interpreter-code-gen))
  - visitor.rs  - this contains the basic Visitor template needed for creating all sorts of Visitors based on the [Visitor Pattern](https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html) - sadly, I could not fully decide on if I should use the Java-like Visitor Pattern, which uses Polymorphic Dispatching, or use the FP way and make use of `match`, thus, even though the current logic uses `match`, the `Visitor` trait is there, besides not being used - part of it generated using [nerodesu017/interpreter-code-gen](https://github.com/nerodesu017/interpreter-code-gen)