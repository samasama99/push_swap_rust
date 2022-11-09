#![allow(unused)]

use std::{
    collections::{HashSet, LinkedList},
    env::args,
    fmt::Display,
    num::ParseIntError,
    process,
};

#[derive(Debug)]
enum Moves {
    Ra,
    Rb,
    Rr,
    Rrr,
    Sa,
    Sb,
    Ss,
    Rra,
    Rrb,
    Pa,
    Pb,
}

impl Display for Moves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Moves::Ra => write!(f, "ra"),
            Moves::Rb => write!(f, "rb"),
            Moves::Rr => write!(f, "rr"),
            Moves::Rrr => write!(f, "rrr"),
            Moves::Sa => write!(f, "sa"),
            Moves::Sb => write!(f, "sb"),
            Moves::Ss => write!(f, "ss"),
            Moves::Rra => write!(f, "rra"),
            Moves::Rrb => write!(f, "rrb"),
            Moves::Pa => write!(f, "pa"),
            Moves::Pb => write!(f, "pb"),
        }
    }
}

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
            Err(e) => return None,
        };
    }
    Some(res)
}

fn rotate(stack: &mut LinkedList<i32>) {
    let e = stack.pop_front().expect("stack in empty");
    stack.push_back(e);
}

fn reverse_rotate(stack: &mut LinkedList<i32>) {
    let e = stack.pop_back().expect("stack in empty");
    stack.push_front(e);
}

fn push(stack1: &mut LinkedList<i32>, stack2: &mut LinkedList<i32>) {
    let e = stack1.pop_front().expect("list in empty");
    stack2.push_front(e);
}

fn rr(stack1: &mut LinkedList<i32>, stack2: &mut LinkedList<i32>) {
    rotate(stack1);
    rotate(stack2);
}

fn rrr(stack1: &mut LinkedList<i32>, stack2: &mut LinkedList<i32>) {
    reverse_rotate(stack1);
    reverse_rotate(stack2);
}

fn swap(stack: &mut LinkedList<i32>) {
    let a = stack.pop_front().expect("stack have less than 2 elements");
    let b = stack.pop_front().expect("stack have less than 2 elements");
    stack.push_front(a);
    stack.push_front(b);
}

fn ss(stack1: &mut LinkedList<i32>, stack2: &mut LinkedList<i32>) {
    swap(stack1);
    swap(stack2);
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
            push(stack_a, &mut stack_b);
            if num < min {
                moves.push(Moves::Rb);
                rotate(&mut stack_b);
            }
            min += 1;
            max += 1;
        } else {
            if stack_a.len() > 2 {
                let (index, _) = get_min_element(&stack_a);
                if index < stack_a.len() / 2 {
                    moves.push(Moves::Ra);
                    rotate(stack_a);
                } else {
                    moves.push(Moves::Rra);
                    reverse_rotate(stack_a);
                }
            } else {
                moves.push(Moves::Pb);
                push(stack_a, &mut stack_b);
            }
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
            x.cmp(&y)
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
            x.cmp(&y)
        })
        .unwrap();
    (index, *value)
}

fn sort(mut stack_b: LinkedList<i32>) -> Vec<Moves> {
    let mut stack_a = LinkedList::new();
    let mut moves = Vec::new();
    while stack_b.len() != 0 {
        let (index, _) = get_max_element(&stack_b);
        if index < stack_b.len() / 2 {
            for _ in 0..index {
                moves.push(Moves::Rb);
                rotate(&mut stack_b);
            }
            moves.push(Moves::Pa);
            push(&mut stack_b, &mut stack_a);
        } else {
            for _ in 0..stack_b.len() - index {
                moves.push(Moves::Rrb);
                reverse_rotate(&mut stack_b);
            }
            moves.push(Moves::Pa);
            push(&mut stack_b, &mut stack_a);
        }
    }
    moves
}

fn get_the_interval(size: usize) -> i32 {
    (size * 21 / 400 + 35 / 4) as i32
}

fn main() {
    let args: Vec<String> = args().collect();

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

    let max = get_the_interval(stack_a.len());
    let (stack_b, mut moves) = chunk(&mut stack_a, 0, max);
    // dbg!(&stack_b);
    // moves.iter().for_each(|s| println!("{}", s));
    // dbg!(moves.len());
    moves.append(&mut sort(stack_b));
    // dbg!(moves.len());
    moves.iter().for_each(|s| println!("{}", s));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rotate_function() {
        let mut stack = LinkedList::from([1, 2, 3]);
        rotate(&mut stack);
        assert_eq!(stack, LinkedList::from([2, 3, 1]));
        rotate(&mut stack);
        assert_eq!(stack, LinkedList::from([3, 1, 2]));
        rotate(&mut stack);
        assert_eq!(stack, LinkedList::from([1, 2, 3]));
    }

    #[test]
    #[should_panic]
    fn rotate_panic_function() {
        let mut stack = LinkedList::from([]);
        rotate(&mut stack);
    }

    #[test]
    fn rr_function() {
        let mut stack_a = LinkedList::from([1, 2, 3]);
        let mut stack_b = LinkedList::from([4, 5, 6]);
        rr(&mut stack_a, &mut stack_b);
        assert_eq!(stack_a, LinkedList::from([2, 3, 1]));
        assert_eq!(stack_b, LinkedList::from([5, 6, 4]));
        rr(&mut stack_a, &mut stack_b);
        assert_eq!(stack_a, LinkedList::from([3, 1, 2]));
        assert_eq!(stack_b, LinkedList::from([6, 4, 5]));
        rr(&mut stack_a, &mut stack_b);
        assert_eq!(stack_a, LinkedList::from([1, 2, 3]));
        assert_eq!(stack_b, LinkedList::from([4, 5, 6]));
    }

    #[test]
    fn reverse_rotate_function() {
        let mut stack = LinkedList::from([1, 2, 3]);
        reverse_rotate(&mut stack);
        assert_eq!(stack, LinkedList::from([3, 1, 2]));
        reverse_rotate(&mut stack);
        assert_eq!(stack, LinkedList::from([2, 3, 1]));
        reverse_rotate(&mut stack);
        assert_eq!(stack, LinkedList::from([1, 2, 3]));
    }

    #[test]
    #[should_panic]
    fn reverse_rotate_panic_function() {
        let mut stack = LinkedList::from([]);
        reverse_rotate(&mut stack);
    }

    #[test]
    fn rrr_function() {
        let mut stack_a = LinkedList::from([1, 2, 3]);
        let mut stack_b = LinkedList::from([4, 5, 6]);
        rrr(&mut stack_a, &mut stack_b);
        assert_eq!(stack_a, LinkedList::from([3, 1, 2]));
        assert_eq!(stack_b, LinkedList::from([6, 4, 5]));
        rrr(&mut stack_a, &mut stack_b);
        assert_eq!(stack_a, LinkedList::from([2, 3, 1]));
        assert_eq!(stack_b, LinkedList::from([5, 6, 4]));
        rrr(&mut stack_a, &mut stack_b);
        assert_eq!(stack_a, LinkedList::from([1, 2, 3]));
        assert_eq!(stack_b, LinkedList::from([4, 5, 6]));
    }

    #[test]
    fn push_function_1() {
        let mut stack_a = LinkedList::from([1, 2, 3]);
        let mut stack_b: LinkedList<i32> = LinkedList::new();
        push(&mut stack_a, &mut stack_b);
        assert_eq!(stack_a, LinkedList::from([2, 3]));
        assert_eq!(stack_b, LinkedList::from([1]));
    }

    #[test]
    fn push_function_2() {
        let mut stack_a = LinkedList::from([1, 2, 3]);
        let mut stack_b = LinkedList::from([5, 6, 7]);
        push(&mut stack_a, &mut stack_b);
        assert_eq!(stack_a, LinkedList::from([2, 3]));
        assert_eq!(stack_b, LinkedList::from([1, 5, 6, 7]));
    }

    #[test]
    #[should_panic]
    fn push_panic_function() {
        let mut stack_a = LinkedList::from([]);
        let mut stack_b = LinkedList::from([]);
        push(&mut stack_a, &mut stack_b);
    }

    #[test]
    fn swap_function() {
        let mut stack = LinkedList::from([1, 2, 3]);
        swap(&mut stack);
        assert_eq!(stack, LinkedList::from([2, 1, 3]));
        swap(&mut stack);
        assert_eq!(stack, LinkedList::from([1, 2, 3]));
    }

    #[test]
    #[should_panic]
    fn swap_panic_function() {
        let mut stack = LinkedList::from([]);
        swap(&mut stack);
    }

    #[test]
    #[should_panic]
    fn swap_panic_function_2() {
        let mut stack = LinkedList::from([1]);
        swap(&mut stack);
    }

    #[test]
    fn ss_function() {
        let mut stack_a = LinkedList::from([1, 2, 3]);
        let mut stack_b = LinkedList::from([4, 5, 6]);
        ss(&mut stack_a, &mut stack_b);
        assert_eq!(stack_a, LinkedList::from([2, 1, 3]));
        assert_eq!(stack_b, LinkedList::from([5, 4, 6]));
        ss(&mut stack_a, &mut stack_b);
        assert_eq!(stack_a, LinkedList::from([1, 2, 3]));
        assert_eq!(stack_b, LinkedList::from([4, 5, 6]));
    }
}
