fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (val1, val2) = pair;

    return (val2, val1);
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    let tuples_or_tuple = ((1u32, 10, "Takam"), ("John", 1));

    let val = reverse((23, false));

    println!("First values is {} ", long_tuple.0);
    println!("Last values is {} ", long_tuple.11);
    println!("Last values is {:?} ", tuples_or_tuple.0);
    println!("Last values is {:?} ", val.1);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    // println!("{:?}", transpo matrix);
}
