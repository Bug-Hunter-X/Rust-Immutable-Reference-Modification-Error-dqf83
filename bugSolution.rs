fn main() {
    let mut x = 5;
    let y = &mut x; 
    *y = 10; // Modify x through y
    let z = &mut x; // z is a mutable reference to x
    *z = 20; // This is now allowed
    println!("x = {}", x);
}