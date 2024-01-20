struct PointGeneric<T> {
    x: T,
    y: T,
}

struct PointTwoGeneric<T, U>{
    x: T,
    y: U,
}

fn main() {
    //? If the struct have only one generic, we have to send the same datatype for all expected values, if not, we'll received an error
    let boolean = PointGeneric { x: true, y: false };
    let integer = PointGeneric { x: 1, y: 9 };
    let float = PointGeneric { x: 1.7, y: 4.3 };
    let string_slice = PointGeneric { x: "high", y: "low" };

    //? If the struct have more than one generic, we don't have to send the same datatype for all expected values
    let integer_and_boolean = PointTwoGeneric { x: 5, y: false };
    let float_and_string = PointTwoGeneric { x: 1.0, y: "hey" };
    let integer_and_float = PointTwoGeneric { x: 5, y: 4.0 };
    let both_integer = PointTwoGeneric { x: 10, y: 30 };
    let both_boolean = PointTwoGeneric { x: true, y: true };
}
