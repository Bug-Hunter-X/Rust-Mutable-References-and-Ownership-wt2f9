fn main() {
    let mut x = 5;
    { // creating a new scope for y
        let y = &mut x;
        *y = 10;
    }
    let z = &x;
    println!("{}", *z); // Prints 10
    let z = &mut x;
    *z = 20;
    println!("{}", x); // Prints 20
}

//Another solution for avoiding mutable reference conflict
fn main() {
    let mut x = 5;
    let y = &mut x; 
    *y = 10; 
    drop(y); // dropping y to allow for mutable reference to x again
    let z = &mut x; 
    *z = 20;
    println!("{}", x); // This will print 20
}