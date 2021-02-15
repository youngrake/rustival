use std::env::args;

use ir::*;

fn main() {
    let args: Vec<_> = args().collect();

    if args.contains(&"perm".to_string()) {
        process_permindex()
    } else if args.contains(&"revtree".to_string()) {
        process_reversedtree()
    }
}

fn process_permindex() {
    let files = Fb2Parser::read_dir("__examples__");

    let mut perm = PermutationIndex::new();
    perm.process_files(&files);

    loop {
        let user_input = get_user_input("Search: ");
        println!("{}", user_input);
        let result = perm.search(user_input);
        println!("{:?}", result);
    }
}

fn process_reversedtree() {
    let files = Fb2Parser::read_dir("__examples__");

    let mut tree = ReversedTree::new();

    tree.process_files(&files);

    loop {
        let user_input = get_user_input("Search: ");
        let result = tree.find_word(&user_input);
        println!("{:?}", result.unwrap_or(vec![]));
    }
}

fn get_user_input(message: &str) -> String {
    use std::io;

    println!("{}", message);

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => {}
    }

    input.trim().to_string()
}
