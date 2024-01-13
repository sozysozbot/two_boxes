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

fn main() {
    //  detect_conflict(&['A', 'B', 'C', 'A', 'B', 'C']);
    //  detect_conflict(&['A', 'B', 'C', 'A', 'B', 'C', 'A', 'B', 'C', 'A', 'B', 'C']);

    let mut vec = vec![];
    for _ in 0..=5 {
        vec.push('A');
        vec.push('B');
        detect_conflict(&vec);
    }
    /*
    main2(&['A', 'B']);
    main2(&['A', 'B', 'A', 'B']);
    main2(&['A', 'B', 'A', 'B', 'A', 'B']);
    main2(&['A', 'B', 'A', 'B', 'A', 'B', 'A', 'B']);
    main2(&['A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B']);
    main2(&['A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B']);
    main2(&[
        'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B',
    ]); */
}

fn detect_conflict(v: &[Ball]) {
    let limit = v.len() / 2;
    let mut cooking = BTreeMap::new();
    cooking.insert(TwoBoxes::new(limit), String::new());

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

                if !s1.contains("LLRR") && !operation.contains("LLRR") {
                    println!("collision:\n  {}\n  {}", s1, operation);
                }
            }
        }
    }

    println!("v: {:?}, ans: {}", v, cooking.len())
}
