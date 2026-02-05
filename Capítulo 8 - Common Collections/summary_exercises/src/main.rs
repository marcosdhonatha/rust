fn main() {
    // println!("{}", exe1());
    exe1_mode();
}

fn exe1() -> i32 {
    // 1. Given a list of integers, use a vector and return the median (when
    // sorted, the value in the middle position) and mode (the value that
    // occurs most often; a hash map will be helpful here) of the list.

    let mut integers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    integers.sort();

    let mut vec: Vec<i32> = Vec::new();
    for i in integers {
        vec.push(i);
    }
    if vec.len() % 2 == 0 {
        return (vec[(vec.len() / 2) - 1] + vec[vec.len() / 2]) / 2;
    } else {
        return (vec[(vec.len() / 2)]);
    }
}

fn exe1_mode() {
    use std::collections::HashMap;

    let mut integers = [1, 2, 3, 4, 5, 6, 7, 3, 8, 9, 10, 3, 11, 12, 13, 14, 2, 3];

    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in integers {
        *map.entry(i).or_insert(0) += 1;
    }

    let mut max: i32 = 0;
    let mut mode: i32 = 0;
    for (&key, &value) in &map {
        if value > max {
            max = value;
            mode = key;
        }
    }
    println!("Key: {} Count: {:?}", mode, max);
}
