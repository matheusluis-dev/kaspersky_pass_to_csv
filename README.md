# Kaspersky Pass to .CSV
`made for practicing rust`

## Purpose
This code converts a text file containing passwords into a CSV file. The CSV file is created in the files directory and contains the website name and password for each entry.

It reads the contents of the text file using `read_to_string`. It then splits the contents by `---` and filters out any entries that do not contain the word "Website". The resulting collection is then iterated over, and each entry is split by `\r\n (enter)` and filtered to only include lines that contain `:`. The resulting lines are then iterated over, and the password is extracted by finding the index of the `:` character and trimming the resulting string. The password is then written to the CSV file using `write_all`.

## How to Use
- Clone the repository.
- Place the text file containing passwords in the files directory.
- Run the code using `cargo run`.
- The resulting CSV file will be created in the files directory.
- Import into another Password Manager
