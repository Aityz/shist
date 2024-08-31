use clap::Parser;
use std::{collections::HashMap, io::Write};

mod args;

mod bash;
mod zsh;

fn main() {
    let arguments = args::Arguments::parse();

    if arguments.zsh && arguments.bash {
        println!("Recieved both bash and zsh arguments, aborting");

        return;
    }

    let mut history: Vec<String> = Vec::new();

    if !arguments.zsh {
        history.extend(bash::read_history());
    }

    if !arguments.bash {
        history.extend(zsh::read_history());
    }

    // we now index the history using a HashMap

    let mut index: HashMap<String, u64> = HashMap::new();

    for hist in &history {
        let hist: String = hist.split(' ').next().unwrap_or_default().to_string();

        if !index.contains_key(&hist) {
            index.insert(hist, 1);
        } else {
            index.insert(hist.clone(), index.get(&hist).unwrap_or(&0) + 1);
        }
    }

    // we calculate the three most used commands

    let mut vals: Vec<(&String, &u64)> = index.iter().collect();

    vals.sort_by(|a, b| b.1.cmp(a.1));

    let mut printed_strings = Vec::new();

    let num_commands = arguments.num_commands.unwrap_or(5);

    let mut i = 0;

    while let Some((k, v)) = vals.get(i) {
        if i < num_commands.into() {
            printed_strings.push(format!("You have used {} {} times", k, v));
        }

        i += 1;
    }

    println!("🚀 You have used the shell {} times!", history.len());
    println!("🎉 You have used {} unique commands!\n", index.keys().len());

    println!("Your Top Commands:");

    for str in printed_strings {
        println!("{}", str);
    }

    println!();

    if arguments.export {
        let file = std::fs::File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open("shist_history.csv");

        if let Ok(mut f) = file {
            match f.write_all("command,frequency\n".as_bytes()) {
                Ok(_) => {
                    for value in &vals {
                        if !value.0.is_empty() {
                            if f.write_all(format!("{},{}\n", value.0, value.1).as_bytes())
                                .is_err()
                            {
                                println!("Failed to write to file")
                            };
                        }
                    }
                }
                Err(_) => println!("Failed to write to file"),
            }
        } else {
            println!("Failed to write to file");
        }
    }
}
