fn main() {
    let a = [1, 2, 3];
    // create a new empty vector
    let mut v: Vec<i32> = Vec::new();
    // push elements into the vector
    v.push(1);
    v.push(2);
    v.push(3);

    // create a vector with initial values
    let v2 = vec![1, 2, 3];
    // access elements using indexing
    let third: &i32 = &v2[2];
    println!("The third element is {}", third);

    // get() returns an Option<&T>
    // a safe way to access elements
    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // iterate over elements in a vector
    for i in &v2 {
        println!("{}", i);
    }

    // enum with vector
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    match row[0] {
        SpreadsheetCell::Int(value) => println!("Int: {}", value),
        _ => println!("Not an Int"),
    }

    // Part 2: Strings
    // Strings are stored as a collection of UTF-8 encoded bytes
    let s1 = String::new();
    let s2 = "initial contents";
    let s3 = s2.to_string();
    let s4 = String::from("initial contents");

    // append to a string
    let mut s5 = String::from("foo");
    s5.push_str("bar");
    println!("{}", s5);

    // Part 3: Hash Maps
    use std::collections::HashMap;

    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    // populate the hash map
    // move the ownership of the strings to the hash map
    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name = String::from("Blue");
    // get the value from the hash map
    // get() returns an Option<&V>
    // a safe way to access elements because we cannot guarantee that the key exists
    // if the key does not exist, get() returns None
    let score = scores.get(&team_name);

    // iterate over the hash map
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // update a value in the hash map
    scores.insert(String::from("Blue"), 25);
    // insert a value if the key does not exist
    // or_insert() returns a mutable reference to the value
    scores.entry(String::from("Yellow")).or_insert(50);
    // yellow already exists, so the value is not updated
    // or_insert() will not update the value if the key already exists
    scores.entry(String::from("Yellow")).or_insert(60);
    println!("{:?}", scores);

    // advanced hash map example
    // The code effectively counts how many times each word appears in the given text and stores these counts in a `HashMap`. The final `HashMap` is printed to the console, displaying the word counts.
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // get() returns an Option<&V>
        // if the key does not exist, get() returns None
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
