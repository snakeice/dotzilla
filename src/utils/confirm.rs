use std::io::{self, Write};

pub fn confirm(question: &str, default: Option<bool>) -> bool {
    loop {
        let prompt = match default {
            Some(true) => format!("{} [Y/n]: ", question),
            Some(false) => format!("{} [y/N]: ", question),
            None => format!("{} [y/n]: ", question),
        };
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input. Please try again.");
            continue;
        }
        let trimmed = input.trim().to_lowercase();
        if trimmed.is_empty() {
            if let Some(def) = default {
                return def;
            } else {
                println!("Please enter 'y' or 'n'.");
                continue;
            }
        }
        match trimmed.as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please enter 'y' or 'n'."),
        }
    }
}
