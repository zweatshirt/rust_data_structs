use std::collections::HashMap;

/* Hashmaps in Rust -
* Stores key value pairs
* Classic map
* HashMap<K, V>
*/

pub fn make_maps() {
    let mut population = HashMap::new();

    // adding key-val pairs
    population.insert("Tokyo", 34_300_100);
    population.insert("Chicago", 12_000_000);
    population.insert("London", 15_120_000);
    println!("{:?}", population);

    let to_find = "Tokyo";
    match population.get(to_find) {
        Some(&pop) => println!("Population of Tokyo: {}", pop),
        None => println!("Failed to find {} in the map", to_find),
    }

    // updating value
    // and to be super safe use if:
    if population.contains_key(to_find) {
        population.insert(to_find, 30_000_000);
    }
    println!("Changing Tokyo population: {:?}", population);

    // redundant but for clarity
    let to_remove = &to_find;
    population.remove(to_remove);
    println!("Tokyo removed, new map: {:?}", population);
}