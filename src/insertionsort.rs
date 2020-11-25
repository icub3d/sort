use crate::*;
use std::cmp::Ordering;

pub fn insertionsort(a: &mut impl Sort) {
    for i in 1..a.ln() {
        let mut j = i;
        while j > 0 && a.cmp(j - 1, j) == Ordering::Greater {
            a.swp(j, j - 1);
            j -= 1;
        }
    }
}

#[test]
fn test_insertion() {
    let mut v = Counter::new(vec![3, 5, 1, 4, 7, 9, 2, 11]);
    insertionsort(&mut v);
    assert_eq!(
        v,
        Counter {
            cmps: 14,
            swaps: 8,
            v: vec![1, 2, 3, 4, 5, 7, 9, 11]
        }
    );
}
