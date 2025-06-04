fn main() {
    println!("Hello, world!");
    another_function(5, 'a');
    tests();
    println!("The value of function five is: {}", five(5));
}

fn another_function(x:i64, y:char){

    println!("The value of x and y are: {x} | {y}");

}

fn tests(){
    let x= {let y=6;
    y*10
};

    println!("The value of x is: {x}");

}

fn five(x:i32)->i32{
    return (x * 5);
}