# Prime Hunter

A memory-efficient Rust CLI application for finding prime numbers with timeout control.

## Project Background

Prime Hunter is a headless command-line application designed to count prime numbers within a user-defined range while maintaining minimal memory footprint. Unlike traditional prime finders that store all discovered primes, this application only tracks the count and the highest prime found, making it suitable for searching extremely large number ranges without exhausting system memory.

### Key Features
- **Memory Efficient**: Doesn't store all primes, only tracks count and highest prime
- **Timeout Control**: User-defined timeout prevents indefinite execution
- **Real-time Timer**: Visual feedback showing elapsed time during execution
- **Async Design**: Uses Tokio for non-blocking operations
- **Simple CLI**: Clean command-line interface with validation

## App Structure/Architecture
```
prime_hunter/
├── Cargo.toml          # Project dependencies and metadata
├── src/
│   └── main.rs         # Main application logic
└── README.md           # This file
```

### Core Components

1. **Prime Detection** (`is_prime` function)
    - Trial division algorithm optimized for odd numbers
    - Tests divisibility only up to √n
    - O(√n) time complexity per number

2. **Async Prime Finder** (`find_primes`)
    - Iterates through range checking each number
    - Monitors elapsed time against timeout
    - Updates atomic counters for thread-safe progress tracking
    - Returns: prime count, highest prime, elapsed time, timeout status

3. **Timer Display** (`display_timer`)
    - Async task running parallel to prime finding
    - Updates console every 100ms with elapsed time
    - Uses atomic boolean for clean shutdown

4. **User Interface**
    - Input validation with retry loops
    - Real-time progress indicator
    - Comprehensive results report
    - Timeout coverage percentage

### Memory Optimization Strategy

The application maintains O(1) memory usage regardless of search range:
- `prime_count`: Single u64 counter
- `highest_prime`: Single u64 value
- `current_number`: Atomic u64 for progress tracking
- No vector or collection to store prime numbers

## Deployment and Usage Instructions

### Prerequisites

- Rust 1.70 or later
- Cargo (included with Rust)

### Installation

1. **Install Rust** (if not already installed):
```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **Clone or create the project**:
```bash
   mkdir prime_hunter
   cd prime_hunter
   # Copy Cargo.toml and src/main.rs to respective locations
```

### Build
```bash
# Development build
cargo build

# Optimized release build (recommended for large searches)
cargo build --release
```

### Usage

#### Development Mode
```bash
cargo run
```

#### Release Mode (faster execution)
```bash
cargo run --release
```

#### Direct Binary Execution
```bash
# After building
./target/release/prime_hunter
```

### Example Session
```
╔═══════════════════════════════════════╗
║     PRIME NUMBER HUNTER v0.1.0        ║
║  "Efficiency: not just for robots"   ║
╚═══════════════════════════════════════╝

Enter the maximum number to search: 1000000
Enter timeout in minutes (e.g., 0.5 for 30 seconds): 2.0

🔍 Searching for primes up to 1000000...
⏰ Timeout set to 2.00 minutes (120.0 seconds)

⏱️  Running: 0.8s

╔═══════════════════════════════════════╗
║            MISSION REPORT             ║
╚═══════════════════════════════════════╝
📊 Search Range:        1 to 1000000
🎯 Primes Found:        78498
👑 Highest Prime:       999983
⏱️  Execution Time:      0.824 seconds
⏰ Timeout Limit:       2.00 minutes

✅ Search completed successfully!
```

### Performance Notes

| Range | Approximate Time | Prime Count |
|-------|-----------------|-------------|
| 10,000 | < 0.01s | 1,229 |
| 100,000 | ~0.05s | 9,592 |
| 1,000,000 | ~0.8s | 78,498 |
| 10,000,000 | ~12s | 664,579 |
| 100,000,000 | ~3-5 min | 5,761,455 |

*Times measured on Apple M4 (performance varies by hardware)*

### Exit Options

- **Ctrl+C**: Immediately terminate the application
- **Timeout**: Automatic termination after user-defined duration
- **Completion**: Natural exit after searching entire range

### Troubleshooting

**Issue**: Slow performance for large numbers
**Solution**: Use release build (`cargo run --release`) which includes optimizations

**Issue**: Timeout too short
**Solution**: Use decimal minutes (e.g., 0.5 for 30 seconds) for fine control

**Issue**: Input validation errors
**Solution**: Ensure entering positive numbers only; decimals allowed for timeout

## Technical Details

- **Language**: Rust 2021 Edition
- **Async Runtime**: Tokio 1.35
- **Algorithm**: Trial Division with √n optimization
- **Concurrency**: Async/await for timer and computation separation
- **Thread Safety**: Atomic operations for shared state

---

*"This little maneuver's gonna save us 51 years... of memory allocation." - TARS*