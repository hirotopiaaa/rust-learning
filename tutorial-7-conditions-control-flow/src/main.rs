fn main() {
    let cond = (2 as f32) < 3.3;
    println!("{}", cond);

    let food = "cookie";
    if food == "cookie" {
        println!("I like cookies");
    } else if food == "cake" {
        println!("I like cake");
    } else {
        println!("I don't like cookies or cake");
    }
}
