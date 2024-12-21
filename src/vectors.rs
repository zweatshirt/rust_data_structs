/* Vec<T> - commonly used data structure 
* Resizable and dynamic array
* (heap allocated)
*/

pub fn make_vecs() {
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);
    // :? for human readable print style
    println!("{:?}", numbers); 

    // vector macro - shorthand way to init a vector
    let numbers = vec![1, 2, 3, 4, 5];
    println!("From vector macro: {:?}", numbers);

    let mut fruits = 
        vec!["apple", "banana", "orange"];
    fruits.push("grape");
    println!("Fruit vector: {:?}", fruits);
    
    // removing elements:
    let popped_fruit = fruits.pop().unwrap();
    println!("{:?}, removed {:?}", fruits, popped_fruit);
    
    let fruit;
    match fruits.get(5) {
        Some(val) => { fruit = val;
            println!("{} found!", val) 
        },
        None => { 
            println!("Failed to find value from {:?}", fruits); 
            fruit = &"None";
        }
    }

    println!("{}", fruit);
}

pub fn iter_vec() {
    let animals = vec!["dog", "cat", "rabbit"];
    for animal in &animals {
        println!("{}", animal);
    }

    let mut nums = vec![1, 2, 3, 4, 5];
    println!("Original: {:?}", nums);
    for num in &mut nums { // &mut for mutable reference
       *num *= 10;
    } 
    println!("After multiplying: {:?}", nums);

    // preallocate space for a vector to avoid constant realloc
    let mut vec: Vec<i32> = Vec::with_capacity(10);
    for i in 0..10 {
        vec.push(i);
    }
    println!("Vector: {:?} with capacity set at {}", vec, vec.capacity());
}