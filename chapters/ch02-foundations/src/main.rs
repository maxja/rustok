use std::convert::TryInto;

fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn main() {
    let a = 10;
    let b: i32 = 20;
    let c = 30i32;
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));
    println!("( a + b ) + ( c + d ) = {}", e);

    let f = a + b;
    println!("a + b = {}", f);

    let one_million: i64 = 1_000_000;
    println!(
        "one_million = {}, one_million ** 2 = {}",
        one_million,
        one_million.pow(2)
    );

    let forty_twos = [42.03, 42f32, 42_f32];
    println!("{:.4e}", forty_twos[0]);

    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12C;

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!(
        "{} in base 16: {:x},\n {} in base 8: {:o},\n {} base 2: {:b}",
        three, three, three_hundred, three_hundred, thirty, thirty
    );

    let unsigned = 42u16;
    let signed = 42i32;
    let unsigned_: i32 = unsigned.try_into().unwrap();

    if i32::from(unsigned) == signed && signed == (unsigned as i32) && unsigned_ == signed {
        println!("unsigned is equal to signed");
    }
}
