use crate::*;
use std::cmp::Ordering;

pub fn selectionsort(a: &mut impl Sort) {
    for i in 0..a.ln() - 1 {
        let mut min = i;
        for j in i + 1..a.ln() {
            if a.cmp(j, min) == Ordering::Less {
                min = j
            }
        }
        if min != i {
            a.swp(i, min);
        }
    }
}

#[test]
fn test_selection() {
    let mut v = Counter::new(vec![3, 5, 1, 4, 7, 9, 2, 11]);
    selectionsort(&mut v);
    assert_eq!(
        v,
        Counter {
            cmps: 28,
            swaps: 4,
            v: vec![1, 2, 3, 4, 5, 7, 9, 11]
        }
    );
}
