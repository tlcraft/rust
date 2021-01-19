fn main() {
    let x = 4; //these two lines, using the let keyword, are examples of statements. Statements don’t evaluate to a value.
    let y = {
        let z = three();
        z + 3  //this is an expression with no semi-colon so the value is returned. Expressions do evaluate to a value.
    }; 
    another_function(x, y);
}

fn three() -> i32{
    3
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of y is: {}", x+y);
}