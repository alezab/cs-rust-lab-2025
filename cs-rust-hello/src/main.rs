fn main() {
    println!("Hello, world!");
    let mut x: i32;
    x = 6;
    println!("x = {} {} {}", x, x, x);
    println!("x = {x}");

    const SECONDS_IN_HOUR: i32 = 60 * 60;
    println!("SECONDS_IN_HOUR = {}", SECONDS_IN_HOUR);

    let x: i32 = x + 1;
    {
        let x: i32 = x * 2;
        println!("W środku x = {}", x);
    }
    println!("x = {x}");

    let spaces = "    ";
    let spaces = spaces.len(); // Zmienna przesłania i zamienia typ
    println!("spaces = {}", spaces);

    /*
     * skalarne:
     *   całkowite:
     *       - i8 u8 -> 8-bitowa liczba całkowita ze znakiem/bez
     *       - i16 u16 -> 16-bitowa liczba całkowita ze znakiem/bez
     *       - i32 u32 -> 32-bitowa liczba całkowita ze znakiem/bez
     *       - i64 u64 -> 64-bitowa liczba całkowita ze znakiem/bez
     *       - i128 u128 -> 128-bitowa liczba całkowita ze znakiem/bez
     *       - isize usize -> zależne od architektury 32/64-bitowa liczba całkowita ze znakiem/bez
     *
     *   zmiennoprzecinkowe:
     *       - f32 -> 32-bitowa liczba zmiennoprzecinkowa
     *       - f64 -> 64-bitowa liczba zmiennoprzecinkowa
     * bool
     * char
     */
    let a = 2;
    let b = 3.0;
    let c: u64 = 4;
    let c = 4u16;
    let c: f32 = 2.0; // jawne zdefiniowanie jako zmiennoprzecinkowa
    let c = 10_000;
    let c = 0xff;
    let c = 0b1111_0000;
    let c = b'A';

    let c = 'A';

    let c: usize = 7;

    let a = 10;
    let b = 5;
    let c = a + b;
    println!("{c}");
    let c = a * b;
    let c = a - b;
    let c = a / b;
    println!("/ = {c}");

    let d = 5.0;
    let c = 11.0 / d;
    println!("Dzielnie zmiennoprzecinkowe = {c}");
    println!("12.0 / 3.0 = {}", 12.0 / 3.0);
    println!("a/b = {}", a / b);
    let a = 7;
    let b = 5;
    if a < b || a == b {
        println!("a < b");
    } else {
        println!("a >= b");
    }

    let mut x = 255u8;
    x += 1;
    println!("x = {}", x);
}
