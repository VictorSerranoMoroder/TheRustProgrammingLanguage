
use std::thread;
use std::time;
use std::collections::HashSet;

fn main() {
    //HashSets don't allow duplicates
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    println!("{:?}", greeks);

    //If an element is added the insert method returns a True
    let added_vega = greeks.insert("vega");
    if added_vega {
        println!("Vega Added")
    }

    if !greeks.contains("Kappa") {
        println!("We don't have Kappa");
    }

    //If a value is removed this method returns true
    let removed = greeks.remove("delta");
    if removed {
        println!("delta removed");
    }
    println!("{:?}", greeks);

    //Subsets

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    println!(
        "is {:?} a subset of {:?}? {}",
        _2_8, _1_10,
        _2_8.is_subset(&_1_10)
    );

    //disjoint = no common elements
    println!(
        "is {:?} a disjoint of {:?}? {}",
        _1_5, _6_10,
        _1_5.is_disjoint(&_6_10)
    );

    //Union = items in both sets
    println!(
        "items in either {:?} and {:?} are {:?}",
        _1_5, _6_10,
        _1_5.union(&_6_10)
    );
}
