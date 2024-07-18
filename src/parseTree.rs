use super::*;
use std::collections::{HashMap, VecDeque};
use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};
#[derive(Debug, Clone)]
pub struct TreeNode {
    val: (i32, i32),
    children: Vec<Box<TreeNode>>,
}

fn insert(head: &mut TreeNode, val: (i32, i32)) {
    head.children.push(Box::new(TreeNode {
        val: val,
        children: Vec::new(),
    }));
}

fn DFS(head: &mut TreeNode, val: (i32, i32)) -> Option<&mut TreeNode> {
    let mut stack: Vec<&mut TreeNode> = Vec::new();
    stack.push(head);
    while !stack.is_empty() {
        let top = stack.pop().unwrap();
        if (*top).val == val {
            return Some(top);
        }
        for i in top.children.iter_mut() {
            stack.push(i.borrow_mut());
        }
    }
    return Option::None;
}
fn print_tree(root: &TreeNode, prefix: i32, rm: &HashMap<i32, String>) {
    for i in 0..prefix {
        print!("\t");
    }
    print!("{}", rm[&root.val.0]);
    print!("\n");
    let num_children = root.children.len();
    for (index, child) in root.children.iter().enumerate() {
        let is_last = index == num_children - 1;
        let new_rm = format!("{}", if is_last { "└──" } else { "├──" });
        for i in 0..(prefix - 1) {
            print!("\t");
        }
        print!("{new_rm}");
        print_tree(child, prefix + 1, rm);
        print!("\n");
    }
}
fn print_treee(
    root: &TreeNode,
    color: Colour,
    prefix: String,
    is_last: bool,
    rm: &HashMap<i32, String>,
) {
    if !prefix.is_empty() {
        println!(
            "{}{}{}",
            prefix,
            if is_last { "└── " } else { "├── " },
            color.bold().paint(rm[&root.val.0].clone())
        );
    } else {
        println!("{}", color.bold().paint(rm[&root.val.0].clone()));
    }
    let mut rng = thread_rng();
    let red: i32 = rng.gen_range(0..256);
    let green: i32 = rng.gen_range(0..256);
    let blue: i32 = rng.gen_range(0..256);

    let color = Colour::RGB(red as u8, green as u8, blue as u8);

    let num_children = root.children.len();
    for (index, child) in root.children.iter().enumerate() {
        let new_prefix = format!("{}{}   ", prefix, if is_last { "  " } else { "│   " });
        let is_last_child = index == num_children - 1;
        print_treee(child, color, new_prefix, is_last_child, rm);
    }
}
fn print_level_order(root: &TreeNode, rm: &HashMap<i32, String>) {
    let mut queue = VecDeque::new();
    queue.push_back(root);

    while !queue.is_empty() {
        if let Some(node) = queue.pop_front() {
            print!("{} ", rm[&node.val.0]);

            for child in node.children.iter() {
                queue.push_back(&**child);
            }
        }
    }
}
pub fn ParserConstructor(
    map: &HashMap<String, i32>,
    count: usize,
    tokens: &Vec<Token>,
    final_rules: &mut Vec<Vec<i32>>,
    T: &Vec<Vec<i32>>,
    terminal_indices: &HashSet<i32>,
) {
    let mut stack: Vec<(i32, i32)> = Vec::new();
    let eof = map["Eof"];
    let mut ct = 0;
    let mut rm: HashMap<i32, String> = HashMap::new();
    for (key, &value) in map.iter() {
        // Inserting the value as key and key as value
        rm.insert(value, key.clone());
    }
    let mut cc = 0;
    stack.push((map["Eof"], cc));
    cc += 1;
    stack.push((0, cc));
    let mut parseTree = TreeNode {
        val: (0, cc),
        children: Vec::new(),
    };
    cc += 1;
    while {
        if let Some(no) = stack.last() {
            if (*no).0 == eof {
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
        if map[&"Null".to_string()] == x.0 {
            stack.pop();
            continue;
        }

        // print!("\nStack\n");
        // for ui in stack.iter().rev(){
        //     print!("{} ,",rm[&(*ui).0]);
        // }
        // print!("\n\n");
        // println!("{} {} {} {}",x.0,tokens[ct].val,tokens[ct].kind,map[&tokens[ct].kind] as usize-count);

        if terminal_indices.contains(&x.0) {
            if x.0 == map[&tokens[ct].kind] {
                stack.pop();
                ct += 1;
            } else {
                panic!("Terminal error {} {}", x.0, map[&tokens[ct].kind]);
            }
        } else if T[x.0 as usize][map[&tokens[ct].kind] as usize - count] != -1 {
            let rulenum = T[x.0 as usize][map[&tokens[ct].kind] as usize - count];
            //Do Depth first search here to find
            let find = DFS(&mut parseTree, x).unwrap();
            stack.pop();
            for ui in <Vec<i32> as Clone>::clone(&final_rules[rulenum as usize])
                .into_iter()
                .rev()
            {
                //insert chilren here
                stack.push((ui, cc));
                find.children.push(Box::new(TreeNode {
                    val: (ui, cc),
                    children: Vec::new(),
                }));
                cc += 1;
            }
        } else {
            panic!("Parse erorr");
        }
    }
    print_treee(&mut parseTree, Colour::Red, String::new(), true, &rm);
}
