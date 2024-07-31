fn main() {
    const THIS_IS_AN_CONSTANT: i32 = 50;
    // variables are inmmutable by default.
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("The value of constant is: {THIS_IS_AN_CONSTANT}");
    shadowing();
    tuples();
    array();
}

fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        // print 12
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    // print 6
    println!("The value of x shadowing is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
}

fn tuples() {
    let tup: (i32, f64, i32) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
}

fn array() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let first_element = array[0];
    println!("The first element of array is {first_element}")
}
