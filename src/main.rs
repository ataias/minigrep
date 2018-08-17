use std::env;

fn main() {
    // collect gets an iterator and turns it into a collection
    // it needs to be annotated
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
    println!("{:?}", args);
}
