/// Main file to translate markdown into HTML.
use std::fs::File;
use std::io::Write;

mod tokenizer;

fn compile_pp_file(filename: &str) {
    print_title();
    println!("[ INFO ] Trying to open {}...", filename);
    let mut tokenizer = tokenizer::Tokenizer::new(filename);

    println!("[ INFO ] Compiling {}...", filename);
    let statement = tokenizer.tokenize_next_statement();
    println!("TOKENS:");
    for token in statement {
        println!("{}", token);
    }

    let mut output_filename = String::from(&filename[..filename.len()-2]);
    output_filename.push_str("js");

    println!("[ INFO ] Successfully compiled to {}!", output_filename);
}

fn write_to_file(output_filename: String, lines: Vec<String>) {
    let mut outfile = File::create(&output_filename)
        .expect(&format!("[ ERROR ] Could not create output file {}!", &output_filename));
    
    for line in &lines {
        outfile.write_all(line.as_bytes())
               .expect("[ ERROR ] Could not write to output file!");
    }
}

fn print_long_info() {
    print_title();
    println!("Written by: {}", env!("CARGO_PKG_AUTHORS"));
    println!("Homepage: {}", env!("CARGO_PKG_HOMEPAGE"));
    println!("Usage: pp [option] [ source.pp ] [args]");
}

fn print_title() {
    println!("{} (v{}), {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"),
                             env!("CARGO_PKG_DESCRIPTION"));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => compile_pp_file(&args[1]),
        _ => print_long_info()
    }
}
