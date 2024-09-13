pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    // Define the shared method signature
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String;
}

// Define a function that takes a type that implements the Summary trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
    println!("Author: {}", item.summarize_author());
}

// Alternatively, use generic type parameters with trait bounds
pub fn notify_generic<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
    println!("Author: {}", item.summarize_author());
}

impl NewsArticle {
    // Factory method to create a new NewsArticle
    pub fn new(headline: String, location: String, author: String, content: String) -> Self {
        Self {
            headline,
            location,
            author,
            content,
        }
    }
}

impl Tweet {
    // Factory method to create a new Tweet
    pub fn new(username: String, content: String) -> Self {
        Self {
            username,
            content,
            reply: false,
            retweet: false,
        }
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let tweet = Tweet::new(
        String::from("horse_ebooks"),
        String::from("of course, as you probably already know, people"),
    );

    let article = NewsArticle::new(
        String::from("Penguins win the Stanley Cup Championship!"),
        String::from("Pittsburgh, PA, USA"),
        String::from("Iceburgh"),
        String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    );

    println!("1 new tweet: {}", tweet.summarize());
    println!("New article available! {}", article.summarize());

    notify(&tweet);
}
