use std::fs::*;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let file_path = "files\\mocked_passwords.txt".to_string();
    
    let contents = read_to_string(file_path)?;
    let collection: Vec<&str> = contents
        .split("---")
        .filter(|x| x.contains("Website"))
        .collect();

    let mut csv_file = File::create("files\\converted.csv")?;
    for password in collection {
        let split_by_enter: Vec<&str> = password
            .split("\r\n")
            .filter(|line| line.contains(':'))
            .collect();

        for line in split_by_enter {
            // can't use .split(":"), because links have "://"
            let index = line.chars().position(|c| c == ':').unwrap() + 1;

            // replace " with "" for escape the double quote in .csv
            let value = format!("\"{}\",", line[index..].replace("\"", "\"\"").trim().to_string());
            
            csv_file.write_all(value.as_bytes())?;
        }

        csv_file.write_all(b"\n")?;
        csv_file.flush()?;
    }

    Ok(())
}
