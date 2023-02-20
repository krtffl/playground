fn main() {
    // mutability and constants
    let mut x = 5;
    println!("the value of x is {x}");

    x = 6;
    println!("the value of x is {x}");

    const SECONDS_IN_A_DAY: i32 = 60 * 60 * 24;

    // shadowing
    let y = 12;

    let y = y + 4;

    {
        let y = y * 2;
        println!("the value of y in the inner scope is {y}");
    }

    println!("the value of y in the outter scope is {y}");

    let spaces = "        ";
    let spaces = spaces.len();

    // won't compile -> cannot mutate a variable type
    // let mut spcs = "   ";
    //spsc = spaces.len();

    let tup: (i32, f64, u8, bool) = (500, 6.1, 1, true);
    let (x, y, z, t) = tup;
    let five_hundred = tup.0;
    let t = tup.3;

    // empty value or empty return type
    // expressions implicitly return the unit value if no other value
    let unit: () = ();

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3, 5]; // [3, 3, 3, 3, 3]

    let first = a[0];
    let second = b[1];

    another_function(24);
}

fn another_function(x: i32) {
    println!("the value of x is {x}");
}
