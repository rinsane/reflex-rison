use std::clone::Clone;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead};
use std::marker::Copy;
use std::path::Path;
mod file_io;
pub enum SymbolType {
    Terminal,
    NonTerminal,
    StartSymbol,
}
impl Clone for SymbolType {
    fn clone(&self) -> Self {
        match self {
            SymbolType::Terminal => SymbolType::Terminal,
            SymbolType::NonTerminal => SymbolType::NonTerminal,
            SymbolType::StartSymbol => SymbolType::StartSymbol,
        }
    }
}
impl Copy for SymbolType {}

impl PartialEq for SymbolType {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

pub struct Symbol {
    pub name: String,
    tyype: SymbolType,
}

pub fn construct(
    terminals_path: &str,
    non_terminals_path: &str,
    production_rules: &str,
) -> (
    Vec<Symbol>,
    Vec<Symbol>,
    HashMap<String, i32>,
    Vec<Vec<Vec<i32>>>,
    HashSet<i32>,
    HashSet<i32>,
    usize,
) {
    let mut nullables: HashSet<i32> = HashSet::new();
    let mut terminal_indices: HashSet<i32> = HashSet::new();
    let mut terminals: Vec<Symbol> = Vec::new();
    let mut non_terminals: Vec<Symbol> = Vec::new();
    file_io::load_symbols(
        &terminals_path.to_string(),
        &mut terminals,
        SymbolType::Terminal,
    );
    file_io::load_symbols(
        &non_terminals_path.to_string(),
        &mut non_terminals,
        SymbolType::NonTerminal,
    );
    non_terminals[0].tyype = SymbolType::StartSymbol;
    terminals.push(Symbol {
        name: "Eof".to_string(),
        tyype: SymbolType::Terminal,
    });
    terminals.push(Symbol {
        name: "Null".to_string(),
        tyype: SymbolType::Terminal,
    });
    let mut map: HashMap<String, i32> = HashMap::new();
    for (i, val) in non_terminals.iter().enumerate() {
        map.insert(val.name.clone(), i as i32);
    }
    let count: usize = non_terminals.len();
    for (i, val) in terminals.iter().enumerate() {
        terminal_indices.insert((non_terminals.len() + i) as i32);
        map.insert(val.name.clone(), (non_terminals.len() + i) as i32);
    }
    let mut rules: Vec<Vec<Vec<i32>>> = Vec::new();
    rules.resize(non_terminals.len(), Vec::new());
    file_io::load_Production_Rules(&production_rules.to_string(), &mut map, &mut rules);
    // rules = dbg!(rules);
    let mut indices_to_modify = Vec::new();
    // println!("temrinals");
    // print_symbol_vector(&terminals);
    // print_symbol_vector(&non_terminals);
    // map = dbg!(map);
    // rules = dbg!(rules);
    // terminal_indices = dbg!(terminal_indices);
    for (i, val) in rules.iter_mut().enumerate() {
        let mut check_left_recursion: bool = false;
        for (_, inner_val) in val.iter().enumerate() {
            if inner_val[0] == i as i32 {
                check_left_recursion = true;
                break;
            }
        }
        if check_left_recursion {
            indices_to_modify.push(i as i32);
        }
    }
    if !indices_to_modify.is_empty() {
        rules.resize(non_terminals.len() + terminals.len(), Vec::new());
    }

    for index in indices_to_modify {
        remove_left_recursion(
            &mut rules,
            index,
            &mut non_terminals,
            &mut map,
            &mut nullables,
            &mut terminals,
        );
    }
    // rules = dbg!(rules);
    check_nullability(&mut nullables, &mut rules, &map, &terminal_indices);
    // rules = dbg!(rules);
    // nullables = dbg!(nullables);
    let mut fi = match File::create("iofiles/rules.txt") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to create file: {}", err);
            return (
                terminals,
                non_terminals,
                map,
                rules,
                nullables,
                terminal_indices,
                count,
            );
        }
    };
    if let Err(err) = writeln!(fi, "{:?}", rules.clone()) {
        eprintln!("Failed to write to file: {}", err);
    }
    return (
        terminals,
        non_terminals,
        map,
        rules,
        nullables,
        terminal_indices,
        count,
    );
}

fn remove_left_recursion(
    rules: &mut Vec<Vec<Vec<i32>>>,
    index: i32,
    non_terminals: &mut Vec<Symbol>,
    map: &mut HashMap<String, i32>,
    nullables: &mut HashSet<i32>,
    terminals: &mut Vec<Symbol>,
) {
    let mut beta: Vec<Vec<i32>> = Vec::new();
    let mut alpha: Vec<Vec<i32>> = Vec::new();
    for (_, val) in rules[index as usize].iter_mut().enumerate() {
        if val[0] == index {
            val.remove(0);
            alpha.push(val.clone());
        } else {
            beta.push(val.clone());
        }
    }
    rules[index as usize].clear();
    non_terminals.push(Symbol {
        name: non_terminals[index as usize].name.clone() + "`",
        tyype: SymbolType::NonTerminal,
    });

    let namme = non_terminals[non_terminals.len() - 1].name.clone();
    map.insert(
        namme.clone(),
        (non_terminals.len() + terminals.len() - 1) as i32,
    );
    nullables.insert(map[&namme[..]]);
    for (_, val) in beta.iter_mut().enumerate() {
        val.push(map[&namme[..]]);
        rules[index as usize].push(val.clone());
    }
    rules.push(Vec::new());
    let rul_ind = rules.len() - 1;
    rules[rul_ind as usize].push(vec![map[&"Null".to_string()[..]]]);
    for (_, val) in alpha.iter_mut().enumerate() {
        val.push(map[&namme[..]]);
        rules[rul_ind as usize].push(val.clone());
    }
}

#[allow(unused_variables)]
pub fn calFIRST(
    terminals: &mut Vec<Symbol>,
    non_terminals: &mut Vec<Symbol>,
    map: &HashMap<String, i32>,
    rules: &mut Vec<Vec<Vec<i32>>>,
    nullables: &mut HashSet<i32>,
    terminal_indices: &mut HashSet<i32>,
) -> Vec<HashSet<i32>> {
    let mut First: Vec<HashSet<i32>> = Vec::new();
    let mut progress: Vec<bool> = Vec::new();
    First.resize(map.len(), HashSet::new());
    progress.resize(map.len(), false);
    // Create a vector of (key, index) pairs
    let mut pairs: Vec<_> = map.into_iter().collect();

    // Sort the vector by index
    pairs.sort_by_key(|&(_, ref index)| **index);

    // Iterate over the sorted pairs
    // Iterate over the sorted pairs
    let mut fi = match File::create("iofiles/Error.txt") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to create file: {}", err);
            return First; // Exit the function if file creation fails
        }
    };
    // File writing logic
    let mut rm = HashMap::new();
    for (key, &value) in map.iter() {
        // Inserting the value as key and key as value
        rm.insert(value, key.clone());
    }
    for (key, i) in pairs {
        // Convert i to usize before using it as an index
        let index = *i as usize;
        if terminal_indices.contains(&(index as i32)) {
            progress[index] = true;
            continue;
        }
        // println!("Call {}",key);
        if let Err(err) = writeln!(fi, "Call {}", key) {
            eprintln!("Failed to write to file: {}", err);
        }
        // Call the function with the correct index type
        first(
            index,
            &mut First,
            &mut progress,
            rules,
            terminal_indices,
            nullables,
            non_terminals,
            map,
            &mut fi,
            &rm,
        );

        // Set the progress flag
        progress[index] = true;
    }

    let output_file = "iofiles/First.txt";
    let mut file = match File::create(output_file) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to create file: {}", err);
            return First; // Exit the function if file creation fails
        }
    };

    for (i, set) in First.iter().enumerate() {
        if let Err(err) = writeln!(file, "{}", rm[&(i as i32)]) {
            eprintln!("Failed to write to file: {}", err);
        }
        if let Err(err) = writeln!(file, "[") {
            eprintln!("Failed to write to file: {}", err);
        }
        for &terminal in set {
            if let Err(err) = writeln!(file, "{}", rm[&terminal]) {
                eprintln!("Failed to write to file: {}", err);
            }
        }
        if let Err(err) = writeln!(file, "]") {
            eprintln!("Failed to write to file: {}", err);
        }
    }
    return First;
}

fn first(
    index: usize,
    First: &mut Vec<HashSet<i32>>,
    progress: &mut Vec<bool>,
    rules: &Vec<Vec<Vec<i32>>>,
    terminal_indices: &mut HashSet<i32>,
    nullables: &mut HashSet<i32>,
    non_terminals: &mut Vec<Symbol>,
    map: &HashMap<String, i32>,
    fi: &mut File,
    rm: &HashMap<i32, String>,
) {
    if !progress[index] {
        // If progress is false, mark it as true and return the value at the index
        progress[index] = true;
        let mut ans: HashSet<i32> = HashSet::new();
        for val in rules[index].iter() {
            for token in val.iter() {
                // println!("{}",token.clone());
                // println!("Is terminal {}",terminal_indices.contains(token));
                // println!("Is null {}",map[&"Null".to_string()] != *token);
                if let Err(err) = writeln!(fi, "{}", token.clone()) {
                    eprintln!("Failed to write to file: {}", err);
                }
                if let Err(err) = writeln!(fi, "Is terminal {}", terminal_indices.contains(token)) {
                    eprintln!("Failed to write to file: {}", err);
                }
                if let Err(err) = writeln!(fi, "Is null {}", map[&"Null".to_string()] != *token) {
                    eprintln!("Failed to write to file: {}", err);
                }
                if terminal_indices.contains(token) && map[&"Null".to_string()] != *token {
                    ans.insert(token.clone());
                    break;
                } else if terminal_indices.contains(token) && map[&"Null".to_string()] == *token {
                    continue;
                } else {
                    if !nullables.contains(token) {
                        // println!("Un null {}",non_terminals[token.clone() as usize].name);
                        if let Err(err) =
                            writeln!(fi, "Un null {}", non_terminals[token.clone() as usize].name)
                        {
                            eprintln!("Failed to write to file: {}", err);
                        }
                        first(
                            token.clone() as usize,
                            First,
                            progress,
                            rules,
                            terminal_indices,
                            nullables,
                            non_terminals,
                            map,
                            fi,
                            rm,
                        );
                        ans.extend(First[token.clone() as usize].clone());
                        break;
                    } else {
                        // println!("Null {}",non_terminals[token.clone() as usize].name);
                        if let Err(err) =
                            writeln!(fi, "Null {}", non_terminals[token.clone() as usize].name)
                        {
                            eprintln!("Failed to write to file: {}", err);
                        }
                        first(
                            token.clone() as usize,
                            First,
                            progress,
                            rules,
                            terminal_indices,
                            nullables,
                            non_terminals,
                            map,
                            fi,
                            rm,
                        );
                        ans.extend(First[token.clone() as usize].clone());
                    }
                }
            }
        }

        // dbg!(ans.clone());
        if let Err(err) = writeln!(fi, "{:?}", ans.clone()) {
            eprintln!("Failed to write to file: {}", err);
        }
        // println!(" Done {}",rm[&(index as i32)]);
        if let Err(err) = writeln!(fi, "Done {}\n", rm[&(index as i32)]) {
            eprintln!("Failed to write to file: {}", err);
        }
        First[index] = ans.clone();
    }
}

pub fn check_nullability(
    nullables: &mut HashSet<i32>,
    rules: &mut Vec<Vec<Vec<i32>>>,
    map: &HashMap<String, i32>,
    terminal_indices: &HashSet<i32>,
) {
    for (i, val) in rules.iter().enumerate() {
        let mut nulls = false;
        for v in val.iter() {
            let mut n_ulls = true;
            for j in v.iter() {
                if terminal_indices.contains(j) && map[&"Null".to_string()] != *j {
                    n_ulls = false;
                    break;
                } else if !terminal_indices.contains(j) && !nullables.contains(j) {
                    n_ulls = false;
                    break;
                }
            }
            if n_ulls {
                nulls = true;
                break;
            }
        }
        if nulls {
            nullables.insert(i as i32);
        }
    }
}

#[allow(unused_variables)]
pub fn calFollow(
    terminals: &mut Vec<Symbol>,
    non_terminals: &mut Vec<Symbol>,
    map: &mut HashMap<String, i32>,
    rules: &mut Vec<Vec<Vec<i32>>>,
    nullables: &mut HashSet<i32>,
    terminal_indices: &mut HashSet<i32>,
    First: &mut Vec<HashSet<i32>>,
) -> Vec<HashSet<i32>> {
    let mut Follow: Vec<HashSet<i32>> = Vec::new();
    let mut progress: Vec<bool> = Vec::new();
    Follow.resize(map.len(), HashSet::new());
    progress.resize(map.len(), false);
    Follow[0].insert(map[&"Eof".to_string()]);
    // println!("follow check");
    // Follow = dbg!(Follow);
    let mut pairs: Vec<_> = map.into_iter().collect();
    // Sort the vector by index
    pairs.sort_by_key(|&(_, ref index)| **index);

    // Iterate over the sorted pairs
    // Iterate over the sorted pairs
    for (_key, i) in pairs {
        // Convert i to usize before using it as an index
        let index = *i as usize;
        if terminal_indices.contains(&(index as i32)) {
            progress[index] = true;
            continue;
        }
        // Call the function with the correct index type
        // println!("Call {}",non_terminals[index as usize].name);
        follow(
            index as i32,
            rules,
            terminal_indices,
            nullables,
            First,
            &mut Follow,
            &mut progress,
        );

        // Set the progress flag
        progress[index] = true;
    }

    // File writing logic
    let output_file = "iofiles/Follow.txt";
    let mut file = match File::create(output_file) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to create file: {}", err);
            return Follow; // Exit the function if file creation fails
        }
    };

    for (i, set) in Follow.iter().enumerate() {
        if let Err(err) = writeln!(file, "{}", i) {
            eprintln!("Failed to write to file: {}", err);
        }
        if let Err(err) = writeln!(file, "[") {
            eprintln!("Failed to write to file: {}", err);
        }
        for &terminal in set {
            if let Err(err) = writeln!(file, "{}", terminal) {
                eprintln!("Failed to write to file: {}", err);
            }
        }
        if let Err(err) = writeln!(file, "]") {
            eprintln!("Failed to write to file: {}", err);
        }
    }
    return Follow;
}

fn follow(
    index: i32,
    rules: &mut Vec<Vec<Vec<i32>>>,
    terminal_indices: &HashSet<i32>,
    nullables: &mut HashSet<i32>,
    First: &mut Vec<HashSet<i32>>,
    Follow: &mut Vec<HashSet<i32>>,
    progress: &mut Vec<bool>,
) {
    if !progress[index as usize] {
        progress[index as usize] = true;
        let mut result: HashSet<i32> = HashSet::new();
        let mut follow_calls: Vec<i32> = Vec::new();

        for (ix, val) in rules.iter_mut().enumerate() {
            // println!("index num {}",ix);
            for (_uu, production) in val.iter().enumerate() {
                let mut found_index = None;

                // Find the index of the symbol 'index' in the production, starting from the end
                for (j, &item) in production.iter().rev().enumerate() {
                    if item == index {
                        found_index = Some(j);
                        break;
                    }
                }

                if let Some(index_in_production) = found_index {
                    let mut done = false;
                    // Iterate over the symbols after 'index' in the production
                    let it = production.len() - index_in_production - 1;
                    // println!("index found {} in rule {} ",it, uu);
                    // println!("Items inserted");
                    for &item in production.iter().skip(it + 1) {
                        if terminal_indices.contains(&item) {
                            // If it's a terminal symbol, add it to the result
                            // println!("terminal {}",item.clone());
                            result.insert(item);
                            // Stop further iteration
                            done = true;
                            break;
                        } else if !nullables.contains(&item) {
                            // println!("non nullable {}",item.clone());
                            result.extend(First[item as usize].iter().cloned());
                            done = true;
                            break;
                        } else {
                            // println!("nullable {}",item.clone());
                            result.extend(First[item as usize].iter().cloned());
                        }
                    }
                    if !done && index != (ix as i32) {
                        follow_calls.push(ix as i32);
                    }
                }
            }
        }
        // println!("follow calls");
        // Handle follow calls outside the loop to avoid multiple mutable borrows
        for ix in follow_calls {
            follow(
                ix,
                rules,
                terminal_indices,
                nullables,
                First,
                Follow,
                progress,
            );
            // println!("{}",ix.clone());
            result.extend(Follow[ix as usize].iter().cloned());
        }

        Follow[index as usize].extend(result.clone());
        // dbg!(Follow[index.clone() as usize].clone());
        // println!("Done");
        // Now 'result' contains FOLLOW set for 'index'
        // println!("FOLLOW({}): {:?}", index, result);
    }
}

pub fn table_maker(
    terminals: &mut Vec<Symbol>,
    rules: &mut Vec<Vec<Vec<i32>>>,
    First: &mut Vec<HashSet<i32>>,
    Follow: &mut Vec<HashSet<i32>>,
    nullables: &mut HashSet<i32>,
    terminal_indices: &mut HashSet<i32>,
    map: &mut HashMap<String, i32>,
    count: i32,
) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let rows = rules.len();
    let columns = terminals.len() - 1;
    // Print dimensions
    // println!("Rows: {}, Columns: {}", rows, columns);
    // File writing logic
    let mut rm = HashMap::new();
    for (key, &value) in map.iter() {
        // Inserting the value as key and key as value
        rm.insert(value, key.clone());
    }
    let mut T: Vec<Vec<i32>> = Vec::with_capacity(rows);
    for _ in 0..rows {
        let row = vec![-1; columns];
        T.push(row);
    }

    let mut final_rules: Vec<Vec<i32>> = Vec::new();
    let mut counter = 0;

    for (i, outer_vec) in rules.iter().enumerate() {
        // println!("i: {}", i); // Print iteration count

        if terminal_indices.contains(&(i as i32)) {
            continue;
        }
        if nullables.contains(&(i as i32)) {
            for iti in Follow[i].iter() {
                if map[&"Null".to_string()] != *iti as i32 {
                    T[i][((*iti) - count) as usize] = counter;
                }
            }
            // println!("Nullable");
            // T[i] = dbg!(T[i].clone());
            counter += 1;
        }

        for (j, inner_vec) in outer_vec.iter().enumerate() {
            // println!("j: {}", j); // Print iteration count

            final_rules.push((*inner_vec).clone());
            if nullables.contains(&(i as i32)) && j == 0 {
                continue;
            }
            let mut fir: Vec<i32> = Vec::new();
            for (_k, value) in inner_vec.iter().enumerate() {
                if terminal_indices.contains(value) && map[&"Null".to_string()] != *value as i32 {
                    fir.push(*value);
                    break;
                } else if !nullables.contains(value) {
                    fir.extend(First[*value as usize].clone());
                    break;
                } else {
                    fir.extend(First[*value as usize].clone());
                }
            }
            // fir = dbg!(fir);
            for term in fir {
                println!("{}", rm[&term]);
                T[i][(term - count) as usize] = counter;
            }
            counter += 1;
        }
    }

    (T, final_rules)
}

#[allow(dead_code)]
fn write_2d_vector_to_file(filename: &str, vector: &Vec<Vec<i32>>) -> io::Result<()> {
    let mut file = File::create(filename)?;

    for row in vector {
        writeln!(
            file,
            "{}",
            row.iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )?;
    }

    Ok(())
}
