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
}
