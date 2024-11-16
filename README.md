# Imputator

Simple Rust program to impute work in Tempo

## How it works

The program read the CSV files in the `csv` folder, iterates through the imputations, and in each one

1. Log in the user
2. Get the internal Jira ID of the user
3. Get the internal Jira issue ID of the issue
4. Store both IDs in a database to cache them and not request them again
5. Record the imputation

## Usage

1. Download [RustRover](https://www.jetbrains.com/rust/)
2. Clone this repository
3. Open the project in RustRover
4. Install Rust in File > Settings > Rust > Install
5. Run `cargo build` to install dependencies
6. Edit the CSV files on the `csv` folder to add your imputations and credentials
7. Edit the `src/main.rs` file to change the domain of your company
8. Run `cargo run` or push the play button on the top right corner