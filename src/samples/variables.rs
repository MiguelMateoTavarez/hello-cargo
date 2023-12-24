fn main() {
    //*Variables

    let a_number;

    //Without keyword "mut" we can change its value later
    let mut a_word = "Ten";

    println!("The word is {}", a_word);

    a_word = "One";
    a_number = 10;

    println!("The number is {}", a_number);
    println!("The word is {}", a_word);
}
