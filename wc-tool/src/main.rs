use std::fmt::Display;
// use std::fmt::format;
use std::fs::File;
use std::io;
use std::io::BufRead;
// use std::io::Read;
use std::io::Write;
use std::path::Path;

use clap::Arg;
use clap::ArgAction;
use clap::Command;

fn main() {
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
                .help("Files to process")
                .default_value("test.txt"),
        )
        .after_help("Coding challenge building unix WC tool in Rust")
        .get_matches();

    let path = Path::new(m.get_one::<String>("file").unwrap());
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
    };

    let mut stats: Stats = Stats {
        lines: 0,
        words: 0,
        bytes: 0,
        chars: 0,
        file_name: path.file_name().unwrap().to_str().unwrap().to_string(),
    };

    if m.get_flag("count") {
        stats.bytes = file.metadata().unwrap().len();
    }

    let (lines_count, word_count, number_of_chars) = file_lines_and_words_count(file);

    if m.get_flag("lines") {
        stats.lines = lines_count;
    }
    if m.get_flag("words") {
        stats.words = word_count;
    }
    if m.get_flag("chars") {
        stats.chars = number_of_chars;
    }
    
    match io::stdout().write_all(format!("{}", stats).as_bytes()) {
        Ok(_) => {}
        Err(why) => panic!("couldn't write to stdout: {}", why),
    }
}

fn file_lines_and_words_count(file: File) -> (u64, u64, u64) {
    let mut lines_count: u64 = 0;
    let mut word_count: usize = 0;
    let mut number_of_chars = 0;

    for line in io::BufReader::new(file).lines() {
        match line {
            Ok(l) => {
                word_count += l.split_whitespace().count();
                lines_count += 1;
                number_of_chars += l.chars().count();
            }
            Err(why) => panic!("couldn't read line: {}", why),
        }
    }
    (lines_count, word_count as u64, number_of_chars as u64)
}

struct Stats {
    lines: u64,
    words: u64,
    bytes: u64,
    chars: u64,
    file_name: String,
}

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}    {}  {}  {}", self.lines, self.words, self.bytes, self.file_name) 
    }
}
