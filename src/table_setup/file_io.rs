use super::*;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn load_symbols(path: &String, T: &mut Vec<Symbol>, ver: SymbolType) {
    let mut text: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            text.push(line);
        }
    }
    for (_i, st) in text.iter().enumerate() {
        // `index` is the index of the current element, and `st` is a reference to the element
        let st = st.clone(); // Clone the element if you need to own it
        let mut index = None;
        for (i, ch) in st.chars().enumerate() {
            if ch == '#' {
                index = Some(i);
                break;
            }
        }
        let temp = match index {
            None => st,
            Some(token) => st[..token].to_string(),
        };
        let temp = temp.trim().to_string();
        T.push(Symbol {
            name: temp,
            tyype: ver,
        });
    }
}

pub fn load_Production_Rules(
    path: &String,
    map: &HashMap<String, i32>,
    rules: &mut Vec<Vec<Vec<i32>>>,
) {
    let mut processed_lines: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        // Consumes the iterator, returns an (Optional) String
        let mut current_line = String::new();
        for line in lines.flatten() {
            let line = line.trim();

            // If the line is empty or starts with a comment character, skip it
            if line.is_empty() || line.starts_with("//") {
                continue;
            }

            // Add the current line to the joined line
            current_line.push_str(line);

            // If the current line ends with a semicolon, add it to the processed lines
            if line.ends_with(';') {
                processed_lines.push(current_line.clone());
                current_line.clear();
            }
        }
    }
    for line in processed_lines {
        let line = line.trim();
        let mut ind = 0;
        for (i, c) in line.char_indices() {
            if c == ':' {
                ind = i;
                break;
            }
        }
        let non_terminal = line[0..ind].to_string();
        let mut line = (*line).to_string();
        line.drain(0..(ind + 1));
        let parts: Vec<&str> = line.split('|').map(|word| word.trim()).collect();
        let ind = map[&non_terminal[..]];
        for part in parts {
            let mut temp: Vec<i32> = Vec::new();
            // println!("{}", part);
            let tokens: Vec<&str> = part.split(' ').collect();
            for token in tokens {
                let mut toke: String = token.to_string(); // Initialize with the original token value
                if toke.starts_with("'") {
                    toke = toke[("'".len())..].to_string(); // Remove the first character and subsequent substring
                }
                if toke.ends_with("';") {
                    toke = toke[..toke.len() - 2].to_string(); // Remove the last two characters
                }
                if toke.ends_with("'") {
                    toke = toke[..toke.len() - 1].to_string(); // Remove the last two characters
                }
                if toke.ends_with(";") {
                    toke = toke[..toke.len() - 1].to_string(); // Remove the last two characters
                }
                // println!("{}",toke);
                temp.push(map[&toke[..]]);
            }
            rules[ind as usize].push(temp);
        }
    }
}
