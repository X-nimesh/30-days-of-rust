#![allow(overflowing_literals)]

type NanoSecond = u64;
type Inch = u64;
type U64 = u64;






fn main() {
  let nanoseconds: NanoSecond = 5 as u64;
   type_casting_ex();
    interference_demo();
}

fn type_casting_ex(){
    let decimal = 65.6321_f32;
    let integer = decimal as u8 ;
    let character = integer as char;
    println!("{} decimal -> int: {}, char: {}",decimal,integer,character);
   println!("1000 as a u16 is: {}", 1000 as u16);
        let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}

fn interference_demo(){
    // Because of the annotation, the compiler knows that `elem` has type u8.
    let elem = 5u8;

    // Create an empty vector (a growable array).
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`).

    // Insert `elem` in the vector.
    //vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
    // TODO ^ Try commenting out the `vec.push(elem)` line

    println!("{:?}", vec);
}


