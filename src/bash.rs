use std::io::BufRead as _;

pub fn read_history() -> Vec<String> {
    let bash_history =
        std::path::PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(".bash_history");

    if bash_history.exists() {
        let file = std::fs::File::open(bash_history);

        if let Ok(f) = file {
            let bf = std::io::BufReader::new(f);

            let mut history: Vec<String> = Vec::new();

            for line in bf.lines() {
                history.push(line.unwrap_or_default().trim().to_string());
            }

            history
        } else {
            return Vec::new();
        }
    } else {
        Vec::new()
    }
}
