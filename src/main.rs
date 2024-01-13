type Ball = char;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct TwoBoxes {
    left: Vec<Ball>,
    right: Vec<Ball>,
    limit: usize,
}

impl TwoBoxes {
    pub fn new(limit: usize) -> TwoBoxes {
        TwoBoxes {
            left: vec![],
            right: vec![],
            limit,
        }
    }
    fn push(&mut self, k: Ball, is_left: bool) -> Option<()> {
        if is_left {
            if self.left.len() == self.limit {
                None
            } else {
                self.left.push(k);
                Some(())
            }
        } else {
            if self.right.len() == self.limit {
                None
            } else {
                self.right.push(k);
                Some(())
            }
        }
    }
    pub fn push_all(s: TwoBoxes, k: Ball) -> Vec<TwoBoxes> {
        let mut ans = vec![];
        let mut try_left = s.clone();
        if let Some(()) = try_left.push(k, true) {
            ans.push(try_left);
        }
        let mut try_right = s.clone();

        if let Some(()) = try_right.push(k, false) {
            ans.push(try_right);
        }
        ans
    }

    pub fn push_all_with_operations(
        (key, tb): (String, TwoBoxes),
        k: Ball,
    ) -> Vec<(String, TwoBoxes)> {
        let mut ans = vec![];
        let mut try_left = tb.clone();
        if let Some(()) = try_left.push(k, true) {
            ans.push((format!("{key}L"), try_left));
        }
        let mut try_right = tb.clone();

        if let Some(()) = try_right.push(k, false) {
            ans.push((format!("{key}R"), try_right));
        }
        ans
    }
}

use std::collections::BTreeMap;

use std::collections::BTreeSet;

fn count_how_many(v: &[Ball]) {
    let limit = v.len() / 2;
    let mut cooking = BTreeSet::new();
    cooking.insert(TwoBoxes::new(limit));

    for b in v {
        cooking = cooking
            .into_iter()
            .flat_map(|tb| TwoBoxes::push_all(tb, *b))
            .collect::<BTreeSet<TwoBoxes>>();
    }

    println!("v: {:?}, ans: {}", v, cooking.len())
}

fn main() {
    println!("||| Counting |||");
    count_how_many(&['A', 'B', 'C', 'A', 'B', 'C']);
    count_how_many(&['A', 'B', 'C', 'A', 'B', 'C', 'A', 'B', 'C', 'A', 'B', 'C']);

    for i in 1..=10 {
        count_how_many(&['A', 'B'].repeat(i));
    }

    println!();
    println!("------------------------------");
    println!("||| Looking for collisions |||");
    println!("------------------------------");
    detect_conflict(&['A', 'B'].repeat(8));
}

fn detect_conflict(v: &[Ball]) {
    let limit = v.len() / 2;
    let mut cooking = BTreeMap::new();
    cooking.insert(TwoBoxes::new(limit), String::new());

    let mut nontrivial_collisions: Vec<(String, String)> = vec![];

    for b in v {
        let iter = cooking
            .into_iter()
            .flat_map(|(tb, operations)| TwoBoxes::push_all_with_operations((operations, tb), *b));

        cooking = BTreeMap::new();
        for (operation, tb) in iter {
            if let std::collections::btree_map::Entry::Vacant(e) = cooking.entry(tb.clone()) {
                e.insert(operation);
            } else {
                let s1 = cooking.get(&tb).unwrap();

                let mut already_known = false;

                for (u, v) in &nontrivial_collisions {
                    if s1.contains(u)
                        || operation.contains(u)
                        || s1.contains(v)
                        || operation.contains(v)
                    {
                        already_known = true;
                    }
                }

                if !already_known {
                    println!("nontrivial collision:\n  {}\n  {}", s1, operation);
                    nontrivial_collisions.push((s1.clone(), operation.clone()))
                }
            }
        }
    }
}
