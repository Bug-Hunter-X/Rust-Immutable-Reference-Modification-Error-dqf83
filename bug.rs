fn main() {
    let mut x = 5;
    let y = &mut x; 
    *y = 10; // Modify x through y
    let z = &x; // z is an immutable reference to x
    *z = 20; // This will cause a compile-time error
}