fn main() {
    //*Conditionals */
    if 1 == 2 {
        println!("True, the number are equal.");
    } else {
        println!("False, the number are not equal");
    }

    let formal = true;

    let greeting = if formal { "Good day to you." } else { "Hey!" };

    println!("{}", greeting);

    let num = 500;
    let out_of_range: bool;
    if num < 0 {
        out_of_range = true;
    } else if num == 0 {
        out_of_range = true;
    } else if num > 512 {
        out_of_range = true;
    } else {
        out_of_range = false;
    }

    println!("{}",out_of_range);
}
