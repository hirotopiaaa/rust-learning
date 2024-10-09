#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // into_iter() consumes the vector and takes ownership of its elements
    // .filter() is a method on the Iterator trait, which takes a closure as an argument, and
    // applies that closure to each item in the iterator, producing a new iterator
    // .collect() consumes the iterator and collects the resulting values into a collection data,
    // in this case a vector
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn main() {
    // Example 1
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for value in v1_iter {
        println!("Got: {}", value);
    }

    let total: i32 = v1.iter().sum();
    println!("Total: {}", total);

    // Example 2
    let v2: Vec<i32> = vec![1, 2, 3];
    // .map() is a method on the Iterator trait, which takes a closure as an argument, and applies
    // that closure to each item in the iterator, producing a new iterator
    // .collect() consumes the iterator and collects the resulting values into a collection data,
    // in this case a vector
    let v2_iter: Vec<_> = v2.iter().map(|x| x + 1).collect();

    assert_eq!(v2_iter, vec![2, 3, 4]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        );
    }
}
