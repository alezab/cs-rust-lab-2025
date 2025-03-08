// 1. Napisz program, który wyświetla informację o przestępności danego roku.
// 2. Do domu
// 3. Napisz program służący do konwersji wartości temperatury podanej w stopniach Celsjusza na stopnie w skali Fahrenheita
// F=32+9/5C

fn is_leap_year(year: i32) {
    if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
        println!("Rok {} jest przestępny", year)
    }
    print!("Rok {} nie jest przestępny", year);
}

fn convert_to_fahrenheit(celsius: f32) -> f32 {
    let fahrenheit = 32.0 + 9.0 / 5.0 * celsius;
    fahrenheit
}

fn main() {
    let year = 2024;

    is_leap_year(year);

    let cecl = 100.0;

    let fahrenheit = convert_to_fahrenheit(cecl);
    println!(
        "{} stopni Celsjusza to {} stopni Fahrenheita",
        cecl, fahrenheit
    );
}
