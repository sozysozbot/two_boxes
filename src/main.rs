type Ball = char;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct TwoBoxes {
    u: Vec<Ball>,
    v: Vec<Ball>,
    limit: usize,
}

impl TwoBoxes {
    pub fn new(limit: usize) -> TwoBoxes {
        TwoBoxes {
            u: vec![],
            v: vec![],
            limit,
        }
    }
    fn push(&mut self, k: Ball, is_u: bool) -> Option<()> {
        if is_u {
            if self.u.len() == self.limit {
                None
            } else {
                self.u.push(k);
                Some(())
            }
        } else {
            if self.v.len() == self.limit {
                None
            } else {
                self.v.push(k);
                Some(())
            }
        }
    }
    pub fn push_all(s: TwoBoxes, k: Ball) -> Vec<TwoBoxes> {
        let mut ans = vec![];
        let mut try_u = s.clone();
        if let Some(()) = try_u.push(k, true) {
            ans.push(try_u);
        }
        let mut try_v = s.clone();

        if let Some(()) = try_v.push(k, false) {
            ans.push(try_v);
        }
        ans
    }
}

use std::collections::BTreeSet;

fn main() {
    main2(&['A', 'B', 'C', 'A', 'B', 'C']);
    main2(&['A', 'B', 'C', 'A', 'B', 'C', 'A', 'B', 'C', 'A', 'B', 'C']);

    let mut vec = vec![];
    for _ in 0..20 {
        vec.push('A');
        vec.push('B');
        main2(&vec);
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

fn main2(v: &[Ball]) {
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
