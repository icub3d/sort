use crate::*;
use std::cmp::Ordering;

pub fn bubblesort(a: &mut impl Sort) {
    loop {
        let mut swapped = false;
        for i in 1..a.ln() {
            if a.cmp(i - 1, i) == Ordering::Greater {
                a.swp(i - 1, i);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

#[test]
fn test_bubble() {
    let mut v = Counter::new(vec![3, 5, 1, 4, 7, 9, 2, 11]);
    bubblesort(&mut v);
    assert_eq!(
        v,
        Counter {
            cmps: 42,
            swaps: 8,
            v: vec![1, 2, 3, 4, 5, 7, 9, 11]
        }
    );
}
