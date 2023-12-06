use std::{fs, io};
pub fn read_file_by_line(file_path: &str) -> Result<Vec<String>, io::Error> {
    println!("Reading file: {}", file_path);
    let contents = fs::read_to_string(file_path);

    match contents {
        Ok(contents) => {
            let lines = contents.lines().map(|s| s.to_string()).collect();
            Ok(lines)
        }
        Err(e) => {
            println!("Error reading file: {}", e);
            Err(e)
        }
    }
}
