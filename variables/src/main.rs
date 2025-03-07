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

}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}

fn another_function(x: i32) {
    println!("Another function!");
    println!("The value of the parameter is {}", x);
}
