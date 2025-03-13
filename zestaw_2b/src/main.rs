/*
Zestaw 2b
1. Wyświetl tabelę widzialnych znaków ASCII wraz kodami (od 33 do 126).

2. Napisz funkcję, która dla danego całkowitego dodatniego n
 zwraca numer iteracji, w której osiągamy jedynkę w problemie Collatza (np. dla n=12
 wynikiem jest 9
).

3. Napisz funkcję, która odpowiada na pytanie, czy jej argument jest liczbą Armstronga.

4. Napisz funkcję, która odpowiada na pytanie, czy jej argument jest liczbą doskonałą.

5. Napisz funkcję, która wyświetli rozkład podanej liczby na czynniki pierwsze.

6. Napisz funkcję pow_mod(x: u128, n: u128, p: u128) -> u128 która obliczy (xn)%p
 w taki sposób, by działało to prawidłowo dla jak największych danych.
    - Wskazówka 1: skorzystaj z własności reszty z dzielenia dla iloczynu (czy też inaczej: iloczynu modulo).
    - Wskazówka 2: w celu ewentualnej optymalizacji czasowej użyj algorytmu szybkiego potęgowania.
*/

/*
Stos i sterta (kopiec):

Stos:
+ dane o znanym i stałym rozmiarze

Sterta:
+ dane tworzone dynamicznie np. z C++ new() i trzeba je samemu usuwać delete()
+ alokacja pamięci na stercie jest wolniejsza niż na stosie

Zasady właśności:
+ każda wartość w Rust ma właściciela
+ w danym momencie może być tylko jeden właściciel
+ gdy właściciel wyjdzie poza zakres, wartość zostanie usunięta
*/

fn main() {
    // print_ascii();
    #[allow(unused)]
    {
        // s jeszcze nie jest zadeklarowane
        let s = String::from("Hello"); // s obowiązuje od tego miejsca
                                       // Zrób coś z s
    } // Zakres się skończył, s nie jest już dostępne
      // Wartość zostanie usunięta (wywołana zostanie funkcja drop)
      // Wzorzec RAII (Resource Acquisition Is Initialization)

    // Kopiowanie i przenoszenie
    let x = 5;
    let y = x; // Wykonała się kopia, bo typy proste są kopiowalne
    println!("x={x}, y={y}"); // x=5, y=5

    let s1 = "Hello".to_string();
    //let s2 = s1; // Wykonało się przeniesienie, bo typy złożone są przenoszone
    // println!("s1={s1}, s2={s2}"); // s1=, s2=Hello; nie da się, wartość przeniesiona
    let s3 = s1.clone(); // Wykonała się kopia, bo wywołano metodę clone
    println!("s1={s1}, s3={s3}"); // s1=Hello, s3=Hello; Jawna głęboka kopia

    // Typy przechowowywane na stosie z autoamtycznym kopiowaniem
    // + typy proste: i32, f64, bool, char, ...
    // + tablice o stałym rozmiarze
    // + krotki, jeśli ich wszystkie elementy są kopiowalne np. krotka ze String nie jest kopiowalna

    // Tablica -- niemodyfikowalny rozmiar, jeden typ danych
    let arr = ["poniedziałek", "wtorek", "środa", "czwartek", "piątek"];
    //println!("arr={arr}") // Brak implementacjji println! dla tablicy
    println!("arr={arr:?}");
    // [typ: rozmiar] = [wartość; rozmiar]
    let arr: [i8; 3] = [10; 3];
    for i in 0..=2
    /*3*/
    {
        println!("arr[{i}]={}", arr[i]);
    }

    // Krotka - grupuje wartości różnych typów w jeden obiekt złożony
    let tup: (i8, f64, i32) = (1, 1.1415, 123456);
    println!("tup = {tup:?}");
    // Możemy użyć dopasowywania wzorców, aby zdestrukturyzować wartość krotki
    let (_a, b, _c) = tup;
    println!("b = {b}");
    println!("tup.2 = {}", tup.2);

    // Własność a funckje ---------------------------------
    let s1 = String::from("ownership");
    takes_ownership(s1); // wartość s1 została przeniesiona do funkcji
                         // println!("s1={s1}"); // błąd, s1 nie jest dostępne
    let x = 5;
    makes_copy(x); // wartość x została skopiowana do funkcji, bo i32 jest kopiowalny
    dbg!(x); // x=5

    let s1 = give_ownership(); // funckja przenosi swoją wartość zwracaną do s1
    println!("s1={s1}");

    let s2 = takes_and_gives(s1); // wartość s1 została przeniesiona do funkcji, a następnie zwrócona
                                  // dbg!(s1); // błąd, s1 nie jest dostępne
    println!("s2={s2}");

    //let length = calculate_length(s2); // wartość s2 została przeniesiona do funkcji
    //println!("Długość {s2} = {length}"); // s2 nie jest dostępne

    let length = calculate_length(s2.clone()); // wartość s2 została skopiowana do funkcji
    println!("Długość {s2} = {length}");

    let (s2, length) = calculate_length2(s2); // wartość s2 została przeniesiona do funkcji, a następnie zwrócona
    println!("Długość {s2} = {length}");

    // Referencje i wypożyczenia ---------------------------
    let length = calculate_length_ref(&s2); // s2 zostało wypożyczone do funkcji
    println!("Długość ref {s2} = {length}");

    // Zasady referencji
    // + możemy mieć tylko jedną referencję mutowalną
    // lub wiele referencji niemutowalnych
    let mut s1 = String::from("tekst");
    let r1 = &mut s1;
    // let r2 = &mut s1; // błąd, bo mamy już referencję mutowalną
    println!("r1={r1}");

    let r1 = &s1;
    let r2 = &s1;
    println!("r1={r1}, r2={r2}");

    let r3 = &mut s1;
    // println!("r1={r1}, r2={r2}, r3={r3}"); // błąd
    println!("r3={r3}");

    change(&mut s1);
    println!("s1={s1}");

    // -------------------------

    let n = 12;
    println!("{n}, liczba iteracji: {}", collatz(n));

    // Z3
    let n = 153;
    println!("{n} armstrong?: {}", armstrong(n));

    // Z5
    let n = 49 * 3 * 17;
    println!("Rozkład {n}:");
    //rozklad(n);
}

// Z2
fn collatz(mut n: i32) -> i32 {
    let mut steps = 0;
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        steps += 1;
    }
    steps
}

// Z3
fn armstrong(x: i32) -> bool {
    let n = x.ilog10() + 1;
    let mut tmp = x;
    let mut sum = 0;
    while tmp > 0 {
        sum += (tmp % 10).pow(n);
        tmp /= 10;
    }
    sum == x
}

// Z5
// fn rozklad(){}

fn takes_ownership(s: String) {
    println!("s w funkcji: {s}");
}

fn makes_copy(z: i32) {
    println!("z w funkcji: {z}");
}

fn give_ownership() -> String {
    let s = "z funkcji".to_string();
    s
}

fn takes_and_gives(s: String) -> String {
    s
}

fn calculate_length(s: String) -> usize {
    s.len()
}

fn calculate_length2(s: String) -> (String, usize) {
    let l = s.len();
    (s, l)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

/*
fn dangle() -> &String {
    let s = String::from("dangled");
    &s // Zwracamy referencję do zaraz usuniętej pamięci
} // wartośc s zostanie usunięta z pamięci
*/

fn change(s: &mut String) {
    s.push_str(" ze zmianą");
}

// Z1
#[allow(unused)]
fn print_ascii() {
    for z in 33 as char..=126 as char {
        println!("{}: {}", z, z as u8);
    }
}
