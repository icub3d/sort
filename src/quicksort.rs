use crate::*;
use std::cmp::Ordering;

pub fn quicksort(a: &mut impl Sort) {
    qsort(a, 0 as isize, a.ln() as isize - 1);
}

fn qsort(a: &mut impl Sort, lo: isize, hi: isize) {
    if lo < hi {
        let p = partition(a, lo, hi);
        qsort(a, lo, p - 1);
        qsort(a, p + 1, hi);
    }
}

fn partition(a: &mut impl Sort, lo: isize, hi: isize) -> isize {
    let mut i = lo;
    for j in lo..hi {
        if a.cmp(j as usize, hi as usize) == Ordering::Less {
            a.swp(i as usize, j as usize);
            i += 1;
        }
    }
    a.swp(i as usize, hi as usize);
    i as isize
}

#[test]
fn test_quick() {
    let mut v = Counter::new(vec![3, 5, 1, 4, 7, 9, 2, 11]);
    quicksort(&mut v);
    assert_eq!(
        v,
        Counter {
            cmps: 19,
            swaps: 16,
            v: vec![1, 2, 3, 4, 5, 7, 9, 11]
        }
    );
}
