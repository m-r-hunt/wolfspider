extern crate regex;

// wolfspidertag use-env
use std::env;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;

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
        let outfile = File::create(outfile)?;
        for action in &self.actions {
            match action {
                &Action::WholeFile { ref file } => println!("{}", file),
                &Action::Tag { ref file, ref tag } => println!("{} tag {}", file, tag),
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
