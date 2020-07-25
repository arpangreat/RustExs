fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest_num = list[0];

    for &item in list {
        if item > largest_num {
            largest_num = item;
        }
    }

    largest_num
}

fn main() {
    let number_list = vec![64, 56, 48, 78, 65];

    let result = largest(&number_list);

    println!("The largest number is {:?}", result);

    let char_list = vec!['y', 'o', 'k', 'a'];

    let result = largest(&char_list);

    println!("This is the biggest Char {:?}", result);
}
