use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

struct Stack {
    contents: Vec<char>,
}

struct CrateStacks {
    stacks: Vec<Stack>,
}

struct MoveInstruction {
    moved_amount: usize,
    from_stack: usize,
    to_stack: usize,
}

impl MoveInstruction {
    // need to reduce duplicate code without just passing in a bool
    fn execute_move(&self, mut crate_stacks: CrateStacks) -> CrateStacks {
        let mut pushed_values = crate_stacks.stacks[self.from_stack]
            .contents
            .drain(..self.moved_amount)
            .rev()
            .collect::<Vec<char>>();

        pushed_values.append(&mut crate_stacks.stacks[self.to_stack].contents);

        crate_stacks.stacks[self.to_stack] = Stack {
            contents: pushed_values,
        };
        crate_stacks
    }

    fn move_bulk(&self, mut crate_stacks: CrateStacks) -> CrateStacks {
        let mut pushed_values = crate_stacks.stacks[self.from_stack]
            .contents
            .drain(..self.moved_amount)
            .collect::<Vec<char>>();

        pushed_values.append(&mut crate_stacks.stacks[self.to_stack].contents);

        crate_stacks.stacks[self.to_stack] = Stack {
            contents: pushed_values,
        };
        crate_stacks
    }
}

impl MoveInstruction {
    fn new(x: String) -> Self {
        let Some((_, x)) = x.split_once("move ") else {
            panic!("Expected move instruction to contain the phrase `move `");
        };

        let Some((move_amount, x)) = x.split_once(" from ") else {
            panic!("Expected move instruction to contain the phrase ` from `");
        };

        let Some((from_stack, to_stack)) = x.split_once(" to ") else {
            panic!("Expected move instruction to contain the phrase ` to `");
        };

        MoveInstruction {
            moved_amount: move_amount.parse().unwrap(),
            from_stack: from_stack.parse::<usize>().unwrap() - 1,
            to_stack: to_stack.parse::<usize>().unwrap() - 1,
        }
    }
}

fn get_input(path: &str) -> Lines<BufReader<File>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
}

fn parse_input(path: &str) -> (CrateStacks, Vec<MoveInstruction>) {
    let mut line_iter = get_input(path);

    let mut provided_crate_lines = Vec::new();
    while let Some(Ok(current_line)) = line_iter.next() {
        if current_line.is_empty() {
            // match lines up until an empty line occurs, which indicates the start of the move instructions
            break;
        }
        provided_crate_lines.push(current_line);
    }
    provided_crate_lines.pop(); // remove the line containing crate stack index's

    (
        get_crate_stacks(provided_crate_lines),
        line_iter
            .map(|x| x.unwrap())
            .into_iter()
            .map(MoveInstruction::new)
            .collect(),
    )
}

fn get_line_values(mut string: String) -> Vec<Option<char>> {
    let mut result = Vec::new();

    while !string.is_empty() {
        result.push(if let [b'[', x, b']'] = string[..3].as_bytes() {
            Some(*x as char)
        } else {
            None
        });

        string = if string.len() >= 4 {
            string[4..].to_string()
        } else {
            String::new()
        }
    }

    result
}

fn pop_front_stack(crate_contents: &mut Vec<Vec<Option<char>>>) -> Stack {
    let mut stack_contents = Vec::new();

    for current_section in crate_contents.iter_mut() {
        if let Some(x) = current_section.remove(0) {
            stack_contents.push(x);
        }
    }

    Stack {
        contents: stack_contents,
    }
}

/* Expects inputs formatted similar to:
"    [D]    "
"[N] [C]    "
"[Z] [M] [P]"
*/
fn get_crate_stacks(mut crate_contents: Vec<String>) -> CrateStacks {
    let mut stacks = Vec::new();

    let mut crate_contents: Vec<Vec<Option<char>>> = crate_contents
        .into_iter()
        .map(|x| get_line_values(x))
        .collect();

    while !crate_contents[0].is_empty() {
        stacks.push(pop_front_stack(&mut crate_contents));
    }

    CrateStacks { stacks }
}

fn puzzle_1() {
    let (mut crate_stacks, move_instructions) = parse_input("input.txt");
    for current_instruction in move_instructions {
        crate_stacks = current_instruction.execute_move(crate_stacks);
    }

    crate_stacks
        .stacks
        .into_iter()
        .for_each(|x| print!("{}", x.contents[0]));
    println!();
}

fn puzzle_2() {
    let (mut crate_stacks, move_instructions) = parse_input("input.txt");
    for current_instruction in move_instructions {
        crate_stacks = current_instruction.move_bulk(crate_stacks);
    }
    crate_stacks
        .stacks
        .into_iter()
        .for_each(|x| print!("{}", x.contents[0]));
    println!();
}

fn main() {
    puzzle_1();
    puzzle_2();
}
