// tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    let (ini_param, bool_param) = pair;

    (bool_param, ini_param)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    // a tuple is a collection of values of different types.    

    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);
    
    // values cna be extracted from the tuple using tuple indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // to create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses
    println!("one element typle: {:?}", (5u32, ));
    println!("just an integer: {:?}", (5u32));

    // tuple can be destructured to create binddings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);


    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
}

