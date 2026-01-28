use splits::split_at_mut;

fn main() {
    let mut data = [1, 2, 3, 4, 5];
    let (a, b) = split_at_mut(&mut data, 2);
    println!("{:?} {:?}", a, b);
}
