use clap::Parser;
use colored::*;
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use prettytable::{row, Table};
use rusty::fibonacci_ffi;
use std::fs::File;
use std::io::Write;
use std::{process::Command, time::Instant};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Fibonacci number to calculate
    #[arg(short = 'N', long, default_value_t = 88)]
    number: u32,

    /// Use only Rust implementation
    #[arg(long)]
    rust: bool,

    /// Use only Ruby implementation
    #[arg(long)]
    ruby: bool,

    /// Enable MCP server mode
    #[arg(long)]
    metrics: bool,
}

fn benchmark_rust(n: u32) -> (u64, Vec<f64>) {
    let mut times = Vec::new();

    // Run multiple iterations for more accurate timing
    for i in 0..100 {
        let start = Instant::now();
        let result = fibonacci_ffi(n); // FFI function is already marked as safe
        let duration = start.elapsed().as_secs_f64() * 1000.0;
        times.push(duration);
        println!(
            "Rust FFI Iteration {}: Result = {}, Time = {:.6}ms",
            i + 1,
            result,
            duration
        );
    }

    times.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (fibonacci_ffi(n), times) // FFI function is already marked as safe
}

fn run_ruby_command(n: u32) -> Result<(u64, Vec<f64>), String> {
    let mut times = Vec::new();
    let mut result = None;

    // Run multiple iterations for more accurate timing
    for i in 0..100 {
        let start = Instant::now();
        let output = Command::new("ruby")
            .args(["fibonacci.rb", &n.to_string()])
            .output()
            .map_err(|e| format!("Failed to execute Ruby: {}", e))?;

        let duration = start.elapsed().as_secs_f64() * 1000.0;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        let current_result = String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse::<u64>()
            .map_err(|e| format!("Failed to parse Ruby output: {}", e))?;

        // Store the result from the first run
        if i == 0 {
            result = Some(current_result);
        }

        times.push(duration);
        println!(
            "Ruby Iteration {}: Result = {}, Time = {:.6}ms",
            i + 1,
            current_result,
            duration
        );
    }

    // Sort times for percentile calculations
    times.sort_by(|a, b| a.partial_cmp(b).unwrap());

    Ok((result.unwrap(), times))
}

fn run_ruby_ffi_command(n: u32) -> Result<(u64, Vec<f64>), String> {
    let mut times = Vec::new();
    let mut result = None;

    // Run multiple iterations for more accurate timing
    for i in 0..100 {
        let start = Instant::now();
        let output = Command::new("ruby")
            .args(["fibonacci_ffi.rb", &n.to_string()])
            .output()
            .map_err(|e| format!("Failed to execute Ruby FFI: {}", e))?;

        let duration = start.elapsed().as_secs_f64() * 1000.0;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        let current_result = String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse::<u64>()
            .map_err(|e| format!("Failed to parse Ruby FFI output: {}", e))?;

        // Store the result from the first run
        if i == 0 {
            result = Some(current_result);
        }

        times.push(duration);
        println!(
            "Ruby FFI Iteration {}: Result = {}, Time = {:.6}ms",
            i + 1,
            current_result,
            duration
        );
    }

    // Sort times for percentile calculations
    times.sort_by(|a, b| a.partial_cmp(b).unwrap());

    Ok((result.unwrap(), times))
}

async fn generate_comparison_text(rust_time: f64, ruby_time: f64) -> String {
    let ollama = Ollama::default();
    let prompt = format!(
        "Write a fun, playful, one-paragraph comparison of these benchmark results: Rust took {:.6}ms and Ruby took {:.6}ms. Be creative and entertaining!",
        rust_time,
        ruby_time
    );

    let request = GenerationRequest::new(
        "mistral".to_string(), // or whichever model you have locally
        prompt,
    );

    match ollama.generate(request).await {
        Ok(response) => response.response,
        Err(_) => format!(
            "Rust was {:.2}x faster than Ruby! 🚀",
            ruby_time / rust_time
        ),
    }
}

fn print_consolidated_table(
    _n: u32,
    rust_result: Option<&(u64, Vec<f64>)>,
    ruby_result: Option<&(u64, Vec<f64>)>,
    ruby_to_rust_result: Option<&(u64, Vec<f64>)>,
    rust_to_ruby_result: Option<&(u64, Vec<f64>)>,
    file: &mut File,
) {
    let header = format!(
        "\n{}",
        "🎯 Consolidated Performance Results:".green().bold()
    );
    println!("{}", header);
    writeln!(file, "{}", header).expect("Failed to write to file");

    let separator = "--------------------------------";
    println!("{}", separator);
    writeln!(file, "{}", separator).expect("Failed to write to file");

    let mut table = Table::new();
    table.add_row(row![
        "Implementation",
        "Result",
        "Mean (ms)",
        "Median (ms)",
        "P95 (ms)",
        "Min (ms)",
        "Max (ms)",
        "Speedup vs Rust"
    ]);

    if let (
        Some((rust_val, rust_times)),
        Some((ruby_val, ruby_times)),
        Some((r2r_val, r2r_times)),
        Some((rtr_val, rtr_times)),
    ) = (
        rust_result,
        ruby_result,
        ruby_to_rust_result,
        rust_to_ruby_result,
    ) {
        // Calculate statistics for each implementation
        let rust_stats = calculate_stats(rust_times);
        let ruby_stats = calculate_stats(ruby_times);
        let r2r_stats = calculate_stats(r2r_times);
        let rtr_stats = calculate_stats(rtr_times);

        // Add rows for each implementation
        table.add_row(row![
            "🦀 Pure Rust",
            rust_val,
            format!("{:.6}", rust_stats.mean),
            format!("{:.6}", rust_stats.median),
            format!("{:.6}", rust_stats.p95),
            format!("{:.6}", rust_stats.min),
            format!("{:.6}", rust_stats.max),
            "1.00x (baseline)"
        ]);

        table.add_row(row![
            "💎 Pure Ruby",
            ruby_val,
            format!("{:.6}", ruby_stats.mean),
            format!("{:.6}", ruby_stats.median),
            format!("{:.6}", ruby_stats.p95),
            format!("{:.6}", ruby_stats.min),
            format!("{:.6}", ruby_stats.max),
            format!("{:.2}x slower", ruby_stats.median / rust_stats.median)
        ]);

        table.add_row(row![
            "🔄 Ruby->Rust FFI",
            r2r_val,
            format!("{:.6}", r2r_stats.mean),
            format!("{:.6}", r2r_stats.median),
            format!("{:.6}", r2r_stats.p95),
            format!("{:.6}", r2r_stats.min),
            format!("{:.6}", r2r_stats.max),
            format!("{:.2}x slower", r2r_stats.median / rust_stats.median)
        ]);

        table.add_row(row![
            "🔄 Rust->Ruby FFI",
            rtr_val,
            format!("{:.6}", rtr_stats.mean),
            format!("{:.6}", rtr_stats.median),
            format!("{:.6}", rtr_stats.p95),
            format!("{:.6}", rtr_stats.min),
            format!("{:.6}", rtr_stats.max),
            format!("{:.2}x slower", rtr_stats.median / rust_stats.median)
        ]);
    } else {
        table.add_row(row![
            "Error",
            "Could not calculate metrics",
            "-",
            "-",
            "-",
            "-",
            "-",
            "-"
        ]);
    }

    // Capture table output as string
    let mut table_string = Vec::new();
    table
        .print(&mut table_string)
        .expect("Failed to capture table");
    let table_string = String::from_utf8(table_string).expect("Invalid UTF-8");

    // Print to both console and file
    print!("{}", table_string);
    writeln!(
        file,
        "{}",
        String::from_utf8(table_string.into_bytes()).expect("Invalid UTF-8")
    )
    .expect("Failed to write to file");
}

// Helper struct for statistics
struct Stats {
    mean: f64,
    median: f64,
    p95: f64,
    min: f64,
    max: f64,
}

fn calculate_stats(times: &[f64]) -> Stats {
    let len = times.len();
    Stats {
        mean: times.iter().sum::<f64>() / len as f64,
        median: times[len / 2],
        p95: times[(len as f64 * 0.95) as usize],
        min: times[0],
        max: times[len - 1],
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Create output file
    let mut output_file = std::fs::File::create("output.txt")?;

    // Write UTF-8 BOM at the start if the file is empty
    let metadata = output_file.metadata().expect("Failed to get file metadata");
    if metadata.len() == 0 {
        output_file
            .write_all(&[0xEF, 0xBB, 0xBF])
            .expect("Failed to write BOM");
    }

    // Write timestamp to the file
    writeln!(output_file, "\n=== Run at {} ===", chrono::Local::now())
        .expect("Failed to write timestamp");

    // Helper macro to write to both console and file
    macro_rules! output {
        ($($arg:tt)*) => {{
            let output = format!($($arg)*);
            println!("{}", output);
            writeln!(output_file, "{}", output).expect("Failed to write to file");
        }};
    }

    // If neither implementation is specified, run both
    let run_rust = args.rust || !args.ruby;
    let run_ruby = args.ruby || !args.rust;

    if !args.rust && !args.ruby {
        output!(
            "Running both Rust and Ruby implementations for N = {}\n",
            args.number
        );
    } else {
        output!(
            "Running {} implementation{} for N = {}\n",
            match (args.rust, args.ruby) {
                (true, false) => "Rust",
                (false, true) => "Ruby",
                _ => unreachable!(),
            },
            if args.rust && args.ruby { "s" } else { "" },
            args.number
        );
    }

    let mut rust_result = None;
    let mut ruby_result = None;
    let mut ruby_to_rust_result = None;
    let mut rust_to_ruby_result = None;

    // Run Rust implementation
    if run_rust {
        output!("\n{}", "🦀 Rust Implementation:".cyan().bold());
        let (result, times) = benchmark_rust(args.number);
        output!("The {}th Fibonacci number is: {}", args.number, result);
        rust_result = Some((result, times));
    }

    // Run Ruby implementation
    if run_ruby {
        output!("\n{}", "💎 Ruby Implementation:".cyan().bold());
        match run_ruby_command(args.number) {
            Ok((result, times)) => {
                ruby_result = Some((result, times));
            }
            Err(e) => output!("{}: {}", "Error running Ruby".red().bold(), e),
        }
    }

    // Run Ruby→Rust FFI implementation
    if run_ruby {
        output!("\n{}", "🔗 Ruby→Rust FFI Implementation:".cyan().bold());
        match run_ruby_ffi_command(args.number) {
            Ok((result, times)) => {
                ruby_to_rust_result = Some((result, times));
            }
            Err(e) => output!("{}: {}", "Error running Ruby FFI".red().bold(), e),
        }
    }

    // Run Rust→Ruby FFI implementation
    if run_rust {
        output!("\n{}", "🔗 Rust→Ruby FFI Implementation:".cyan().bold());
        match run_ruby_command(args.number) {
            Ok((result, times)) => {
                rust_to_ruby_result = Some((result, times));
            }
            Err(e) => output!("{}: {}", "Error running Rust→Ruby FFI".red().bold(), e),
        }
    }

    // Get AI comparison if both implementations ran successfully
    if let (
        Some((_, rust_times)),
        Some((_, ruby_times)),
        Some((_, _r2r_times)),
        Some((_, _r2r_rev_times)),
    ) = (
        &rust_result,
        &ruby_result,
        &ruby_to_rust_result,
        &rust_to_ruby_result,
    ) {
        let rust_median = rust_times[rust_times.len() / 2];
        let ruby_median = ruby_times[ruby_times.len() / 2];

        let comparison = generate_comparison_text(rust_median, ruby_median).await;
        output!("\n{}", comparison.cyan().italic());
    }

    Ok(())
}
