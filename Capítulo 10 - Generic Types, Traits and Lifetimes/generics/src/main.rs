fn main() {
    //Removing Duplication by Extracting a Function
    //Generic Data Types

    //In Function Definitions:
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    methods();
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
//In Struct Definitions:
fn struct_definitions() {
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
//In Enum Definitions:
fn enums() {
    enum Option<T> {
        Some(T),
        None,
    }
}
//In Method Definitions:

fn methods() {
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    ////////////////////////////////////////////////

    struct Point2<X1, Y1> {
        x: X1,
        y: Y1,
    }

    impl <X1, Y1> Point2<X1, Y1> {
        fn mixup<X2, Y2>(self, another:Point2<X2, Y2>)->Point2<X1, Y2>{

            Point2 { x: self.x, y: another.y }
        }
        
    }
    let p1 = Point2{
        x:5, 
        y:10.4
    };
    let p2 = Point2{x:"Hello", y:'c'};
let p3 = p1.mixup(p2);
println!("p3.x = {}, p3.y= {}", p3.x, p3.y);
}
//Performance of Code Using Generics:
// Rust compiles generic code into code that specifies the
// type in each instance, we pay no runtime cost for using generics. When
// the code runs, it performs just as it would if we had duplicated each defi-
// nition by hand. The process of monomorphization makes Rust’s generics
// extremely efficient at runtime.