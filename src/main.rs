#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod lexer;
mod parseTree;
mod table_setup;
use std::collections::{HashMap, HashSet};
use ansi_term::Colour;
use lexer::*;
use parseTree::*;
use rand::prelude::*;
use std::cmp;
use table_setup::*;
use term_size::dimensions_stdout;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let file = "example_file/ex.c";
    driverFunction("source_files/Grammar1", file);
}

fn driverFunction(rootPath: &str, file: &str) {

    // PATHS
    let terminals_path = format!("{}/term.txt", rootPath);
    let terminals_symbolic_path = format!("{}/termsymbolic.txt", rootPath);
    let non_terminals_path = format!("{}/nonterm.txt", rootPath);
    let production_rules = format!("{}/cfg.txt", rootPath);

    //Keywords
    let key: HashMap<&str, &str> = {
        let mut map = HashMap::new();
        map.insert("id", "id");
        map.insert("num", "num");
        map.insert("+", "plus");
        map.insert("*", "asterisk");
        map.insert("(", "leftb");
        map.insert(")", "rightb");
        map
    };
    let keyy: HashSet<&str> = {
        let mut map = HashSet::new();
        map.insert("id");
        map.insert("num");
        map.insert("+");
        map.insert("*");
        map.insert("(");
        map.insert(")");
        map
    };

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
    let text = format!(
        "Parse Tree for the provided snippet:\n\n",
    );

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
