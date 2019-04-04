use std::env;

fn parse_range(s: &str) -> Option<String> {
    let first = s.chars().take_while(|c| c.is_digit(10)).count();
    if first == 0 {
        return None;
    }

    if !s[first..].starts_with("..") {
        return None;
    }

    if let (Ok(n0), Ok(n1)) = (s[..first].parse::<u32>(), s[first + 2..].parse::<u32>()) {
        Some((n0..n1).filter_map(std::char::from_u32).collect())
    } else {
        return None;
    }
}

fn main() {
    if env::args().count() == 1 {
        eprintln!(concat!(
            env!("CARGO_PKG_NAME"),
            " v",
            env!("CARGO_PKG_VERSION"),
            "\n",
            env!("CARGO_PKG_AUTHORS"),
            "\n",
            env!("CARGO_PKG_DESCRIPTION"),
            "\n\n",
            "USAGE: uchr (<string>|<integer>|<closed integer range>)..."
        ));
    }

    for arg in env::args().skip(1) {
        if let Ok(n) = arg.parse::<u32>() {
            if let Some(c) = std::char::from_u32(n) {
                println!("{}", c);
                continue;
            }
        }

        if arg.contains("..") {
            if let Some(v) = parse_range(&arg) {
                println!("{}", v);
                continue;
            }
        }

        for c in arg.chars() {
            println!("{}", c as u32);
        }
    }
}
