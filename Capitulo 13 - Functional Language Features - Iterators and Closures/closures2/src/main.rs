use std::thread;
fn main() {
    //Capturing References or Moving Ownership:
    // let list = vec![1, 2, 3];
    // println!("Before defining closure: {:?}", list);
    // let only_borrows = || println!("From closure: {:?}", list);
    // println!("Before calling closure: {:?}", list);
    // only_borrows();
    // println!("After calling closure: {:?}", list);

    // let mut list = vec![1, 2, 3];
    // println!("Before defining closure: {:?}", list);
    // let mut borrows_mutably = || list.push(7);
    // borrows_mutably();
    // println!("After calling closure: {:?}", list);

    // let list = vec![1, 2, 3];
    // println!("Before defining closure: {:?}", list);
    // thread::spawn(move || println!("From thread: {:?}", list))
    // .join()
    // .unwrap();
    main2();
}

fn main2() {
    //Moving Captured Values Out of Closures and the Fn Traits:

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
println!("{:#?}", list);
}
