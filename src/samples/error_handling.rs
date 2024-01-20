fn main() {
    //*Panic handler */
    // panic!("Farewell");
    // let v = vec![0, 1, 2, 3];
    // println!("{}", v[6]); // this will cause a panic!

    //*Option handler */
    // enum Option<T> {
    //     None,   //The value doesn't exist
    //     Some(T),//The value exists
    // }

    // let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    // for &index in [0, 2, 99].iter() {
    //     match fruits.get(index) {
    //         Some(&"coconut") => println!("Coconuts are awesome!!!"),
    //         Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
    //         None => println!("There is no fruit! :("),
    //     }
    // }

    // //*If let */
    // let a_number: Option<u8> = Some(7);
    // if let Some(7) = a_number {
    //     println!("That's my lucky number!");
    // }

    println!("{:?}", safe_division(9.0, 3.0));
    println!("{:?}", safe_division(4.0, 0.0));
    println!("{:?}", safe_division(0.0, 2.0));
}

//* Result */
// enum Result<T, E> {
//     Ok(T), //?A value T was obtained.
//     Err(E) //? An error of type E was encountered instead.
// }

#[derive(Debug)]
struct DivisionByZeroError;

fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
    if divisor == 0.0 {
        Err(DivisionByZeroError)
    } else {
        Ok(dividend / divisor)
    }
}
