// wolfspidertag use-env
use std::env;

// wolfspidertag build-order-struct
struct BuildOrder {

}

impl BuildOrder {
    fn build(&self) {

    }
}

// wolfspidertag parse-bookfile
fn parse_bookfile(bookfile: &str) -> BuildOrder {
    return BuildOrder{};
}

// wolfspidertag main
fn main() {
    //wolfspidertag arg-parsing
    let args: Vec<String> = env::args().collect();

    let mut bookfile = "./Bookfile";
    let mut output = "./out.md";

    let mut i = 0;
    while i < args.len() {
        match args[i].as_ref() {
            "-o" => {output = args[i+1].as_ref(); i += 1},
            "-b" => {bookfile = args[i+1].as_ref(); i += 1},
            _ => (),
        }
        i = i + 1;
    }

    let build_order = parse_bookfile(bookfile);
    build_order.build();

    println!("Hello, world!");
}
