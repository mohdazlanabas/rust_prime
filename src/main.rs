// src/main.rs
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as u64;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

async fn display_timer(running: Arc<AtomicBool>, start_time: Instant) {
    while running.load(Ordering::Relaxed) {
        sleep(Duration::from_millis(100)).await;
        let elapsed = start_time.elapsed();
        print!("\r⏱️  Running: {:.1}s", elapsed.as_secs_f64());
        io::stdout().flush().unwrap();
    }
    println!(); // New line after timer stops
}

async fn find_primes(
    max_number: u64,
    timeout_minutes: f64,
    current_number: Arc<AtomicU64>,
    running: Arc<AtomicBool>,
) -> (u64, u64, Duration, bool) {
    let start = Instant::now();
    let timeout_duration = Duration::from_secs_f64(timeout_minutes * 60.0);

    let mut prime_count = 0u64;
    let mut highest_prime = 0u64;
    let mut timed_out = false;

    for n in 1..=max_number {
        if start.elapsed() > timeout_duration {
            timed_out = true;
            break;
        }

        current_number.store(n, Ordering::Relaxed);

        if is_prime(n) {
            prime_count += 1;
            highest_prime = n;
        }
    }

    running.store(false, Ordering::Relaxed);
    let elapsed = start.elapsed();

    (prime_count, highest_prime, elapsed, timed_out)
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

#[tokio::main]
async fn main() {
    println!("╔═══════════════════════════════════════╗");
    println!("║     PRIME NUMBER HUNTER v0.1.0        ║");
    println!("║  \"Efficiency: not just for robots\"   ║");
    println!("╚═══════════════════════════════════════╝\n");

    // Get target number
    let max_number: u64 = loop {
        let input = get_user_input("Enter the maximum number to search: ");
        match input.parse() {
            Ok(num) if num > 0 => break num,
            Ok(_) => println!("❌ Please enter a positive number."),
            Err(_) => println!("❌ Invalid input. Please enter a valid number."),
        }
    };

    // Get timeout in minutes
    let timeout_minutes: f64 = loop {
        let input = get_user_input("Enter timeout in minutes (e.g., 0.5 for 30 seconds): ");
        match input.parse() {
            Ok(num) if num > 0.0 => break num,
            Ok(_) => println!("❌ Please enter a positive number."),
            Err(_) => println!("❌ Invalid input. Please enter a valid number."),
        }
    };

    println!("\n🔍 Searching for primes up to {}...", max_number);
    println!("⏰ Timeout set to {:.2} minutes ({:.1} seconds)\n",
             timeout_minutes, timeout_minutes * 60.0);

    let running = Arc::new(AtomicBool::new(true));
    let current_number = Arc::new(AtomicU64::new(0));
    let start_time = Instant::now();

    // Spawn timer task
    let running_clone = Arc::clone(&running);
    let timer_handle = tokio::spawn(display_timer(running_clone, start_time));

    // Run prime finder
    let running_clone = Arc::clone(&running);
    let current_number_clone = Arc::clone(&current_number);
    let (prime_count, highest_prime, elapsed, timed_out) =
        find_primes(max_number, timeout_minutes, current_number_clone, running_clone).await;

    // Wait for timer to finish
    timer_handle.await.unwrap();

    // Display results
    println!("\n╔═══════════════════════════════════════╗");
    println!("║            MISSION REPORT             ║");
    println!("╚═══════════════════════════════════════╝");
    println!("📊 Search Range:        1 to {}", max_number);
    println!("🎯 Primes Found:        {}", prime_count);
    println!("👑 Highest Prime:       {}", highest_prime);
    println!("⏱️  Execution Time:      {:.3} seconds", elapsed.as_secs_f64());
    println!("⏰ Timeout Limit:       {:.2} minutes", timeout_minutes);

    if timed_out {
        println!("\n⚠️  TIMEOUT: Search terminated before completion.");
        println!("   Last number checked: {}", current_number.load(Ordering::Relaxed));
        println!("   Coverage: {:.2}%",
                 (current_number.load(Ordering::Relaxed) as f64 / max_number as f64) * 100.0);
    } else {
        println!("\n✅ Search completed successfully!");
    }

    println!("\n\"Everybody good? Plenty of slaves for my robot colony?\" - TARS (25% humor)");
}