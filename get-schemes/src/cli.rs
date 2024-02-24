use clap::Parser;

/// Simple program to get scheme using gradlew and xcodebuild
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to your Flutter project
    #[arg(short, long)]
    pub path: String,
}