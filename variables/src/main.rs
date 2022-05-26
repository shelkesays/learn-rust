fn main() {
    let tup = (500, 6.4, 1);

    println!("The first value is: {}", tup.0);
    println!("The second value is: {}", tup.1);
    println!("The third value is: {}", tup.2);

    let (x, y, z) = tup;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let a = [1, 2, 3, 4, 5];

    let b = [3; 5];

    println!("The value of a is {}: ", a[0]);
    println!("The value of b is {}: ", b[4]);
}
