use std::collections::HashMap;

/* Hashmaps in Rust -
* Stores key value pairs
* Classic map
* HashMap<K, V>
* O(1) addition + deletion but of course
* buckets can overlap reducing performance
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

    // iterating over hashmap
    for (city, pop) in &population {
        println!("{}'s population is {}", city, pop);
    }

    for (city, pop) in &mut population {
        if *pop < 20_000_000 {
            let orig_pop = *pop;
            *pop = 20_000_000;
            println!("Updated {}'s population from {} to {}", city, orig_pop, pop);
        }
    }

    // iterate over values
    for val in population.values() {
        println!("Population: {}", val)
    }

    // Inserts Delhi if it doesn't exist else inserts 0 as the value
    // Delhi is not the map so add it
    population.entry("Delhi").or_insert(0);

    // Tokyo exists, so insert 0 as the value
    population.entry("Tokyo").or_insert(0);
    println!("{:?}", population);
}

pub fn map_with_cap() {
    let mut scores = HashMap::with_capacity(10);
    for i in 0..10 {
        scores.insert(i, i*10);
    }
    println!("{:?}, capacity: {}", scores, scores.capacity());

    // capacity shows 14 in output instead of 10, why?
    // actual capacity may vary due to internal HashMap algorithm
    // also hash map is unordered data struct
}

/* Student grades tracker challenge */
pub fn map_challenge() {
    /*
    * key: String
    * value: i32
    * 1. program should add grades
    * for multiple students
    * 2. Find specific student's grade
    * 3. be able to update student's grade
    * 4. Remove student
    * 5. Print all students and their grade
     */

    let mut map = HashMap::new();
    map.insert(String::from("Joe"), 90);
    map.insert(String::from("Jim"), 80);
    map.insert(String::from("Alphaeus"), 80);

    // 1
    let student = String::from("Joe");
    let grade = 70;
    update_grade(&student, &grade, &mut map);
    
    // 2
    let student = String::from("Jim");
    let grade = find_grade_for(&student, &map);
    println!("Student {}'s grade is {}", student, grade);

    // 3 same as add_grade in 1
    
    // 4 bye Jim
    remove_student(&student, &mut map);

    // 5
    print_all_grades(&map);
}

fn update_grade(student: &String, grade: &i32, map: &mut HashMap<String, i32>) {
    map.entry(student.to_string()).or_insert(*grade); 
    println!("Updated map: {:?}", map);
}

fn find_grade_for(student: &String, map: &HashMap<String, i32>) -> i32 {
    match map.get(student) {
        Some(&grade) => { 
            return grade;
        }
        None =>
        { 
            println!("Student {} doesn't exist in the map", student);
            return -1;
        }
    }
}

fn remove_student(student: &String, map: &mut HashMap<String, i32>) {
    map.remove(student);
    println!("Bye {}.", student);
}

fn print_all_grades(map: &HashMap<String, i32>) {
    for (student, grade) in map {
        println!("{}'s grade is {}", student, grade);
    }
}



