// Refernece pointers - Point to a resource in memory

pub fn run() {
    // Primitive Array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    // with non-primtives, if you assign another variable to piece of data, the first variable
    // will no longer hold the value. You will need to use a reference (&) to point to the
    // resource.

    // Vectors
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("Values: {:?}", (arr1, arr2));

    println!("V Values: {:?}", (&vec1, vec2));
}
