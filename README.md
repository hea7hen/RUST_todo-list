# Simple To-Do List Application (Rust)

This is a simple command-line to-do list application written in Rust. It allows you to add tasks, mark them as completed, remove them, and persist the data between program runs using a JSON file.

## Prerequisites

* Rust installed (using `rustup` is highly recommended). You can install it from [rustup.rs](https://rustup.rs/).

## How to Run

1.  **Clone or create the project:**
    * If you have the code in a repository, clone it:
        ```bash
        git clone <repository_url>
        cd todo
        ```
    * If you created the files manually, ensure you have the following file structure:

        ```
        todo/
        ├── Cargo.toml
        └── src/
            └── main.rs
        ```

2.  **Add Dependencies:**
    * Make sure your `Cargo.toml` file contains the following dependencies:

        ```toml
        [package]
        name = "todo"
        version = "0.1.0"
        edition = "2021"

        [dependencies]
        serde = { version = "1.0", features = ["derive"] }
        serde_json = "1.0"
        ```

3.  **Run the Application:**
    * Open your terminal in the `todo` directory.
    * Run the following command:

        ```bash
        cargo run
        ```

4.  **Use the Application:**
    * The application will display a menu with the following options:
        * Add task
        * Mark task as completed
        * Remove task
        * Exit
    * Follow the on-screen instructions to interact with the application.
    * The to-do list data will be saved in a file named `tasks.json` in the project directory.

## File Structure

* `Cargo.toml`: Contains the project's metadata and dependencies.
* `src/main.rs`: Contains the Rust source code for the application.
* `tasks.json`: (Created by the application) Stores the to-do list data in JSON format.
