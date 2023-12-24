fn main(){
    let formal = "Formal: Goodbye.";
    let casual = "Casual: See you later!";
    goodbye(formal);
    goodbye(casual);

    let num: u32 = 25;
    print!("{} divided by 5 = {}", num, divide_by_five(num));
}

fn goodbye(message: &str) -> bool {
    print!("\n{}", message);
}

fn divide_by_five(num: u32) -> u32 {
    if num == 0 {
        return 0;
    }
    num/5
}