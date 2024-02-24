# What is this?
- This repository is a small and easy project to learn Rust syntax.
- This has a ability to get schemes from Flutter project.

## Run Code
`get-schemes --path <YOUR-FLUTTER-PROJECT-PATH>`

## Input
- the result of `xcodebuild -list`
- the result of `./gradlew`

### Output
- schemes from input data


## Syntax
### How can I run command?
- `Command::new("xcodebuild")`

### How can I convert Vec to String?
- `String::from_utf8(vec)`

### How can I make struct?
```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

### How can I make for loop?
```
    for i in 0..3 {
        println!("> {}", i);
    }
```

### How can I import internal crate?
```
## main.rs
mod cli;
use cli::Args;
```

### How can I import external crate?
```
## Cargo.toml
[dependencies]
clap = { version = "4.5.0", features = ["derive"] }

## main.rs
use clap::Parser;
```
- "features" mean feature flag
