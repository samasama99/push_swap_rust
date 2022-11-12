use std::collections::{HashSet, LinkedList};

pub fn parse_args(args: &[String]) -> Option<LinkedList<i32>> {
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
