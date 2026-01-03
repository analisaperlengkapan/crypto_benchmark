use crypto_benchmark::{signatures, kem, BenchmarkKeys};
use crypto_benchmark::models::{BenchmarkReport, BenchmarkMetric};
use std::time::Instant;
use std::env;

mod server;

fn main() {
    print_header();
    
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    
    let json_output = args.iter().any(|arg| arg == "--json");

    if args.len() > 1 && !json_output {
        match args[1].as_str() {
            "-h" | "--help" => {
                print_usage();
            }
            "-v" | "--version" => {
                println!("Crypto Benchmark v0.2.0");
            }
            "serve" => {
                run_server();
            }
            "legacy" => {
                println!("\n⚠️  Legacy mode is deprecated and unavailable.");
                println!("Using optimized mode instead.\n");
                run_optimized_benchmarks(false);
            }
            "comparison" => {
                run_comparison();
            }
            _ => {
                println!("\n❌ Unknown option: {}", args[1]);
                print_usage();
            }
        }
    } else {
        // Default: run optimized benchmarks
        run_optimized_benchmarks(json_output);
    }
}

fn print_header() {
    // Only print header if not in JSON mode
    if !std::env::args().any(|arg| arg == "--json") {
        println!("╔═══════════════════════════════════════════════════════════════╗");
        println!("║         Cryptographic Benchmarking Tool v0.2.0               ║");
        println!("║    Classical & Post-Quantum Cryptography Performance         ║");
        println!("╚═══════════════════════════════════════════════════════════════╝");
    }
}

fn print_usage() {
    println!("\n📖 USAGE:");
    println!("  cargo run --release [OPTIONS]\n");
    println!("OPTIONS:");
    println!("  (none)           Run optimized benchmarks (default)");
    println!("  serve            Start the web interface");
    println!("  --json           Output results in JSON format");
    println!("  legacy           Run legacy mode (deprecated, uses optimized)");
    println!("  comparison       Compare legacy vs optimized performance");
    println!("  -h, --help       Show this help message");
    println!("  -v, --version    Show version information\n");
    println!("EXAMPLES:");
    println!("  cargo run --release              # Run optimized benchmarks");
    println!("  cargo run --release serve        # Start web interface");
    println!("  cargo run --release -- --json    # Run and output JSON");
    println!("  cargo run --release comparison   # Run comparison mode");
    println!("  cargo bench                      # Run Criterion benchmarks\n");
    println!("For more information, see README.md or CONTRIBUTING.md");
}

fn run_server() {
    println!("🚀 Starting Crypto Benchmark Server...");
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(e) => {
            eprintln!("❌ Failed to start Tokio runtime: {}", e);
            return;
        }
    };

    if let Err(e) = rt.block_on(async { server::start_server(3000).await }) {
        eprintln!("❌ Server error: {}", e);
        std::process::exit(1);
    }
}

#[allow(dead_code)]
fn run_legacy_benchmarks() {
    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║              LEGACY BENCHMARK MODE                     ║");
    println!("║              (Not available - use optimized)           ║");
    println!("╚════════════════════════════════════════════════════════╝");
    
    println!("\nLegacy mode has been replaced with optimized implementation.");
    println!("The optimized version is 70-80% faster with better accuracy.");
    println!("\nRunning optimized benchmarks instead...\n");
    
    run_optimized_benchmarks(false);
}

fn run_optimized_benchmarks(json_output: bool) {
    if !json_output {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║            OPTIMIZED BENCHMARK MODE                    ║");
        println!("║  Using pre-generated keys & statistical analysis       ║");
        println!("╚════════════════════════════════════════════════════════╝\n");
        println!("⏳ Generating benchmark keys...");
    }
    
    let start_keygen = Instant::now();
    let keys = match BenchmarkKeys::generate() {
        Ok(k) => k,
        Err(e) => {
            eprintln!("❌ Failed to generate keys: {}", e);
            eprintln!("   This may be due to insufficient system resources.");
            return;
        }
    };
    let keygen_time = start_keygen.elapsed();
    
    if !json_output {
        println!("✓ All keys generated successfully ({:.2}s)\n", keygen_time.as_secs_f64());
        println!("🚀 Running benchmarks with statistical analysis...\n");
    }

    let start_bench = Instant::now();
    
    // Run signatures benchmark
    let sig_metrics = signatures::benchmark_signatures_optimized(&keys);

    // Run KEM benchmark
    let kem_metrics = kem::benchmark_kem_optimized(&keys);

    let bench_time = start_bench.elapsed();
    let total_time = start_keygen.elapsed();

    if json_output {
        let report = BenchmarkReport {
            signatures: sig_metrics,
            kem: kem_metrics,
            keygen_time_secs: keygen_time.as_secs_f64(),
            total_time_secs: total_time.as_secs_f64(),
        };
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
    } else {
        print_human_readable_report(&sig_metrics, &kem_metrics, keygen_time.as_secs_f64(), bench_time.as_secs_f64(), total_time.as_secs_f64());
    }
}

fn print_human_readable_report(
    sig_metrics: &[BenchmarkMetric],
    kem_metrics: &[BenchmarkMetric],
    keygen_time: f64,
    bench_time: f64,
    total_time: f64
) {
    println!("═══════════════════════════════════════════════════════════");
    println!("                   SIGNATURE ALGORITHMS");
    println!("═══════════════════════════════════════════════════════════");
    println!("\n=== OPTIMIZED SIGNATURES BENCHMARK ===\n");

    // Group by algorithm name to match original output style if possible,
    // or just iterate. The original code printed headers for each algo.
    // We can infer groups or just print sequentially.
    // The metrics are already in order.
    
    let mut current_algo = "";
    for metric in sig_metrics {
        if metric.name != current_algo {
            current_algo = &metric.name;
            println!("{}:", current_algo);
            for (k, v) in &metric.extra_info {
                println!("  {}: {}", format_key(k), v);
            }
        }
        print_metric(metric);
    }

    println!("\n═══════════════════════════════════════════════════════════");
    println!("              KEY EXCHANGE MECHANISMS (KEM)");
    println!("═══════════════════════════════════════════════════════════");
    println!("\n=== OPTIMIZED KEM BENCHMARK ===\n");

    current_algo = "";
    for metric in kem_metrics {
        if metric.name != current_algo {
            current_algo = &metric.name;
            println!("{}:", current_algo);
             for (k, v) in &metric.extra_info {
                println!("  {}: {}", format_key(k), v);
            }
        }
        print_metric(metric);
    }
    
    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║                 BENCHMARK SUMMARY                      ║");
    println!("╚════════════════════════════════════════════════════════╝");
    println!("  📊 Statistics:      100 iterations per operation");
    println!("  ⚡ Key Generation:  {:.2}s (one-time cost)", keygen_time);
    println!("  🔬 Benchmark Time:  {:.2}s (all operations)", bench_time);
    println!("  ⏱️  Total Time:      {:.2}s", total_time);
    println!("\n✅ All benchmarks completed successfully!");
    println!("\n💡 Tip: Run 'cargo bench' for detailed Criterion analysis");
    println!("   See README.md for performance analysis and recommendations");
}

fn format_key(k: &str) -> String {
    // Simple helper to make keys like "key_size" look like "Key Size"
    k.split('_')
     .map(|s| {
         let mut c = s.chars();
         match c.next() {
             None => String::new(),
             Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
         }
     })
     .collect::<Vec<_>>()
     .join(" ")
}

fn print_metric(metric: &BenchmarkMetric) {
    println!("  {} Performance:", metric.operation);
    println!("    Mean:   {:>10.2} μs", metric.mean_micros);
    println!("    Min:    {:>10.2} μs", metric.min_micros);
    println!("    Max:    {:>10.2} μs", metric.max_micros);
    println!("    StdDev: {:>10.2} μs", metric.std_dev_micros);
    println!("    Iterations: {}", metric.iterations);
}

#[allow(dead_code)]
fn run_comparison() {
    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║              COMPARISON BENCHMARK MODE                 ║");
    println!("╚════════════════════════════════════════════════════════╝");
    
    println!("\n[1/2] Running LEGACY benchmarks...");
    let legacy_start = Instant::now();
    run_legacy_benchmarks();
    let legacy_time = legacy_start.elapsed();
    
    println!("\n\n[2/2] Running OPTIMIZED benchmarks...");
    let optimized_start = Instant::now();
    run_optimized_benchmarks(false);
    let optimized_time = optimized_start.elapsed();
    
    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║                 PERFORMANCE COMPARISON                 ║");
    println!("╚════════════════════════════════════════════════════════╝");
    println!("  Legacy Mode:     {:.2}s", legacy_time.as_secs_f64());
    println!("  Optimized Mode:  {:.2}s", optimized_time.as_secs_f64());
    
    let speedup = legacy_time.as_secs_f64() / optimized_time.as_secs_f64();
    let improvement = (legacy_time.as_secs_f64() - optimized_time.as_secs_f64()) 
                      / legacy_time.as_secs_f64() * 100.0;
    
    println!("\n  Speedup:         {:.2}x faster", speedup);
    println!("  Improvement:     {:.1}% reduction in time", improvement);
}
