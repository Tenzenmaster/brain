use brain::Program;

fn main() {
    let mut args = std::env::args();

    let source = args.nth(1)
        .expect("Please provide a source file");
    let input = args.next().unwrap_or("".to_owned());

    Program::new(&std::fs::read_to_string(&source).unwrap(), &input).run();
}

