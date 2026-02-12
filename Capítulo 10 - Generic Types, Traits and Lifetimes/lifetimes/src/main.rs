fn main() {}

//Validating References with Lifetimes:
fn dangling_references() {
    let r: &i32;

    {
        let x = 5;
        //r= &x;
    }
    //println!("r: {}", r);
}

//Generic Lifetimes in Functions

fn lifetime() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}

//           fn longest(x: &str, y: &str) -> &str {
//               if x.len() > y.len() { x } else { y }
//           }

//Lifetime Annotation Syntax:

//            &i32 // a reference
//            &'a i32 // a reference with an explicit lifetime
//            &'a mut i32 // a mutable reference with an explicit lifetime

//Lifetime Annotations in Function Signatures:

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
//Lifetime Annotations in Struct Definitions:

struct ImportantExcerpt<'a> {
    part: &'a str,
}
fn struct_lifetimes() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
//Lifetime Elision:
// The first rule is that the compiler assigns a lifetime parameter to each
// parameter that’s a reference. In other words, a function with one param-
// eter gets one lifetime parameter: fn foo<'a>(x: &'a i32); a function with two
// parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32,
// y: &'b i32); and so on.
// The second rule is that, if there is exactly one input lifetime parameter,
// that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a
// i32) -> &'a i32.
// The third rule is that, if there are multiple input lifetime parameters,
// but one of them is &self or &mut self because this is a method, the lifetime
// of self is assigned to all output lifetime parameters. This third rule makes
// methods much nicer to read and write because fewer symbols are necessary.

//Lifetime Annotations in Method Definitions:
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        print!("Attention please: {announcement} ");
        self.part
    }
}

//The Static Lifetime:
//   let s: &'static str = "I have a static lifetime.";


