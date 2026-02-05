fn main() {
    // Storing Lists of Values with Vectors

    //Creating a New Vector
    let v: Vec<i32> = Vec::new();
    let v: Vec<i32> = vec![1, 2, 3];

    //Updating a Vector
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    //Reading Elements of Vectors

    let v: Vec<i32> = vec![1, 2, 3, 4, 5];

    let third = v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    //Iterating Over the Values in a Vector
    for i in v {
        println!("{i}");
    }
    let mut v = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    for i in &mut v {
        *i = *i / 10;
    }
    hash_maps();
}

fn strings() {
    //Creating a New String

    let s = String::new();

    let data = "Initial contents";
    let s = data.to_string();

    // The method also works on a literal directly
    let s = "Initial contents".to_string();

    //We can also use the function String::from to create a String from a string literal.
    let s = String::from("Initial content");

    //we can include any properly encoded data in them, as shown in:
    let hello = String::from("عليكم السالم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("ָׁלֹוםש ");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá"); //BRASSSIL MENTIONED!!!!!! 
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    //Updating a String
    //Appending to a String with push_str and push

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s = String::from("lo");
    s.push('l');

    //Concatenation with the + Operator or the format! Macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    //Indexing into Strings
    //Slicing Strings
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{s}");

    //Methods for Iterating Over Strings
    for c in s.chars() {
        println!("{c}")
    }
    for b in s.bytes() {
        println!("{b}");
    }
    //Strings Are Not So Simple!!!
}

fn hash_maps() {
    use std::collections::HashMap;
    //Storing Keys with Associated Values in Hash Maps
    //Creating a New Hash Map
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //Accessing Values in a Hash Map

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}:{value}");
    }
    //Updating a Hash Map
    //Overwriting a Value
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    //Adding a Key and Value Only If a Key Isn’t Present
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    //Updating a Value Based on the Old Value
let text= "hello world wonderful world";
let mut map: HashMap<&str, i32> = HashMap::new();
for w in text.split_whitespace(){
    let count = map.entry(w).or_insert(0);
    *count +=1;
}
println!("{:?}", map);
}
