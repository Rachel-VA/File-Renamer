
# File Renamer App

## Description

The File Renamer app allows users to effortlessly rename multiple files in a directory based on specific patterns or criteria. By providing a folder path, a pattern to match, and a new name or pattern, users can mass-rename files and images, ensuring they return with a numerical order.

## Features

- **Colorful UI**: Enhanced with `ansi_term` for color-coded outputs, guiding users through each step.
- **Regex Support**: Uses the `regex` crate to provide powerful pattern matching capabilities for file renaming.
- **File System Interaction**: Leverages the `std::fs` and `std::path::PathBuf` libraries for seamless file and directory handling.

## Usage

1. **Introduction**: The app welcomes you with a brief introduction and a clear set of instructions.
2. **Directory Path**: Enter the path to the folder containing the files you want to rename.
3. **Pattern Matching**: Input the regular expression pattern to match the section of filenames you wish to modify.
4. **Rename**: Provide the new name or regular expression pattern for renaming the files.
5. **Looping**: After renaming, you can choose to continue renaming files in other directories or exit the app.

## Getting Started

To run the app:

```bash
cargo run -- "<path_to_directory>" "<regex_pattern>" "<new_name>"
