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

    // P3 - caz de succes
    if let Ok(result) = add_result(10, 20) {
        println!("Rezultat adunare P3: {}", result);
    }

    if let Ok(result) = mul_result(10, 5) {
        println!("Rezultat inmultire P3: {}", result);
    }

    // P3 - eroare la adunare
    if let Err(MyError::Overflow) = add_result(u32::MAX, 1) {
        println!("Overflow la adunare P3!");
    }

    // P3 - eroare la inmultire
    if let Err(MyError::Overflow) = mul_result(u32::MAX, 2) {
        println!("Overflow la inmultire P3!");
    }

    // P3 - functie care propaga erorile
    if let Ok((suma, produs)) = calculate(10, 20) {
        println!("Suma: {}", suma);
        println!("Produsul: {}", produs);
    }

    // Overflow la adunare, propagat prin ?
    if let Err(MyError::Overflow) = calculate(u32::MAX, 2) {
        println!("Eroare propagata de la adunare!");
    }

    // Overflow la inmultire, propagat prin ?
    if let Err(MyError::Overflow) = calculate(100_000, 100_000) {
        println!("Eroare propagata de la inmultire!");
    }
}
