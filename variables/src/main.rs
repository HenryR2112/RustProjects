use std::io;

fn main() {
    let mut x = 5;
    println!("the values of x is: {x}");
    x = 6;
    println!("the values of x is: {x}");

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let x = 5;

    let x = x + 1;

    {
        let x = x *2;
        println!("the values of x in the inner scope is {x}")
    }
    println!("the value of x is {x}");

    let y = 2.01;
    let z: f32 = 12.0561234145125151;

    println!("the value of x is {y}");
    println!("the value of x is {z}");

    let sum = y + z;
    let difference = z-y;
    let product = 4*30;
    let quotient = 336/123;
    let truncated = -5/3;
    let remainder = 43 %5;

    println!("{sum},{difference} ,{product}, {quotient}, {truncated}, {remainder}");

    let t = true;
    let f: bool = false; //explicit typing

    //tuples
    let tup: (i32, f64, u8) = (500, 2.2, 1);
    let (x, y, z) = tup; //unpacking
    println!("{x} {y} {z}");

    //tuple indexing
    let five_hundred = tup.0;
    let two_point_two = tup.1;
    let one = tup.2;

    //arrays
    let a = [1,2,3,4,5];
    let a2: [i32;5] = [1,1,1,1,1];
    let a3 = [3;5];

    let first = a[0];
    let second = a[1];

    new_game()
}

fn new_game() {
    let a = [1, 2, 3, 4, 5];
    let mut counter = a.len();
    loop {
        if counter == 0 {break};
        println!("please enter an array index");
        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("failed to read line");

        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");

        let element = a[index];

        println!("the value of the element at index {index} is: {element}");

        counter = counter - 1;
    }
}