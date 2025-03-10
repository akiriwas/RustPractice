fn main() {
    //const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let guess: u32 = "42".parse().expect("Not a number!");

    println!("Value = {guess}");

    let tup: (u32, i16, f64) = (1,-2,1.9);
    println!("Tuple = {}, {}, {}", tup.0, tup.1, tup.2);

    let tup2: (u32, u32, u32, u32) = (0,1,2,3);
    println!("Tuple = {}, {}, {}, {}", tup2.0, tup2.1, tup2.2, tup2.3);

    let x_0 = tup2.0;
    let x_1 = tup2.1*10;
    let x_2 = tup2.2+15;
    let x_3 = tup2.3;

    println!("Breakout: {x_0}, {x_1}, {x_2}, {x_3}");

    let a: [i32; 5] = [1,2,3,4,5];
    let b = [0; 50000];

    println!("a[2] = {}", a[2]);
    println!("b[25000] = {}", b[25000]);

    another_function(22);
    print_labeled_measurement(5, 'h');

    //Expression blocks
    let y = {
        let x = 3;
        x + 1
    };

    println!("Function returns {}", ret_function());
    
    println!("plus_one returns {}", plus_one(41));

    let number = 3;
    if number < 5 {
        println!("Number is less than 5");
    } else {
        println!("Number is greater than or equal to 5");
    }

    let condition1 = false;
    let condition2 = false;
    let number = if condition1 { 5 } else { if condition2 { 6 } else { 7 }  };

    println!("cond1 = {}, cond2 = {}, number = {}", condition1, condition2, number);

    // Loop Labels

    let mut count = 0;  // Loop counter
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("Ending count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("number = {number}");
        number -= 1;
    }

    let a = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let mut index = 0;

    while index < 10 {
        println!("The value of index {index} is {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("The value of element is: {element}");
    }

    for number in (1..100) {
        println!("Simple for loop iterator: {number}");
    }

}

fn ret_function() -> i32 {
    //This is equivalent to "return 42;"
    42
}

fn plus_one(x: i32) -> i32 {
    return x+1;
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}

fn another_function(x: i32) {
    println!("Another function!");
    println!("The value of the parameter is {}", x);
}
