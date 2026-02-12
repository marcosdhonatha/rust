use std::{fmt::format, iter::Sum};

fn main() {}
//Defining a Trait:

fn def_trait() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    //Implementing a Trait on a Type:

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
//Default Implementations:
fn default_impl() {
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }
    impl Summary for NewsArticle {}
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());
}

//Traits as Parameters:
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

//Trait Bound Syntax:
fn notify1<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// pub fn notify(item1: &impl Summary, item2: &impl Summary) {}        //mucho texto:/
//pub fn notify<T: Summary>(item1: &T, item2: &T) {}                   //OK:)

//Specifying Multiple Trait Bounds with the + Syntax:
pub fn notify2(item: &(impl Summary + std::fmt::Display)) {}
pub fn notify3<T: Summary + std::fmt::Display>(item: &T) {}

//Clearer Trait Bounds with where Clauses:
fn some_function<T: std::fmt::Display + Clone, U: Clone + std::fmt::Debug>(t: &T, u: &U) {}
fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: std::fmt::Display + Clone,
    U: Clone + std::fmt::Debug,
{
    0
}
//Returning Types That Implement Traits:
impl Summary for Tweet {}
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
//Using Trait Bounds to Conditionally Implement Methods:

fn traits_bounds() {
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }


    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }


    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}
