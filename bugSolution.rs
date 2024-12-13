fn main() {
    let mut x = 5;
    { //Creating a new scope to use mutable borrow of x
        let y = &mut x; 
        *y = 10;
    }
    { //Creating a new scope to use mutable borrow of x
        let z = &mut x;
        *z = 15; 
    }
    println!("x = {}", x);
}