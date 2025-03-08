// Zestaw 1a
// 1. Napisz program, który wyświetla informację o przestępności danego roku.
// 2. Napisz program, który wyświetla liczbę dni miesiąca na podstawie jego numeru i numeru roku.
// 3. Napisz program służący do konwersji wartości temperatury podanej w stopniach Celsjusza na stopnie w skali Fahrenheita
// F=32+9/5C
// 4. I odwrotnie.
// 5. Napisz program, który dla danych dwóch poprawnych pór jednej doby (w postaci całkowitych godzin, minut
// i sekund) wyświetla różnicę czasów (także w postaci analogicznej trójki, z minutami i sekundami w
// przedziale [0;59])
// 6. Napisz program, który oblicza silnię dla danej liczby.
// 7. Napisz program, który wyświetla cyfry danej liczby całkowitej (od końca).
// 8. Napisz program, który oblicza sumę cyfry danej liczby całkowitej.
// 9. Napisz program, który znajduje wszystkie trójki pitagorejskie o wartościach nie większych niż dana.
// Zakładamy, że 0 < a < b < c.

// Funkcja void
fn is_leap_year(year: i32) -> () {
    if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
        println!("Rok {year} jest przestępny")
    } else {
        println!("Rok {year} nie jest przestępny")
    }
}

fn convert_to_fahrenheit(celsius: f32) -> f32 {
    let fahrenheit = 32.0 + 9.0 / 5.0 * celsius;
    fahrenheit
}

fn convert_to_ceclsius(fahrenheit: f32) -> f32 {
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    celsius
}

fn main() {
    let year = 2024;
    is_leap_year(year);

    let cecl = 13.0;
    println!(
        "{} stopni Celsjusza to {} stopni Fahrenheita",
        cecl,
        convert_to_fahrenheit(cecl)
    );

    let fahr = 55.0;
    println!(
        "{} stopni Fahrenheita to {} stopni Celsjusza",
        fahr,
        convert_to_ceclsius(fahr)
    );

    // Z5
    let h1 = 10;
    let m1 = 20;
    let s1 = 55;

    let h2 = 12;
    let m2 = 40;
    let s2 = 40;

    let t1s = h1 * 3600 + m1 * 60 + s1;
    let t2s = h2 * 3600 + m2 * 60 + s2;

    // Statment(instruction)
    // Expression(wyrażenie)

    // let x = (let z =5); // nie zadziała
    // let x = {let z = 5; z}; // zadziała

    let tdiff = if t1s > t2s { t1s - t2s } else { t2s - t1s };
    let hdiff = tdiff / 3600;
    let mdiff = (tdiff % 3600) / 60;
    let sdiff = tdiff % 60;
    println!("Diff = {hdiff}:{mdiff}:{sdiff}");

    // Pętle:
    // Loop
    let mut i = 0;
    loop {
        if i > 15 {
            break;
        }
        println!("Hi: {i}");
        i += 1;
    }

    // Przypisanei wartości obliczonej w pętli
    let result = loop {
        i += 1;
        if i == 20 {
            break i * 2;
        }
    };
    println!("Result: {result}");

    let mut count = 0;
    'counting_loop: loop {
        println!("Count: {count}");
        let mut rem = 10;
        loop {
            println!("Rem: {rem}");
            if rem == 9 {
                break;
            }
            if count == 2 {
                break 'counting_loop; // Przerywanie zewnętznej pętli z nazwą
            }
            rem -= 1;
        }
        count += 1;
    }

    // Z6
    let n = 5;
    let mut i = 2;
    let mut factorial = 1;
    loop {
        if i > n {
            break;
        }
        factorial *= i;
        i += 1;
    }
    println!("{n}! = {factorial}");

    // To samo z while
    factorial = 1;
    i = 2;
    while i <= n {
        factorial *= i;
        i += 1;
    }
    println!("{n}! = {factorial}");

    // To samo z for
    // Dla każdego elemntu kolekcji
    let range = 2..10;
    println!("{:?}", range);

    factorial = 1;
    for i in 2..n + 1 {
        factorial *= i;
    }
    println!("{n}! = {factorial}");
    // beg..end -> beg, beg+1, beg+2, ..., end-1

    for x in [6, 11, -8, 4] {
        print!("{x} ");
    }
    print!("\n");

    // Z7 + Z8
    let mut d = 12345;
    let mut sum = 0;
    while d > 0 {
        println!("{} ", d % 10);
        sum += d % 10;
        d /= 10;
    }
    println!("Suma={sum}");

    // Z9
    // Wzór: a^2 + b^2 = c^2
    let dana = 15; // MAX

    for a in 1..dana + 1 {
        for b in a + 1..dana + 1 {
            for c in b + 1..dana + 1 {
                if a * a + b * b == c * c {
                    println!("{}^2 + {}^2 = {}^2", a, b, c);
                }
            }
        }
    }

    my_fun();
    fun_with_param(5, 'c');
    let x = 7;
    println!("x = {}", plus_one(x));
    let x = plus_one(x);
    println!("x = {}", x);
}

fn my_fun() {
    println!("Inna funckja");
}

fn fun_with_param(val: i32, label: char) {
    println!("val={val}, label={label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1 // Wyrażenie na końcu ewaluowane jest do i32
}
