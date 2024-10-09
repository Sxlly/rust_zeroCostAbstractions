// Shae Sullivan
// 20643731
// Programming Languages - Assignment
// 17/09/24


fn main() { //declare main method
    
    let values = 1..=1000;
    let result: Vec<_> = values.into_iter()
        .filter(|&num| num % 2 == 0)
        .map(|num| num * num)
        .collect();

    println!("{:?}", &result[..5]);
}
