mod insertion;

fn main() {
    // selection sort
    let mut slice = [10, 5, 2, 13, 34, 2, 4, 24];

    println!("Original array: {:?}", slice);
    let slice = insertion::sort(&mut slice);
    println!("Sorted array: {:?}", slice);
}
