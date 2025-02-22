use std::io::{self, Read};
use std::{fs, path::Path, process};

#[macro_use]
extern crate clap;
use clap::{App, Arg};

use sudoku::Grid;

fn print_err(err: impl std::fmt::Display) -> ! {
    eprintln!("{}", err);
    process::exit(1);
}

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .arg(
            Arg::with_name("input")
                .required(true)
                .index(1)
                .help("Path to input file.")
                .long_help(
                    "Path to input file. A `-` can be used to read from stdin, e.g. when combining with other programs.",
                )
        )
        .arg(
            Arg::with_name("output")
                .index(2)
                .help("Path to output file")
                .long_help("Path to output file. Can be useful when using verbose mode to prevent redirecting unecessary input to output file.")
        )
        .arg(Arg::with_name("verbose").short("v").long("verbose"))
        .get_matches();

    let is_verbose = matches.is_present("verbose");

    if is_verbose {
        println!("Reading grid from {}", matches.value_of("input").unwrap_or("stdin"));
    }

    let content = match matches.value_of("input").unwrap() {
        "-" => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .expect("Error reading from stdin");
            buffer
        }
        p => {
            if !Path::exists(Path::new(p)) {
                print_err("Input path does not exist");
            }

            fs::read_to_string(p).expect("Error reading sudoku grid!")
        }
    };

    if is_verbose {
        println!("Done reading!");
    }

    let mut grid = Grid::parse(content.as_str()).unwrap_or_else(|e| print_err(e));

    if is_verbose {
        println!("Parsed grid.");
    }

    grid.solve().unwrap_or_else(|e| print_err(e));

    if is_verbose {
        println!("Solved!");
    }

    if let Some(p) = matches.value_of("output") {
        fs::write(p, grid.to_string()).expect("Error writing to output");
    }

    println!("{}", grid);
}
