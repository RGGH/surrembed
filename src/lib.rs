use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    pub name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    pub count: u8,
}

#[cfg(test)]
mod tests {
    use super::Args;
    use clap::Parser;

    #[test]
    fn test_args_default_count() {
        let args = Args::parse_from(&["test", "--name", "Alice"]);
        assert_eq!(args.name, "Alice");
        assert_eq!(args.count, 1);  // Default count should be 1
    }

    #[test]
    fn test_args_with_count() {
        let args = Args::parse_from(&["test", "--name", "Bob", "--count", "3"]);
        assert_eq!(args.name, "Bob");
        assert_eq!(args.count, 3);
    }

    #[test]
    fn test_args_short_flags() {
        let args = Args::parse_from(&["test", "-n", "Carol", "-c", "2"]);
        assert_eq!(args.name, "Carol");
        assert_eq!(args.count, 2);
    }
}
