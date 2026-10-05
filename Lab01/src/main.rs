fn nr_prim(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n.is_multiple_of(i) {
            return false;
        }
        i += 1;
    }
    true
}
fn cmmdc(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let rest = a % b;
        a = b;
        b = rest;
    }
    a
}
fn sticle_99() {
    let mut sticle = 99;
    while sticle >= 0 {
        if sticle > 2 {
            println!(
                "{} bottles of beer on the wall, {} bottles of beer.",
                sticle, sticle
            );
            println!(
                "Take one down, pass it around, {} bottles of beer on the wall,\n",
                sticle - 1
            );
        } else if sticle == 2 {
            println!("2 bottles of beer on the wall, 2 bottles of beer.");
            println!("Take one down and pass it around, 1 bottle of beer on the wall.\n");
        } else if sticle == 1 {
            println!("1 bottle of beer on the wall, 1 bottle of beer.");
            println!("Take one down and pass it around, 1 bottle of beer on the wall.\n");
        } else {
            println!("No more bottles of beer on the wall, no more bottles of beer.");
            println!("Go to the store and buy some more, 99 bottles of beer on the wall.");
        }
        sticle -= 1;
    }
}

fn prime_intre_ele(a: u32, b: u32) -> bool {
    cmmdc(a, b) == 1
}
fn main() {
    let mut num = 0;
    while num <= 100 {
        if nr_prim(num) {
            println!("Nr prim: {}", num);
        }
        num += 1;
    }
    let mut i: u32 = 1;
    while i <= 100 {
        let mut j: u32 = i + 1;
        while j <= 100 {
            if prime_intre_ele(i, j) {
                println!(" Numere prime_intre_ele: {} si {}", i, j);
            }
            j += 1;
        }
        i += 1;
    }
    sticle_99();
}
