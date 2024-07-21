#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod lexer;
mod parseTree;
mod table_setup;

use lexer::*;
use parseTree::*;
use table_setup::*;

use ansi_term::Colour;
use rand::prelude::*;
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use term_size::dimensions_stdout;

fn main() {
    let file = "ex2.c";
    let grammar = "Grammar2";
    driverFunction(
        ("source_files/".to_owned() + grammar).as_str(),
        ("example_files/".to_owned() + file).as_str(),
    );
}

fn driverFunction(rootPath: &str, file: &str) {
    // PATHS
    let terminals_path = format!("{}/term.txt", rootPath);
    let terminals_symbolic_path = format!("{}/termsymbolic.txt", rootPath);
    let non_terminals_path = format!("{}/nonterm.txt", rootPath);
    let production_rules = format!("{}/cfg.txt", rootPath);

    //Keywords
    let tsp = File::open(&terminals_symbolic_path).unwrap();
    let tsp = io::BufReader::new(tsp);
    let tsp: Vec<String> = tsp
        .lines()
        .map(|l| {
            l.expect("Could not parse line")
                .trim_start_matches('\u{feff}')
                .to_string()
        })
        .collect();

    let tp = File::open(&terminals_path).unwrap();
    let tp = io::BufReader::new(tp);
    let tp: Vec<String> = tp
        .lines()
        .map(|l| {
            l.expect("Could not parse line")
                .trim_start_matches('\u{feff}')
                .to_string()
        })
        .collect();

    let mut key: HashMap<&str, &str> = HashMap::new();
    let mut keyy: HashSet<&str> = HashSet::new();
    for (k, v) in tsp.iter().zip(tp.iter()) {
        key.insert(k, v);
        keyy.insert(k);
    }

    //Some code
    let (
        mut terminals,
        mut non_terminals,
        mut map,
        mut rules,
        mut nullables,
        mut terminal_indices,
        count,
    ) = construct(
        &terminals_path[..],
        &non_terminals_path[..],
        &production_rules[..],
    );
    let mut first = calFIRST(
        &mut terminals,
        &mut non_terminals,
        &mut map,
        &mut rules,
        &mut nullables,
        &mut terminal_indices,
    );
    let mut follow = calFollow(
        &mut terminals,
        &mut non_terminals,
        &mut map,
        &mut rules,
        &mut nullables,
        &mut terminal_indices,
        &mut first,
    );

    let (T, mut final_rules) = table_maker(
        &mut terminals,
        &mut rules,
        &mut first,
        &mut follow,
        &mut nullables,
        &mut terminal_indices,
        &mut map,
        count as i32,
    );

    let mut tokens = lexer(file, keyy, key);
    tokens.push(Token {
        val: "Eof".to_string(),
        kind: "Eof".to_string(),
    });
    // RGB values for bright blue
    let bright_blue = Colour::RGB(0, 191, 255);
    // Unicode escape sequence for smiley emoji
    // Get terminal width
    let (term_width, _) = dimensions_stdout().unwrap_or((80, 25)); // Default width: 80

    // Text content
    let text = format!("Parse Tree for the provided snippet:\n\n",);

    // Calculate padding
    let padding = cmp::max(0, (term_width as isize - text.len() as isize) / 2);

    // Generate padding string
    let padding_str: String = std::iter::repeat(" ").take(padding as usize).collect();
    // Print centered text
    println!("{}{}", padding_str, bright_blue.bold().paint(text));
    ParserConstructor(
        &map,
        count,
        &tokens,
        &mut final_rules,
        &T,
        &terminal_indices,
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn grammar1() {
        let file = "ex.c";
        let grammar = "Grammar1";
        driverFunction(
            ("source_files/".to_owned() + grammar).as_str(),
            ("example_files/".to_owned() + file).as_str(),
        );
    }

    #[test]
    fn grammar2() {
        let file = "g3.c";
        let grammar = "Grammar3";
        driverFunction(
            ("source_files/".to_owned() + grammar).as_str(),
            ("example_files/".to_owned() + file).as_str(),
        );
    }
}
