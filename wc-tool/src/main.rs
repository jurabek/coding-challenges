use std::fmt;
// use std::fmt::format;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Read;
// use std::io::Read;
use std::io::Write;
use std::path::Path;

use clap::Arg;
use clap::ArgAction;
use clap::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let m = Command::new("wc-tool")
        .author("Jurabek")
        .version("0.0.1")
        .about("Coding challenge for unix WC tool")
        .arg(
            Arg::new("count")
                .long("count")
                .short('c')
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("chars")
                .long("chars")
                .short('m')
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("lines")
                .long("lines")
                .short('l')
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("words")
                .long("words")
                .short('w')
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("file")
                .index(1)
                .value_name("FILE")
                .help("Files to process"),
        )
        .after_help("Coding challenge building unix WC tool in Rust")
        .get_matches();

    let display_bytes = m.get_flag("count");
    let display_lines = m.get_flag("lines");
    let display_words = m.get_flag("words");
    let display_chars = m.get_flag("chars");

    let no_flags_set = ![display_bytes, display_lines, display_words, display_chars]
        .iter()
        .any(|&x| x);

    let mut stats = Stats::new(
        display_lines || no_flags_set,
        display_words || no_flags_set,
        display_bytes || no_flags_set,
        display_chars || no_flags_set,
    );

    match m.get_one::<String>("file") {
        Some(file_name) => {
            let path: &Path = Path::new(file_name);
            let file = File::open(&path)?;
            let metadata = file.metadata()?;
            stats.bytes = metadata.len();
            stats.file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            let reader = io::BufReader::new(file);
            stats.update_from_reader(reader);
        }
        None => {
            let stdin = io::stdin();
            let reader = stdin.lock();
            stats.bytes = stdin.bytes().size_hint().0 as u64;
            stats.update_from_reader(reader);
        }
    }
    println!("{}", stats);
    return Ok(());
}

struct Stats {
    lines: u64,
    words: u64,
    bytes: u64,
    chars: u64,
    file_name: String,
    display_lines: bool,
    display_words: bool,
    display_bytes: bool,
    display_chars: bool,
}

impl Stats {
    fn new(
        display_lines: bool,
        display_words: bool,
        display_bytes: bool,
        display_chars: bool,
    ) -> Stats {
        Stats {
            lines: 0,
            words: 0,
            bytes: 0,
            chars: 0,
            file_name: String::new(),
            display_lines,
            display_words,
            display_bytes,
            display_chars,
        }
    }

    fn update(&mut self, line: &str) {
        self.words += line.split_whitespace().count() as u64;
        self.lines += 1;
        self.chars += line.chars().count() as u64;
    }

    fn update_from_reader<R: BufRead>(&mut self, reader: R) {
        for line in reader.lines() {
            let line = line.expect("Error reading line");
            self.update(&line);
        }
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.display_bytes {
            write!(f, "{} ", self.bytes)?;
        }
        if self.display_lines {
            write!(f, "{} ", self.lines)?;
        }
        if self.display_words {
            write!(f, "{} ", self.words)?;
        }
        if self.display_chars {
            write!(f, "{} ", self.chars)?;
        }
        write!(f, "{}", self.file_name)
    }
}
