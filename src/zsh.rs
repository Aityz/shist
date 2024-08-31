use std::io::BufRead;

pub fn read_history() -> Vec<String> {
    let zsh_history =
        std::path::PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(".zsh_history");

    if zsh_history.exists() {
        let file = std::fs::File::open(zsh_history);

        if let Ok(f) = file {
            let bf = std::io::BufReader::new(f);

            let mut history: Vec<String> = Vec::new();

            for line in bf.lines() {
                let line = line.unwrap_or("0:0;".to_string());

                let mut split = line.split(';').collect::<Vec<&str>>();

                split.remove(0);

                history.push(split.join(";"));
            }

            history
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    }
}
