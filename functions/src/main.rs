use std::ops::Range;

fn main() {
    println!("Hello, world!");

    //an example of a rust expression versus a statement
    let y = {
        let x = 3;
        x + 1
    };

    let x = five();
    println!("the values of x is {x}");

    println!("the value of y is: {y}");
    another_function(12);

    let num = 7;

    if num < 5 {
        println!("TRUE")
    } else if num % 3 == 0  {
        println!("divisible by 3")
    } else {
        println!("u failed the test")
    }

    //conditioning on an num
    let condition = false;
    let number = if condition{5}else{6};
    println!("the value is {number}");

    //disambiguate loops
    let mut count  = 0;
    'counting_up: loop{
        println!("count = {count}");
        let mut remaining = 10;

        loop{
            if remaining == 9{
                break
            }
            if count == 2{
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count = {count}");

    // while loop example

    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1
    }
    println!("while loop done!");

    //for loop
    let a = [1,2,3,4,5];
    for element in a {
        println!("{element}");
    }

    for i in 1..=5 {
        println!("the range prints these values: {i}")
    }
}


fn another_function(x: i32){
    println!("another function {x}");
}

fn five() -> i32 {
    5
}
