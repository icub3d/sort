use std::cmp::Ordering;

pub mod bubblesort;
pub mod insertionsort;
pub mod quicksort;
pub mod selectionsort;

pub trait Sort {
    // Compare the values at position a and b and return their ordering.
    fn cmp(&mut self, a: usize, b: usize) -> Ordering;

    // Swap the values at positions a and b.
    fn swp(&mut self, a: usize, b: usize);

    // The size of the array.
    fn ln(&self) -> usize;
}

impl Sort for Vec<i32> {
    fn cmp(&mut self, a: usize, b: usize) -> Ordering {
        if self[a] < self[b] {
            Ordering::Less
        } else if self[a] > self[b] {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    fn swp(&mut self, a: usize, b: usize) {
        self.swap(a, b)
    }

    fn ln(&self) -> usize {
        self.len()
    }
}

#[derive(Debug, PartialEq)]
pub struct Counter {
    pub cmps: usize,
    pub swaps: usize,
    v: Vec<i32>,
}

impl Counter {
    pub fn new(v: Vec<i32>) -> Counter {
        Counter {
            cmps: 0,
            swaps: 0,
            v: v,
        }
    }
}

impl Sort for Counter {
    fn cmp(&mut self, a: usize, b: usize) -> Ordering {
        self.cmps += 1;
        if self.v[a] < self.v[b] {
            Ordering::Less
        } else if self.v[a] > self.v[b] {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    fn swp(&mut self, a: usize, b: usize) {
        self.swaps += 1;
        self.v.swap(a, b)
    }

    fn ln(&self) -> usize {
        self.v.len()
    }
}

#[cfg(test)]
mod tests {}
