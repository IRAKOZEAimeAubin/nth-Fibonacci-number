use std::io;

fn main() {
    println!("nth Fibonacci number");
    println!("Find the nth Fibonacci number");

    loop {
        println!("Enter the position of the Fibonacci number:");

        let mut n = String::new();

        io::stdin().read_line(&mut n).expect("Failed to read line");

        let n: u64 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if n == 1 {
            println!(
                "The {}st Fibonacci number using iteration is: {}",
                n,
                fibonacci_iterative(n)
            );
            println!(
                "The {}st Fibonacci number using Binet's formula is: {}",
                n,
                fibonacci_binet(n)
            );
            continue;
        } else if n == 2 {
            println!(
                "The {}nd Fibonacci number using iteration is: {}",
                n,
                fibonacci_iterative(n)
            );
            println!(
                "The {}nd Fibonacci number using Binet's formula is: {}",
                n,
                fibonacci_binet(n)
            );
            continue;
        } else if n == 3 {
            println!(
                "The {}rd Fibonacci number using iteration is: {}",
                n,
                fibonacci_iterative(n)
            );
            println!(
                "The {}rd Fibonacci number using Binet's formula is: {}",
                n,
                fibonacci_binet(n)
            );
            continue;
        } else {
            println!(
                "The {}th Fibonacci number using iteration is: {}",
                n,
                fibonacci_iterative(n)
            );
            println!(
                "The {}th Fibonacci number using Binet's formula is: {}",
                n,
                fibonacci_binet(n)
            );
            continue;
        }
    }
}

fn fibonacci_iterative(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut x: u64 = 0;
    let mut y: u64 = 1;
    let mut z: u64;

    for _ in 2..=n {
        z = x + y;
        x = y;
        y = z;
    }

    y
}

fn fibonacci_binet(n: u64) -> u64 {
    const SQUAREROOT_OF_FIVE: f64 = 2.236067977499790; // sqrt(5) ≈ 2.236067977499790
    const PHI: f64 = 1.618033988749895; // φ = (1 + sqrt(5)) / 2 ≈ 1.618033988749895 (the golden ratio)
    const PSI: f64 = -0.618033988749895; // ψ = (1 - sqrt(5)) / 2 ≈ -0.618033988749895

    // Fn = (φ^n - ψ^n) / sqrt(5)
    return ((PHI.powf(n as f64) - PSI.powf(n as f64)) / SQUAREROOT_OF_FIVE) as u64;
}
