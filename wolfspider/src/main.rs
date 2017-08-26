extern crate regex;

// wolfspidertag use-env
use std::env;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Read;
use std::io::Write;
use regex::Regex;
use std::collections::HashMap;

struct Chunk {
    tag: String,
    content: String,
}

struct WFSFileCache {
    file_chunks: HashMap<String, Vec<Chunk>>,
}

impl WFSFileCache {
    fn new() -> WFSFileCache {
        WFSFileCache{file_chunks: HashMap::new()}
    }

    fn load(&mut self, filename: &str) -> &Vec<Chunk> {
        let mut file = File::open(filename).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let re = Regex::new(r"\[([^\[\]]+)\].*").unwrap();
        let mut chunks = Vec::new();
        for cap in re.captures_iter(content.as_ref()) {
            chunks.push(Chunk{tag: cap[1].to_string(), content: cap[2].to_string()});
        }
        self.file_chunks.insert(filename.to_string(), chunks);
        self.file_chunks.get(filename).unwrap()
    }

    fn get(&mut self, filename: &str) -> String {
        let chunks = if self.file_chunks.contains_key(filename) {
            self.file_chunks.get(filename).unwrap()
        } else  {
            self.load(filename)
        };
        let mut whole_file = String::new();
        for ch in chunks {
            whole_file.push_str(&ch.content);
        }
        whole_file
    }

    fn get_tagged(&mut self, filename: &str, tag: &str) -> String {
        let chunks = if self.file_chunks.contains_key(filename) {
            self.file_chunks.get(filename).unwrap()
        } else  {
            self.load(filename)
        };
        for ch in chunks {
            if ch.tag == tag {
                return ch.content.clone();
            }
        }
        panic!("Couldn't find tag.")
    }
}

// wolfspidertag build-order-struct
enum Action {
    WholeFile { file: String },
    Tag { file: String, tag: String },
}

struct BuildOrder {
    actions: Vec<Action>,
}

#[derive(Debug)]
enum BuildError {
    IoError(io::Error),
}

impl From<io::Error> for BuildError {
    fn from(error: io::Error) -> Self {
        BuildError::IoError(error)
    }
}

impl BuildOrder {
    fn build(&self, outfile: &str) -> Result<(), BuildError> {
        let mut outfile = File::create(outfile)?;
        let mut read_files = WFSFileCache::new();
        for action in &self.actions {
            match action {
                &Action::WholeFile { ref file } => {outfile.write(read_files.get(file).as_bytes())?;},
                &Action::Tag { ref file, ref tag } => {outfile.write(read_files.get_tagged(file, tag).as_bytes())?;},
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
enum ParseError {
    IoError(io::Error),
}

impl From<io::Error> for ParseError {
    fn from(error: io::Error) -> Self {
        ParseError::IoError(error)
    }
}

// wolfspidertag parse-bookfile
fn parse_bookfile(bookfile: &str) -> Result<BuildOrder, ParseError> {
    let mut actions: Vec<Action> = Vec::new();

    // String constant so regex always compiles.
    let tag_re = Regex::new(r"([^:]+):([^:]+)").unwrap();

    let file = File::open(bookfile)?;
    let file = BufReader::new(&file);

    for line in file.lines() {
        let line = line?;
        if tag_re.is_match(line.as_ref()) {
            // We checked is_match() so always unwraps.
            let cap = tag_re.captures(line.as_ref()).unwrap();
            actions.push(Action::Tag {
                file: cap[1].to_string(),
                tag: cap[2].to_string(),
            });
        } else {
            actions.push(Action::WholeFile { file: line });
        }
    }
    return Ok(BuildOrder { actions: actions });
}

// wolfspidertag main
fn main() {
    // wolfspidertag arg-parsing
    let args: Vec<String> = env::args().collect();

    let mut bookfile = "./Bookfile";
    let mut output = "./out.md";

    let mut i = 0;
    while i < args.len() {
        match args[i].as_ref() {
            "-o" => {
                output = args[i + 1].as_ref();
                i += 1
            }
            "-b" => {
                bookfile = args[i + 1].as_ref();
                i += 1
            }
            _ => (),
        }
        i += 1;
    }

    let build_order = match parse_bookfile(bookfile) {
        Ok(build_order) => build_order,
        Err(parse_error) => {
            match parse_error {
                ParseError::IoError(err) => println!("Error while parsing: {}", err),
            };
            return;
        }
    };

    match build_order.build(output) {
        Ok(()) => (),
        Err(build_error) => {
            match build_error {
                BuildError::IoError(err) => println!("Error while building: {}", err),
            };
            return;
        }
    };

    println!("All done.");
}
