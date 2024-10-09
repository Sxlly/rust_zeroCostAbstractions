// Shae Sullivan
// 20643731
// Programming Languages - Assignment
// 17/09/24


fn main() { //declare main method
    
    let values = 1..=1000; //declare variables of collection of 1000 numbers or values
    let result: Vec<_> = values.into_iter() //declare variable result which is a vector(array) -> feed values into iteration method and store them in result vector
        .filter(|&num| num % 2 == 0) // filter out all values that arent even and dont satisfy 
        .map(|num| num * num) //map these remaining values into a method squaring them
        .collect(); //collect the final squared values and place them in vector

    println!("{:?}", &result[..5]); //print to terminal 
}
