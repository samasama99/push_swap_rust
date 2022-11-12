// #![allow(unused)]

mod stack_moves;
mod checker;

use std::{
    collections::{HashSet, LinkedList},
    env::args,
    process,
};

use checker::start_checker;

use crate::stack_moves::stack_moves_mod::*;

fn parse_args(args: &[String]) -> Option<LinkedList<i32>> {
    let args = args.iter().flat_map(|el| el.split_whitespace());
    let mut res: LinkedList<i32> = LinkedList::new();
    let mut exists = HashSet::new();

    for arg in args {
        match arg.parse() {
            Ok(el) => {
                if exists.contains(&el) {
                    return None;
                }
                exists.insert(el);
                res.push_back(el);
            }
            Err(_) => return None,
        };
    }
    Some(res)
}

fn chunk(stack_a: &mut LinkedList<i32>, min: i32, max: i32) -> (LinkedList<i32>, Vec<Moves>) {
    assert!(min < max);
    let mut stack_b = LinkedList::new();
    let (mut max, mut min, diff) = (max, min, max - min);
    let mut moves = Vec::new();
    while let Some(current) = stack_a.front() {
        let num = *current;

        let min_element = *stack_a.iter().min().unwrap();

        if max < min_element {
            min = min_element;
            max = min + diff;
        }

        if num <= max {
            moves.push(Moves::Pb);
            push(stack_a, &mut stack_b).expect("not enough elements in stack to push from A to B");
            if num < min {
                moves.push(Moves::Rb);
                rotate(&mut stack_b).expect("not enough elements in stack to rotate B");
            }
            min += 1;
            max += 1;
        } else if stack_a.len() > 2 {
            let (index, _) = get_min_element(stack_a);
            if index < stack_a.len() / 2 {
                moves.push(Moves::Ra);
                rotate(stack_a).expect("not enough elements in stack to rotate A");
            } else {
                moves.push(Moves::Rra);
                reverse_rotate(stack_a)
                    .expect("not enough elements in stack to reverse rotate A");
            }
        } else {
            moves.push(Moves::Pb);
            push(stack_a, &mut stack_b)
                .expect("not enough elements in stack to push from A to B");
        }
    }
    (stack_b, moves)
}

fn get_max_element(val: &LinkedList<i32>) -> (usize, i32) {
    let (index, value) = val
        .iter()
        .enumerate()
        .max_by(|x, y| {
            let ((_, x), (_, y)) = (x, y);
            x.cmp(y)
        })
        .unwrap();
    (index, *value)
}

fn get_min_element(val: &LinkedList<i32>) -> (usize, i32) {
    let (index, value) = val
        .iter()
        .enumerate()
        .min_by(|x, y| {
            let ((_, x), (_, y)) = (x, y);
            x.cmp(y)
        })
        .unwrap();
    (index, *value)
}

fn sort(mut stack_b: LinkedList<i32>) -> Vec<Moves> {
    let mut stack_a = LinkedList::new();
    let mut moves = Vec::new();
    while !stack_b.is_empty() {
        let (index, _) = get_max_element(&stack_b);
        if index < stack_b.len() / 2 {
            for _ in 0..index {
                moves.push(Moves::Rb);
                rotate(&mut stack_b).expect("not enough elements in stack to rotate B");
            }
            moves.push(Moves::Pa);
            push(&mut stack_b, &mut stack_a)
                .expect("not enough elements in stack to push from B to A");
        } else {
            for _ in 0..stack_b.len() - index {
                moves.push(Moves::Rrb);
                reverse_rotate(&mut stack_b)
                    .expect("not enough elements in stack to reverse rotate B");
            }
            moves.push(Moves::Pa);
            push(&mut stack_b, &mut stack_a)
                .expect("not enough elements in stack to push from B to A");
        }
    }
    moves
}

fn get_max_of_interval(size: usize) -> i32 {
    (size * 21 / 400 + 35 / 4) as i32
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        return;
    }

    if args[1] == "checker" && args.len() >= 3 {
        start_checker(&args[2..]);
        return;
    } 

    let (_, elements) = args.split_first().unwrap();

    let mut stack_a: LinkedList<i32> = match parse_args(elements) {
        Some(stack) => stack,
        None => {
            eprintln!("Error");
            process::exit(1);
        }
    };

    if stack_a.len() <= 2 {
        return;
    }

    let max = get_max_of_interval(stack_a.len());
    let (stack_b, mut moves) = chunk(&mut stack_a, 0, max);
    moves.append(&mut sort(stack_b));
    moves.iter().for_each(|s| println!("{}", s));
}
