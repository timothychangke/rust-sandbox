fn main() {
    // Exercise 5
    let x: i32 = 5;
    {
        let x: i32 = 12;
        // X's value is the value in the inner scope
        asset_eq!(x, 12);
    }
    // X's value is the value in the outer scope
    assert_eq!(x, 5);
    
    let x = 42;
    println!("{}", x); // This prints 42 as the variable x is shadowed and redeclared
    
    // Exercise 7
    let mut x: i32 = 1
    x = 7
    // Shadowing and re-binding x
    let x = X
    // This line will error as the redeclared x is immutable
    x += 1

    let y: i32 = 4
    // Shadowing allows for the same variable to be assign a value of a different type
    let y : &str = "test"

}