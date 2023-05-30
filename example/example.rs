use stalin_sort::stalin_sort;

fn main() {
    let mut array: Vec<u8> = vec![5, 3, 4, 7, 8, 10, 45, 9, 43, 98, 40];

    array = stalin_sort(array, true);

    println!("{:02X?}", array);
}
