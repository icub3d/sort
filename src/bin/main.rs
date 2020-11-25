use rand::Rng;

use sort::{
    bubblesort::bubblesort, insertionsort::insertionsort, quicksort::quicksort,
    selectionsort::selectionsort, Counter,
};

fn main() {
    let mut rng = rand::thread_rng();

    // These are the array sizes we'll test. We do various sizes so we
    // can see them on the graph.
    let sizes = vec![
        100, 500, 1_000, 5_000, 10_000, 20_000, 30_000, 40_000, 50_000,
    ];

    // These are the algorithms we are going to run.
    let algos: Vec<&dyn Fn(&mut Counter)> =
        vec![&bubblesort, &insertionsort, &selectionsort, &quicksort];
    let names = vec!["bubble", "insertion", "selection", "quick"];

    // Print header
    println!("algo n cmps swaps total");

    for size in sizes {
        let vals: Vec<i32> = (0..size).map(|_| rng.gen_range(0, 20)).collect();
        for (i, algo) in algos.iter().enumerate() {
            let mut counter = Counter::new(vals.clone());
            // Run the alog and print the results.
            (algo)(&mut counter);
            println!(
                "{} {} {} {} {}",
                names[i],
                size,
                counter.cmps,
                counter.swaps,
                counter.cmps + counter.swaps
            );
        }
    }
}
