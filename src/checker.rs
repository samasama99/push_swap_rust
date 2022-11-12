use std::{collections::LinkedList, io, process, str::FromStr};

use crate::{
    parse_args,
    stack_moves::stack_moves_mod::{self, push, reverse_rotate, rotate, rr, rrr, ss, Moves},
};

fn is_sorted(stack: &LinkedList<i32>) -> bool {
    let mut stack = stack.clone(); 
    while stack.len() >= 2 {
        let s1 = stack.pop_front().unwrap();
        let s2 = stack.pop_front().unwrap();
        if s1 > s2 {
            return false;
        }
        stack.push_front(s2);
    }
    true
}

fn do_moves(moves: Vec<Moves>, stack_a: &mut LinkedList<i32>, stack_b: &mut LinkedList<i32>) {
    for m in moves {
        match m {
            Moves::Ra => rotate(stack_a),
            Moves::Rb => rotate(stack_b),
            Moves::Rr => rr(stack_a, stack_b),
            Moves::Rrr => rrr(stack_a, stack_b),
            Moves::Sa => stack_moves_mod::swap(stack_a),
            Moves::Sb => stack_moves_mod::swap(stack_b),
            Moves::Ss => ss(stack_a, stack_b),
            Moves::Rra => reverse_rotate(stack_a),
            Moves::Rrb => reverse_rotate(stack_b),
            Moves::Pb => push(stack_a, stack_b),
            Moves::Pa => push(stack_b, stack_a),
        };
    }
}

pub fn start_checker(elements: &[String]) {
    let mut stack_a: LinkedList<i32> = match parse_args(elements) {
        Some(stack) => stack,
        None => {
            eprintln!("Error");
            process::exit(1);
        }
    };

    let Ok(moves) = io::read_to_string(io::stdin()) else {
        eprintln!("Error");
        return;
    };

    let moves: Vec<_> = moves
        .split(|n| n == '\n')
        .filter_map(|m| {
            if m.is_empty() {
                return None;
            }
            return match Moves::from_str(m.trim_end_matches(|n| n == '\n')) {
                Ok(ret) => Some(ret),
                Err(_) => {
                    eprintln!("Error");
                    process::exit(1);
                }
            };
        })
        .collect();

    // dbg!(&moves);
    let original_len = stack_a.len();
    let mut stack_b = LinkedList::new();
    do_moves(moves, &mut stack_a, &mut stack_b);
    // dbg!(&stack_a, &stack_b);
    if stack_a.len() != original_len || !is_sorted(&stack_a) {
        println!("ko");
    } else {
        println!("ok");
    }
}

#[test]
fn test_moves_enum() {
    let x = Moves::Ra;
    assert_eq!(x.to_string(), "ra");
    assert_eq!(Moves::from_str(&x.to_string()).unwrap(), Moves::Ra);
}

#[test]
#[should_panic]
fn test_not_a_move() {
    let not_a_move = "xa";
    let _ = Moves::from_str(not_a_move).unwrap();
}
