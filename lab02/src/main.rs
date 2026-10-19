//P1
fn add_chars_n(mut s: String, c: char, n: usize) -> String {
    let mut i = 0;

    while i < n {
        s.push(c);
        i += 1;
    }

    s
}

//P2
fn add_chars_n_ref(s: &mut String, c: char, n: usize) {
    let mut i = 0;

    while i < n {
        s.push(c);
        i += 1;
    }
}

//P3
fn add_space(s: &mut String, n: usize) {
    let mut i = 0;

    while i < n {
        s.push(' ');
        i += 1;
    }
}

fn add_str(s: &mut String, text: &str) {
    s.push_str(text);
}

fn add_integer(s: &mut String, mut n: u64) {
    let mut place = 1;
    let mut digits = 1;

    while n / place >= 10 {
        place *= 10;
        digits += 1;
    }

    while place > 0 {
        let digit = n / place;

        s.push((b'0' + digit as u8) as char);

        n %= place;
        place /= 10;
        digits -= 1;

        if digits > 0 && digits % 3 == 0 {
            s.push('_');
        }
    }
}

fn add_float(s: &mut String, value: f64, decimals: usize) {
    let mut scale = 1u64;
    let mut i = 0;

    while i < decimals {
        scale *= 10;
        i += 1;
    }

    let mut x = value;

    if x < 0.0 {
        s.push('-');
        x = -x;
    }

    let number = (x * scale as f64 + 0.5) as u64;

    add_integer(s, number / scale);

    if decimals > 0 {
        s.push('.');

        let mut rest = number % scale;
        let mut place = scale / 10;

        while place > 0 {
            let digit = rest / place;

            s.push((b'0' + digit as u8) as char);

            rest %= place;
            place /= 10;
        }
    }
}

fn main() {
    // P1
    let mut s = String::from("");
    let mut i = 0;

    while i < 26 {
        let c = (i as u8 + b'a') as char;
        s = add_chars_n(s, c, 26 - i);
        i += 1;
    }

    println!("P1: {}", s);

    // P2
    let mut s1 = String::from("");
    let mut j = 0;

    while j < 26 {
        let c = (j as u8 + b'a') as char;
        add_chars_n_ref(&mut s1, c, 26 - j);
        j += 1;
    }

    println!("P2: {}", s1);

    // P3
    let mut rezultat = String::from("");

    add_space(&mut rezultat, 43);
    add_str(&mut rezultat, "I love");
    rezultat.push('\n');

    add_space(&mut rezultat, 43);
    add_str(&mut rezultat, "RUST.");
    rezultat.push('\n');
    rezultat.push('\n');

    add_space(&mut rezultat, 5);
    add_str(&mut rezultat, "Most");

    add_space(&mut rezultat, 13);
    add_str(&mut rezultat, "crate");

    add_space(&mut rezultat, 7);
    add_integer(&mut rezultat, 306437968);

    add_space(&mut rezultat, 12);
    add_str(&mut rezultat, "and");

    add_space(&mut rezultat, 6);
    add_str(&mut rezultat, "lastest");

    add_space(&mut rezultat, 10);
    add_str(&mut rezultat, "is");

    rezultat.push('\n');

    add_space(&mut rezultat, 11);
    add_str(&mut rezultat, "downloaded");

    add_space(&mut rezultat, 8);
    add_str(&mut rezultat, "has");

    add_space(&mut rezultat, 14);
    add_str(&mut rezultat, "downloads");

    add_space(&mut rezultat, 6);
    add_str(&mut rezultat, "the");

    add_space(&mut rezultat, 10);
    add_str(&mut rezultat, "version");

    add_space(&mut rezultat, 4);
    add_float(&mut rezultat, 2.038, 3);
    add_str(&mut rezultat, ".");

    println!("\n{}", rezultat);
}
