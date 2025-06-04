fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    println!("test function {}", test());
    test2();
    test3();
    test4();
}

fn test() -> i32 {
    let cond = true;
    let number = if cond { 18 } else { 17 };
    number
}

fn test2() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 10;
        }
    };

    println!("The result is {result}");
}

fn test3() {
    let mut count = 0;
    'count_loop: loop {
        println!("Count:{}", count);
        let mut remaining = 10;

        loop {
            println!("Remaining: {}", remaining);
            if remaining == 0 {
                break;
            }
            if count == 2 {
                break 'count_loop;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
    println!("######################");
}

fn test4() {
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("######################");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("{}", a[index]);
        index += 1;
    }
    println!("######################");
    for element in a {
        println!("{element}")
    }
    println!("######################");
    for number in (1..4).rev() {
        println!("{number}")
    }
}
