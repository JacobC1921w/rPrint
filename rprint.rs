pub fn p(message: &str, mode: char, verbose: bool, prefix: &str, suffix: &str) {
    if mode == 'v' && verbose {
        print!("{}\x1b[0m[\x1b[90;1m#\x1b[0m] \x1b[90;2m{}\x1b[0m{}", prefix, message, suffix);
    } else if mode == 'e' {
        print!("{}\x1b[0m[\x1b[91;1m-\x1b[0m] \x1b[90;2m{}\x1b[0m{}", prefix, message, suffix);
    } else if mode == 's' {
        print!("{}\x1b[0m[\x1b[92;1m+\x1b[0m] \x1b[90;2m{}\x1b[0m{}", prefix, message, suffix);
    } else if mode == 'w' {
        print!("{}\x1b[0m[\x1b[93;1m*\x1b[0m] \x1b[90;2m{}\x1b[0m{}", prefix, message, suffix);
    } else {
        print!("{}\x1b[0m[\x1b[94;1m!\x1b[0m] \x1b[90;2m{}\x1b[0m{}", prefix, message, suffix);
    }
}
