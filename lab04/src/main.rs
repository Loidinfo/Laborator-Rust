use std::{fs, io};

fn p1() -> Result<(), io::Error> {
    let s = fs::read_to_string("p1.txt")?;

    let mut max_bytes = 0;
    let mut linia_max_bytes = "";
    let mut max_caractere = 0;
    let mut linia_max_caractere = "";

    for line in s.lines() {
        let bytes = line.len();
        let caractere = line.chars().count();

        if bytes > max_bytes {
            max_bytes = bytes;
            linia_max_bytes = line;
        }

        if caractere > max_caractere {
            max_caractere = caractere;
            linia_max_caractere = line;
        }

        println!("{}: {} bytes, {} caractere", line, bytes, caractere);
    }

    println!("Cea mai lunga linie dupa bytes: {}", linia_max_bytes);
    println!("Numar de bytes: {}", max_bytes);
    println!(
        "Cea mai lunga linie dupa caractere: {}",
        linia_max_caractere
    );
    println!("Numar de caractere: {}", max_caractere);

    Ok(())
}

fn p2() -> Result<(), io::Error> {
    let s = fs::read_to_string("p2.txt")?;

    let mut rezultat = String::new();

    for c in s.chars() {
        if !c.is_ascii() {
            println!("Eroare: caracterul '{}' nu este ASCII!", c);
            return Ok(());
        }

        if ('A'..='M').contains(&c) {
            let litera_noua = (c as u8 + 13) as char;
            rezultat.push(litera_noua);
        } else if ('N'..='Z').contains(&c) {
            let litera_noua = (c as u8 - 13) as char;
            rezultat.push(litera_noua);
        } else if ('a'..='m').contains(&c) {
            let litera_noua = (c as u8 + 13) as char;
            rezultat.push(litera_noua);
        } else if ('n'..='z').contains(&c) {
            let litera_noua = (c as u8 - 13) as char;
            rezultat.push(litera_noua);
        } else {
            rezultat.push(c);
        }
    }

    fs::write("p2_rot13.txt", &rezultat)?;

    println!("{}", rezultat);

    Ok(())
}
fn p3() -> Result<(), io::Error> {
    let s = fs::read_to_string("p3.txt")?;

    let mut rezultat = String::new();

    for cuvant in s.split(' ') {
        if !rezultat.is_empty() {
            rezultat.push(' ');
        }

        if cuvant == "pt" || cuvant == "ptr" {
            rezultat.push_str("pentru");
        } else if cuvant == "dl" {
            rezultat.push_str("domnul");
        } else if cuvant == "dna" {
            rezultat.push_str("doamna");
        } else {
            rezultat.push_str(cuvant);
        }
    }

    fs::write("p3_rezultat.txt", &rezultat)?;

    println!("Textul modificat: {}", rezultat);

    Ok(())
}
fn p4() -> Result<(), io::Error> {
    let s = fs::read_to_string("p4_test.txt")?;

    for line in s.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut coloane = line.split_whitespace();

        if let (Some(ip), Some(nume)) = (coloane.next(), coloane.next()) {
            println!("{} => {}", nume, ip);
        }
    }

    Ok(())
}
fn main() {
    if let Err(error) = p1() {
        println!("Eroare: {}", error);
    }
    if let Err(error) = p2() {
        println!("Eroare: {}", error);
    }
    if let Err(error) = p3() {
        println!("Eroare: {}", error);
    }
    if let Err(error) = p4() {
        println!("Eroare: {}", error);
    }
}
