use std::collections::{BTreeMap, BTreeSet};
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

    fn try_left(mut self, k: Ball) -> Option<Self> {
        if self.left.len() == self.limit {
            None
        } else {
            self.left.push(k);
            Some(self)
        }
    }

    fn try_right(mut self, k: Ball) -> Option<Self> {
        if self.right.len() == self.limit {
            None
        } else {
            self.right.push(k);
            Some(self)
        }
    }

    pub fn push_all(s: TwoBoxes, k: Ball) -> Vec<TwoBoxes> {
        let mut ans = vec![];
        if let Some(left) = s.clone().try_left(k) {
            ans.push(left);
        }
        if let Some(right) = s.try_right(k) {
            ans.push(right);
        }
        ans
    }

    pub fn push_all_with_operations(
        (key, tb): (String, TwoBoxes),
        k: Ball,
    ) -> Vec<(String, TwoBoxes)> {
        let mut ans = vec![];
        if let Some(left) = tb.clone().try_left(k) {
            ans.push((format!("{key}L"), left));
        }

        if let Some(right) = tb.clone().try_right(k) {
            ans.push((format!("{key}R"), right));
        }
        ans
    }
}

fn count_how_many_with_limit(v: &[Ball]) {
    let limit = v.len() / 2;
    count_how_many(v, limit)
}

fn count_how_many_without_limit(v: &[Ball]) {
    let limit = v.len();
    count_how_many(v, limit)
}

fn count_how_many(v: &[Ball], limit: usize) {
    let mut cooking = BTreeSet::new();
    cooking.insert(TwoBoxes::new(limit));

    for b in v {
        cooking = cooking
            .into_iter()
            .flat_map(|tb| TwoBoxes::push_all(tb, *b))
            .collect::<BTreeSet<TwoBoxes>>();
    }

    println!("limit: {}, v: {:?}, ans: {}", limit, v, cooking.len())
}

fn main() {
    println!("||| Counting |||");
    count_how_many_with_limit(&['A', 'B', 'C', 'A', 'B', 'C']);
    count_how_many_with_limit(&['A', 'B', 'C', 'A', 'B', 'C', 'A', 'B', 'C', 'A', 'B', 'C']);

    println!("||| capped at half |||");
    for i in 1..=10 {
        count_how_many_with_limit(&['A', 'B'].repeat(i));
    }

    println!("||| not capped at half |||");
    for i in 1..=20 {
        count_how_many_without_limit(&['A', 'B'].repeat(i));
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
