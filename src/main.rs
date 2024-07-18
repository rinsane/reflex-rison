#[allow(non_snake_case, warnings)]
use std::collections::HashMap;
use std::collections::HashSet;
mod lexer;
mod parseTree;
mod table_setup;
use ansi_term::Colour;
use lexer::*;
use parseTree::*;
use rand::prelude::*;
use std::cmp;
use table_setup::*;
use term_size::dimensions_stdout;
fn main() {
    //!Keywords
    let keyset: HashSet<&str> = {
        let mut map = HashSet::new();
        map.insert("auto");
        map.insert("break");
        map.insert("case");
        map.insert("char");
        map.insert("const");
        map.insert("continue");
        map.insert("default");
        map.insert("do");
        map.insert("double");
        map.insert("else");
        map.insert("enum");
        map.insert("extern");
        map.insert("float");
        map.insert("for");
        map.insert("goto");
        map.insert("if");
        map.insert("int");
        map.insert("long");
        map.insert("register");
        map.insert("return");
        map.insert("short");
        map.insert("signed");
        map.insert("unsigned");
        map.insert("sizeof");
        map.insert("static");
        map.insert("struct");
        map.insert("switch");
        map.insert("typedef");
        map.insert("union");
        map.insert("void");
        map.insert("volatile");
        map.insert("while");
        map.insert("...");
        map.insert(">>=");
        map.insert("<<=");
        map.insert("+=");
        map.insert("-=");
        map.insert("*=");
        map.insert("/=");
        map.insert("%=");
        map.insert("&=");
        map.insert("^=");
        map.insert("|=");
        map.insert(">>");
        map.insert("<<");
        map.insert("++");
        map.insert("--");
        map.insert("->");
        map.insert("&&");
        map.insert("||");
        map.insert("<=");
        map.insert(">=");
        map.insert("==");
        map.insert("!=");
        map.insert("{");
        map.insert("}");
        map.insert(",");
        map.insert(":");
        map.insert("=");
        map.insert("(");
        map.insert(")");
        map.insert("[");
        map.insert("]");
        map.insert(".");
        map.insert("&");
        map.insert("!");
        map.insert("~");
        map.insert("-");
        map.insert("+");
        map.insert("*");
        map.insert("/");
        map.insert("%");
        map.insert("<");
        map.insert("^");
        map.insert("|");
        map.insert("?");
        map.insert(";");
        map.insert(">");
        map
    };
    let keywords: HashMap<&str, &str> = {
        let mut map = HashMap::new();
        map.insert("auto", "AUTO");
        map.insert("break", "BREAK");
        map.insert("case", "CASE");
        map.insert("char", "CHAR");
        map.insert("const", "CONST");
        map.insert("continue", "CONTINUE");
        map.insert("default", "DEFAULT");
        map.insert("do", "DO");
        map.insert("double", "DOUBLE");
        map.insert("else", "ELSE");
        map.insert("enum", "ENUM");
        map.insert("extern", "EXTERN");
        map.insert("float", "FLOAT");
        map.insert("for", "FOR");
        map.insert("goto", "GOTO");
        map.insert("if", "IF");
        map.insert("int", "INT");
        map.insert("long", "LONG");
        map.insert("register", "REGISTER");
        map.insert("return", "RETURN");
        map.insert("short", "SHORT");
        map.insert("signed", "SIGNED");
        map.insert("unsigned", "UNSIGNED");
        map.insert("sizeof", "SIZEOF");
        map.insert("static", "STATIC");
        map.insert("struct", "STRUCT");
        map.insert("switch", "SWITCH");
        map.insert("typedef", "TYPEDEF");
        map.insert("union", "UNION");
        map.insert("void", "VOID");
        map.insert("volatile", "VOLATILE");
        map.insert("while", "WHILE");
        map.insert("...", "ELLIPSIS");
        map.insert(">>=", "RIGHT_ASSIGN");
        map.insert("<<=", "LEFT_ASSIGN");
        map.insert("+=", "ADD_ASSIGN");
        map.insert("-=", "SUB_ASSIGN");
        map.insert("*=", "MUL_ASSIGN");
        map.insert("/=", "DIV_ASSIGN");
        map.insert("%=", "MOD_ASSIGN");
        map.insert("&=", "AND_ASSIGN");
        map.insert("^=", "XOR_ASSIGN");
        map.insert("|=", "OR_ASSIGN");
        map.insert(">>", "RIGHT_OP");
        map.insert("<<", "LEFT_OP");
        map.insert("++", "INC_OP");
        map.insert("--", "DEC_OP");
        map.insert("->", "PTR_OP");
        map.insert("&&", "AND_OP");
        map.insert("||", "OR_OP");
        map.insert("<=", "LE_OP");
        map.insert(">=", "GE_OP");
        map.insert("==", "EQ_OP");
        map.insert("!=", "NE_OP");
        map.insert("{", "LeftBrace");
        map.insert("}", "RightBrace");
        map.insert(",", "Comma");
        map.insert(":", "Colon");
        map.insert("=", "Equal");
        map.insert("(", "LeftParenthesis");
        map.insert(")", "RightParenthesis");
        map.insert("[", "LeftBracket");
        map.insert("]", "RightBracket");
        map.insert(".", "Dot");
        map.insert("&", "Ampersand");
        map.insert("!", "Exclamation");
        map.insert("~", "Tilde");
        map.insert("-", "Minus");
        map.insert("+", "Plus");
        map.insert("*", "Asterisk");
        map.insert("/", "Slash");
        map.insert("%", "Percent");
        map.insert("<", "LessThan");
        map.insert(">", "GreaterThan");
        map.insert("^", "Caret");
        map.insert("|", "VerticalBar");
        map.insert("?", "QuestionMark");
        map.insert(";", "Semicolon");
        map
    };
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
    let terminals_path = String::from("source_files/testterminals.txt");
    let non_terminals_path = String::from("source_files/testnonterminals.txt");
    let production_rules = String::from("source_files/testcfg.txt");
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
    let (mut T, mut final_rules) = table_maker(
        &mut terminals,
        &mut rules,
        &mut first,
        &mut follow,
        &mut nullables,
        &mut terminal_indices,
        &mut map,
        count as i32,
    );
    let file = "example_file/ex.c";
    let mut tokens = lexer(file, keyy, key);
    tokens.push(Token {
        val: "Eof".to_string(),
        kind: "Eof".to_string(),
    });
    // RGB values for bright blue
    let bright_blue = Colour::RGB(0, 191, 255);
    // Unicode escape sequence for smiley emoji
    let smiley = "\u{1F60A}";
    // Get terminal width
    let (term_width, _) = dimensions_stdout().unwrap_or((80, 25)); // Default width: 80

    // Text content
    let text = format!(
        "{}{}Parse tree ban gya finally{}{}\n\n",
        smiley, smiley, smiley, smiley
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

fn LLDriver(
    map: &HashMap<String, i32>,
    count: usize,
    tokens: &Vec<Token>,
    final_rules: &mut Vec<Vec<i32>>,
    T: &Vec<Vec<i32>>,
    terminal_indices: &HashSet<i32>,
) {
    let mut stack: Vec<i32> = Vec::new();
    stack.push(map["Eof"]);
    stack.push(0);
    let eof = map["Eof"];
    let mut ct = 0;
    let mut rm = HashMap::new();
    for (key, &value) in map.iter() {
        // Inserting the value as key and key as value
        rm.insert(value, key.clone());
    }
    while {
        if let Some(no) = stack.last() {
            if *no == eof {
                false
            } else {
                true
            }
        } else {
            panic!("Stack error");
        }
    } {
        let x = if let Some(no) = stack.last() {
            *no
        } else {
            panic!("Stack error 2");
        };
        // stack = dbg!(stack);
        if x == map[&"Null".to_string()] {
            stack.pop();
            continue;
        }
        print!("\nStack\n");
        for ui in stack.iter().rev() {
            print!("{} ,", rm[ui]);
        }
        print!("\n\n");
        println!("{x} {} {}", tokens[ct].val, tokens[ct].kind);
        println!(
            "{x} {} {} {}",
            tokens[ct].val,
            tokens[ct].kind,
            map[&tokens[ct].kind] as usize - count
        );
        if terminal_indices.contains(&x) {
            if x == map[&tokens[ct].kind] {
                stack.pop();
                ct += 1;
            } else {
                panic!("Terminal error {x} {}", map[&tokens[ct].kind]);
            }
        } else if T[x as usize][map[&tokens[ct].kind] as usize - count] != -1 {
            let rulenum = T[x as usize][map[&tokens[ct].kind] as usize - count];
            stack.pop();
            for ui in <Vec<i32> as Clone>::clone(&final_rules[rulenum as usize])
                .into_iter()
                .rev()
            {
                stack.push(ui);
            }
        } else {
            panic!("Parse erorr");
        }
    }
}
