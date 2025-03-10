fn main() {

    let mut s = String::from("Hello");

    s.push_str(", world!");     //push_str() appends a literal to a String
    
    println!("{s}");

    //Shared pointer to the heap memory for the actual characters
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

}
