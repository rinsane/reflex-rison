use std::collections::{HashMap, HashSet};
use std::fs;

#[allow(dead_code)]
pub struct Token {
    pub val: String,
    pub kind: String,
}

pub fn lexer(
    file_path: &str,
    KEYWORDS: HashSet<&str>,
    keywords: HashMap<&str, &str>,
) -> Vec<Token> {
    //!Some basic alphabets
    let D: HashSet<char> = ('0'..='9').collect();
    let L: HashSet<char> = ('a'..='z')
        .chain('A'..='Z')
        .chain(std::iter::once('_'))
        .collect();
    let _H: HashSet<char> = ('a'..='f').chain('A'..='F').chain('0'..='9').collect();
    let ic: HashSet<char> = [' ', '\t', '\n', '\r', '\x0B', '\x0C']
        .iter()
        .cloned()
        .collect();
    let Ca: HashSet<char> = ('A'..='Z').collect();
    let Sm: HashSet<char> = ('a'..='z').collect();

    // Read the contents of the file into a string
    let contents = fs::read_to_string(file_path).unwrap();

    // println!("Start");
    // Print the contents of the file
    // println!("{}", contents);
    // println!("End");

    //?Logic
    let mut tokens: Vec<Token> = Vec::new();
    let mut s: usize = 0;
    // Convert the string to a vector of characters
    let cv: Vec<char> = contents.chars().collect();
    // cv = dbg!(cv);
    // println!("{}",cv.len());
    loop {
        // println!("{s}");
        if s >= cv.len() {
            break;
        } else {
            if ic.contains(&cv[s]) {
                //for ignoring characters
                // println!("H");
                s += 1;
            } else if D.contains(&cv[s]) {
                //for getting numbers
                // println!("Here D");
                let mut tr = s;
                let mut onepoint = false;
                while true && tr < cv.len() {
                    if D.contains(&cv[tr]) {
                        tr += 1;
                    } else if cv[tr] == '.' && onepoint == false {
                        onepoint = true;
                        tr += 1;
                    } else {
                        break;
                    }
                }
                //Pushing CONSTANT
                tokens.push(Token {
                    val: cv[s..tr].to_vec().iter().collect(),
                    kind: "CONSTANT".to_string(),
                });
                s = tr;
                // println!("{s}");
            } else if cv[s] == '"' {
                //for getting String literals
                // println!("T");
                let mut tr = s + 1;
                while true && tr < cv.len() {
                    if cv[tr] == '"' {
                        tr += 1;
                        break;
                    } else {
                        tr += 1;
                    }
                }
                tokens.push(Token {
                    val: cv[s..tr].to_vec().iter().collect(),
                    kind: "STRING_LITERAL".to_string(),
                });
                s = tr;
                // println!("{s}");
            } else if cv[s] == '\'' {
                if s + 2 < cv.len() && cv[s + 2] == '\'' {
                    tokens.push(Token {
                        val: cv[s..(s + 3)].to_vec().iter().collect(),
                        kind: "STRING_LITERAL".to_string(),
                    });
                    s = s + 3;
                } else {
                    panic!("Parsing error::Single character not detected correclty");
                }
            } else if L.contains(&cv[s]) {
                if Ca.contains(&cv[s]) || cv[s] == '_' {
                    //Identifier
                    let mut tr = s + 1;
                    while true && tr < cv.len() {
                        if L.contains(&cv[tr]) || D.contains(&cv[tr]) {
                            tr += 1;
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token {
                        val: cv[s..tr].to_vec().iter().collect(),
                        kind: "IDENTIFIER".to_string(),
                    });
                    s = tr;
                } else {
                    let mut tr = s + 1;
                    let mut find = 0;
                    let mut done = false;
                    while true && tr <= cv.len() {
                        // println!("{tr}");
                        let temp: String = cv[s..tr].iter().collect();
                        if KEYWORDS.contains(temp.as_str()) {
                            find = tr;
                            done = true;
                        }
                        if tr < cv.len() && Sm.contains(&cv[tr]) {
                            tr += 1;
                        } else {
                            break;
                        }
                    }
                    let temp: String = cv[s..tr].iter().collect();
                    if KEYWORDS.contains(temp.as_str()) {
                        find = tr;
                        done = true;
                    }
                    if done {
                        tokens.push(Token {
                            val: cv[s..tr].iter().collect(), // Convert the slice to a String
                            kind: keywords
                                .get(&cv[s..tr].iter().collect::<String>().as_str())
                                .unwrap_or(&"Unknown")
                                .to_string(),
                        });
                        s = find;
                    } else {
                        let mut tr = s + 1;
                        while true && tr <= cv.len() {
                            // println!("{tr}");
                            if L.contains(&cv[tr]) || D.contains(&cv[tr]) {
                                tr += 1;
                            } else {
                                break;
                            }
                        }
                        tokens.push(Token {
                            val: cv[s..tr].to_vec().iter().collect(),
                            kind: "IDENTIFIER".to_string(),
                        });
                        s = tr;
                    }
                }
            } else {
                // println!("N");
                let mut tr = s + 1;
                while true && tr <= cv.len() {
                    // println!("{tr}");
                    let temp: String = cv[s..tr].iter().collect();
                    if KEYWORDS.contains(temp.as_str()) {
                        tr += 1;
                    } else {
                        tr -= 1;
                        break;
                    }
                }

                // println!("now {s} {tr}");

                if tr == s {
                    s += 1;
                    continue;
                }

                if tr <= cv.len() {
                    tokens.push(Token {
                        val: cv[s..tr].iter().collect(), // Convert the slice to a String
                        kind: keywords
                            .get(&cv[s..tr].iter().collect::<String>().as_str())
                            .unwrap_or(&"Unknown")
                            .to_string(),
                    });
                } else {
                    tokens.push(Token {
                        val: cv[s..cv.len()].iter().collect(), // Convert the slice to a String
                        kind: keywords
                            .get(&cv[s..cv.len()].iter().collect::<String>().as_str())
                            .unwrap_or(&"Unknown")
                            .to_string(),
                    });
                }
                s = tr;
            }
        }
    }

    return tokens;
}
