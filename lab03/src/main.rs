fn next_prime(x: u16) -> Option<u16> {
    let mut candidate = x as u32 + 1;

    while candidate <= u16::MAX as u32 {
        let mut prime = false;
        let mut d = 2;

        if candidate >= 2 {
            prime = true;
        }

        while d * d <= candidate {
            if candidate.is_multiple_of(d) {
                prime = false;
                break;
            }
            d += 1;
        }

        if prime {
            return Some(candidate as u16);
        }

        candidate += 1;
    }

    None
}

// P2
fn add(a: u32, b: u32) -> u32 {
    if a > u32::MAX - b {
        panic!("Overflow la adunare!");
    }

    a + b
}

fn mul(a: u32, b: u32) -> u32 {
    if b != 0 && a > u32::MAX / b {
        panic!("Overflow la inmultire!");
    }

    a * b
}

// P3
enum MyError {
    Overflow,
}

fn add_result(a: u32, b: u32) -> Result<u32, MyError> {
    if a > u32::MAX - b {
        Err(MyError::Overflow)
    } else {
        Ok(a + b)
    }
}

fn mul_result(a: u32, b: u32) -> Result<u32, MyError> {
    if b != 0 && a > u32::MAX / b {
        Err(MyError::Overflow)
    } else {
        Ok(a * b)
    }
}

fn calculate(a: u32, b: u32) -> Result<(u32, u32), MyError> {
    let suma = add_result(a, b)?;
    let produs = mul_result(a, b)?;

    Ok((suma, produs))
}
//P4
enum CharError {
    NonAscii,
    NotDigit,
    NotHexDigit,
    NotLetter,
    NotPrintable,
}
fn char_to_number(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NonAscii);
    }

    if !c.is_ascii_digit() {
        return Err(CharError::NotDigit);
    }

    Ok(c as u32 - '0' as u32)
}
fn char_to_number_hex(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NonAscii);
    }

    if !c.is_ascii_hexdigit() {
        return Err(CharError::NotHexDigit);
    }

    if c.is_ascii_digit() {
        Ok(c as u32 - '0' as u32)
    } else if c.is_ascii_uppercase() {
        Ok(c as u32 - 'A' as u32 + 10)
    } else {
        Ok(c as u32 - 'a' as u32 + 10)
    }
}
fn to_uppercase(c: char) -> Result<String, CharError> {
    if !c.is_alphabetic() {
        return Err(CharError::NotLetter);
    }

    let mut result = String::new();

    for ch in c.to_uppercase() {
        result.push(ch);
    }

    Ok(result)
}
fn to_lowercase(c: char) -> Result<String, CharError> {
    if !c.is_alphabetic() {
        return Err(CharError::NotLetter);
    }

    let mut result = String::new();

    for ch in c.to_lowercase() {
        result.push(ch);
    }

    Ok(result)
}
fn print_char(c: char) -> Result<(), CharError> {
    if c.is_control() {
        return Err(CharError::NotPrintable);
    }

    println!("{}", c);
    Ok(())
}
fn print_error(error: CharError) {
    match error {
        CharError::NonAscii => println!("Caracterul nu este ASCII!"),
        CharError::NotDigit => println!("Caracterul nu este o cifra!"),
        CharError::NotHexDigit => println!("Caracterul nu este o cifra hexazecimala!"),
        CharError::NotLetter => println!("Caracterul nu este o litera!"),
        CharError::NotPrintable => println!("Caracterul nu este afisabil!"),
    }
}
//P5
fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        return Err("Nu putem imparti la zero!");
    }

    Ok(a / b)
}
fn main() {
    // P1
    println!("{:?}", next_prime(10));
    println!("{:?}", next_prime(65535));

    let mut x = 65515u16;

    while let Some(p) = next_prime(x) {
        println!("{}", p);
        x = p;
    }

    // P2 - cazuri de succes
    println!("Adunare: {}", add(10, 20));
    println!("Inmultire: {}", mul(10, 5));
    println!("Inmultire cu zero: {}", mul(10, 0));

    // P3
    if let Ok(result) = add_result(10, 20) {
        println!("Rezultat adunare P3: {}", result);
    }

    if let Ok(result) = mul_result(10, 5) {
        println!("Rezultat inmultire P3: {}", result);
    }

    if let Err(MyError::Overflow) = add_result(u32::MAX, 1) {
        println!("Overflow la adunare P3!");
    }

    if let Err(MyError::Overflow) = mul_result(u32::MAX, 2) {
        println!("Overflow la inmultire P3!");
    }

    if let Ok((suma, produs)) = calculate(10, 20) {
        println!("Suma: {}", suma);
        println!("Produsul: {}", produs);
    }

    if let Err(MyError::Overflow) = calculate(u32::MAX, 2) {
        println!("Eroare propagata de la adunare!");
    }

    if let Err(MyError::Overflow) = calculate(100_000, 100_000) {
        println!("Eroare propagata de la inmultire!");
    }
    // P4

    if let Ok(numar) = char_to_number('8') {
        println!("Numarul este: {}", numar);
    }

    if let Err(CharError::NotDigit) = char_to_number('A') {
        println!("Caracterul nu este o cifra!");
    }

    if let Err(CharError::NonAscii) = char_to_number('ă') {
        println!("Caracterul nu este ASCII!");
    }

    if let Ok(numar) = char_to_number_hex('A') {
        println!("Numarul este: {}", numar);
    }

    if let Ok(numar) = char_to_number_hex('f') {
        println!("Numarul este: {}", numar);
    }

    if let Err(CharError::NotHexDigit) = char_to_number_hex('G') {
        println!("Nu este cifra hexazecimala!");
    }

    if let Err(CharError::NonAscii) = char_to_number_hex('ă') {
        println!("Caracterul nu este ASCII!");
    }
    if let Ok(result) = to_uppercase('a') {
        println!("Litera mare: {}", result);
    }

    if let Ok(result) = to_uppercase('ă') {
        println!("Litera mare: {}", result);
    }

    if let Err(CharError::NotLetter) = to_uppercase('7') {
        println!("Caracterul nu este o litera!");
    }
    if let Ok(result) = to_lowercase('A') {
        println!("Litera mica: {}", result);
    }

    if let Ok(result) = to_lowercase('Ă') {
        println!("Litera mica: {}", result);
    }

    if let Err(CharError::NotLetter) = to_lowercase('7') {
        println!("Caracterul nu este o litera!");
    }
    if let Ok(()) = print_char('A') {
        println!("Afisare reusita!");
    }

    if let Err(CharError::NotPrintable) = print_char('\n') {
        println!("Caracterul nu este afisabil!");
    }
    if let Err(error) = char_to_number('A') {
        print_error(error);
    }

    if let Err(error) = char_to_number_hex('G') {
        print_error(error);
    }

    if let Err(error) = to_uppercase('7') {
        print_error(error);
    }

    if let Err(error) = print_char('\n') {
        print_error(error);
    }
    //P5
    if let Ok(rezultat) = divide(20, 4) {
        println!("Rezultatul este: {}", rezultat);
    }

    if let Err(mesaj) = divide(20, 0) {
        println!("Eroare: {}", mesaj);
    }
}
