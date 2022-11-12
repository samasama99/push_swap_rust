pub mod stack_moves {
    use std::{collections::LinkedList, fmt::Display, str::FromStr};

    #[derive(Debug, PartialEq)]
    pub enum Moves {
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

    impl FromStr for Moves {
        type Err = String;

        fn from_str(input: &str) -> Result<Moves, Self::Err> {
            match input {
                "ra" => Ok(Moves::Ra),
                "rb" => Ok(Moves::Rb),
                "raa" => Ok(Moves::Rra),
                "rrb" => Ok(Moves::Rrb),
                "pa" => Ok(Moves::Pa),
                "pb" => Ok(Moves::Pb),
                "sa" => Ok(Moves::Sa),
                "sb" => Ok(Moves::Sb),
                "rr" => Ok(Moves::Rr),
                "rrr" => Ok(Moves::Rrr),
                "ss" => Ok(Moves::Ss),
                _ => Err("not a move".to_string()),
            }
        }
    }

    #[must_use]
    pub fn rotate(stack: &mut LinkedList<i32>) -> Option<()> {
        let num = stack.pop_front()?;
        Some(stack.push_back(num))
    }

    #[must_use]
    pub fn reverse_rotate(stack: &mut LinkedList<i32>) -> Option<()> {
        let num = stack.pop_back()?;
        Some(stack.push_front(num))
    }

    #[must_use]
    pub fn push(stack1: &mut LinkedList<i32>, stack2: &mut LinkedList<i32>) -> Option<()> {
        let num = stack1.pop_front()?;
        Some(stack2.push_front(num))
    }

    #[must_use]
    pub fn rr(stack1: &mut LinkedList<i32>, stack2: &mut LinkedList<i32>) -> Option<()> {
        rotate(stack1)?;
        rotate(stack2)?;
        Some(())
    }

    #[must_use]
    pub fn rrr(stack1: &mut LinkedList<i32>, stack2: &mut LinkedList<i32>) -> Option<()> {
        reverse_rotate(stack1)?;
        reverse_rotate(stack2)?;
        Some(())
    }

    #[must_use]
    pub fn swap(stack: &mut LinkedList<i32>) -> Option<()> {
        let a = stack.pop_front()?;
        let b = stack.pop_front()?;
        stack.push_front(a);
        stack.push_front(b);
        Some(())
    }

    #[must_use]
    pub fn ss(stack1: &mut LinkedList<i32>, stack2: &mut LinkedList<i32>) -> Option<()> {
        swap(stack1)?;
        swap(stack2)?;
        Some(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn rotate_function() {
            let mut stack = LinkedList::from([1, 2, 3]);
            rotate(&mut stack).unwrap();
            assert_eq!(stack, LinkedList::from([2, 3, 1]));
            rotate(&mut stack).unwrap();
            assert_eq!(stack, LinkedList::from([3, 1, 2]));
            rotate(&mut stack).unwrap();
            assert_eq!(stack, LinkedList::from([1, 2, 3]));
        }

        #[test]
        #[should_panic]
        fn rotate_panic_function() {
            let mut stack = LinkedList::from([]);
            rotate(&mut stack).unwrap();
        }

        #[test]
        fn rr_function() {
            let mut stack_a = LinkedList::from([1, 2, 3]);
            let mut stack_b = LinkedList::from([4, 5, 6]);
            rr(&mut stack_a, &mut stack_b).unwrap();
            assert_eq!(stack_a, LinkedList::from([2, 3, 1]));
            assert_eq!(stack_b, LinkedList::from([5, 6, 4]));
            rr(&mut stack_a, &mut stack_b).unwrap();
            assert_eq!(stack_a, LinkedList::from([3, 1, 2]));
            assert_eq!(stack_b, LinkedList::from([6, 4, 5]));
            rr(&mut stack_a, &mut stack_b).unwrap();
            assert_eq!(stack_a, LinkedList::from([1, 2, 3]));
            assert_eq!(stack_b, LinkedList::from([4, 5, 6]));
        }

        #[test]
        fn reverse_rotate_function() {
            let mut stack = LinkedList::from([1, 2, 3]);
            reverse_rotate(&mut stack).unwrap();
            assert_eq!(stack, LinkedList::from([3, 1, 2]));
            reverse_rotate(&mut stack).unwrap();
            assert_eq!(stack, LinkedList::from([2, 3, 1]));
            reverse_rotate(&mut stack).unwrap();
            assert_eq!(stack, LinkedList::from([1, 2, 3]));
        }

        #[test]
        #[should_panic]
        fn reverse_rotate_panic_function() {
            let mut stack = LinkedList::from([]);
            reverse_rotate(&mut stack).unwrap();
        }

        #[test]
        fn rrr_function() {
            let mut stack_a = LinkedList::from([1, 2, 3]);
            let mut stack_b = LinkedList::from([4, 5, 6]);
            rrr(&mut stack_a, &mut stack_b).unwrap();
            assert_eq!(stack_a, LinkedList::from([3, 1, 2]));
            assert_eq!(stack_b, LinkedList::from([6, 4, 5]));
            rrr(&mut stack_a, &mut stack_b).unwrap();
            assert_eq!(stack_a, LinkedList::from([2, 3, 1]));
            assert_eq!(stack_b, LinkedList::from([5, 6, 4]));
            rrr(&mut stack_a, &mut stack_b).unwrap();
            assert_eq!(stack_a, LinkedList::from([1, 2, 3]));
            assert_eq!(stack_b, LinkedList::from([4, 5, 6]));
        }

        #[test]
        fn push_function_1() {
            let mut stack_a = LinkedList::from([1, 2, 3]);
            let mut stack_b: LinkedList<i32> = LinkedList::new();
            push(&mut stack_a, &mut stack_b).unwrap();
            assert_eq!(stack_a, LinkedList::from([2, 3]));
            assert_eq!(stack_b, LinkedList::from([1]));
        }

        #[test]
        fn push_function_2() {
            let mut stack_a = LinkedList::from([1, 2, 3]);
            let mut stack_b = LinkedList::from([5, 6, 7]);
            push(&mut stack_a, &mut stack_b).unwrap();
            assert_eq!(stack_a, LinkedList::from([2, 3]));
            assert_eq!(stack_b, LinkedList::from([1, 5, 6, 7]));
        }

        #[test]
        #[should_panic]
        fn push_panic_function() {
            let mut stack_a = LinkedList::from([]);
            let mut stack_b = LinkedList::from([]);
            push(&mut stack_a, &mut stack_b).unwrap();
        }

        #[test]
        fn swap_function() {
            let mut stack = LinkedList::from([1, 2, 3]);
            swap(&mut stack).unwrap();
            assert_eq!(stack, LinkedList::from([2, 1, 3]));
            swap(&mut stack).unwrap();
            assert_eq!(stack, LinkedList::from([1, 2, 3]));
        }

        #[test]
        #[should_panic]
        fn swap_panic_function() {
            let mut stack = LinkedList::from([]);
            swap(&mut stack).unwrap();
        }

        #[test]
        #[should_panic]
        fn swap_panic_function_2() {
            let mut stack = LinkedList::from([1]);
            swap(&mut stack).unwrap();
        }

        #[test]
        fn ss_function() {
            let mut stack_a = LinkedList::from([1, 2, 3]);
            let mut stack_b = LinkedList::from([4, 5, 6]);
            ss(&mut stack_a, &mut stack_b).unwrap();
            assert_eq!(stack_a, LinkedList::from([2, 1, 3]));
            assert_eq!(stack_b, LinkedList::from([5, 4, 6]));
            ss(&mut stack_a, &mut stack_b).unwrap();
            assert_eq!(stack_a, LinkedList::from([1, 2, 3]));
            assert_eq!(stack_b, LinkedList::from([4, 5, 6]));
        }
    }
}
