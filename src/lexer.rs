use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

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
    let H: HashSet<char> = ('a'..='f').chain('A'..='F').chain('0'..='9').collect();
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
    let mut cv: Vec<char> = contents.chars().collect();
    // cv = dbg!(cv);
    // println!("{}",cv.len());
    while true {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lexing() {
        let KEYWORDS: HashSet<&str> = {
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
        let K: HashSet<&str> = {
            let mut map = HashSet::new();
            map.insert("id");
            map.insert("num");
            map.insert("+");
            map.insert("*");
            map.insert("(");
            map.insert(")");
            map
        };
        let file = "example_file/ex2.c";
        let mut tokens = lexer(file, K, key);
        for i in tokens {
            println!("{} {}", i.val, i.kind);
        }
    }
}
