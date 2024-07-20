fn main() {
    // variables are immutable by default
    let x = 5;
    println!("the of x is : {}", x);
    let x = 6;
    println!("the of x is : {}", x);

    const GOALS_COUNT: u32 = 1_00_0;

    // Integers

    let a = 98_222; // decimal
    let b = 0xff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; // Byte (u8)only

    let f: u8 = 255; // 256 is 0

    // Compound Types
    // tupls
    let tup = ("hello", 100_000);
    let (text, number) = tup;
    let counter = tup.1;

    // arrays in rust are fixed size
    let error_codes = [200, 404, 500];
    let not_found = error_codes[0];

    let byte = [0; 8]; // create and array with 8 value set to 0

    // function call
    let sum = my_function(1, 2);
    println!("sum number et {}", sum);

    // control flow if statement
    if sum < 5 {
        println!("small");
    } else if sum > 5 {
        println!(" big ");
    } else {
        println!("equal ");
    }

    // if can be put it in a statement
    let sum = if sum > 2 { 1 } else { 3 };
    let mut counter = 0;
    let result = loop {
        if counter == 10 {
            break counter;
        } else {
            counter += 1;
        }
    };

    println!( "the result est {}",result);

    let mut number = 3; 
    while number !=0 {
        println!("{}!",number);
        number -=1;
    }

    println!("LIFTOFF!!!!!!");


        let a = [10,20,30,40,50];

        for elem in a.iter() { // loop through array 
            println!("the value is {}",elem);
        }

        for number in 1..4 {
            println!("{}!",number);
        }


}

// function declaration

fn my_function(x: i32, y: i32) -> i32 {
    println!("the value of x is : {}", x);
    println!("the value of y is : {}", y);
    x + y
}
