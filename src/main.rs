use clap::Parser;
use colored::*;
use criterion::black_box;
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use prettytable::{row, Table};
use std::process::Command;
use std::time::Instant;

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
}

// Direct Rust implementation
fn fibonacci(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    let mut a = 0u64;
    let mut b = 1u64;
    for _ in 1..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

fn benchmark_rust(n: u32) -> (u64, Vec<f64>) {
    let mut times = Vec::new();

    // Run multiple iterations for more accurate timing
    for i in 0..100 {
        let start = Instant::now();
        let result = fibonacci(black_box(n));
        let duration = start.elapsed().as_secs_f64() * 1000.0;
        times.push(duration);
        println!(
            "Iteration {}: Result = {}, Time = {:.6}ms",
            i + 1,
            result,
            duration
        );
    }

    // Sort times for percentile calculations
    times.sort_by(|a, b| a.partial_cmp(b).unwrap());

    (fibonacci(n), times)
}

fn run_ruby_command(n: u32) -> Result<(u64, Vec<f64>), String> {
    let mut times = Vec::new();
    let mut result = None;

    // Run multiple iterations for more accurate timing
    for i in 0..100 {
        let start = Instant::now();
        let output = Command::new("ruby")
            .args(&["fibonacci.rb", &n.to_string()])
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

fn print_statistics(times: &[f64], implementation: &str) {
    let len = times.len();
    let mean: f64 = times.iter().sum::<f64>() / len as f64;
    let median = times[len / 2];
    let p95 = times[(len as f64 * 0.95) as usize];
    let min = times[0];
    let max = times[len - 1];

    let mut table = Table::new();
    table.add_row(row!["Metric", "Time (ms)"]);
    table.add_row(row![
        format!("{} Mean", implementation),
        format!("{:.6}", mean)
    ]);
    table.add_row(row![
        format!("{} Median", implementation),
        format!("{:.6}", median)
    ]);
    table.add_row(row![
        format!("{} P95", implementation),
        format!("{:.6}", p95)
    ]);
    table.add_row(row![
        format!("{} Min", implementation),
        format!("{:.6}", min)
    ]);
    table.add_row(row![
        format!("{} Max", implementation),
        format!("{:.6}", max)
    ]);

    table.printstd();
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

fn main() {
    let args = Args::parse();

    // If neither implementation is specified, run both
    let run_rust = args.rust || (!args.rust && !args.ruby);
    let run_ruby = args.ruby || (!args.rust && !args.ruby);

    if !args.rust && !args.ruby {
        println!(
            "{}",
            "No implementation specified, running both for comparison.".yellow()
        );
    }

    println!("{}", "\nBenchmarking Implementations:".bold());
    println!("================================");

    let mut rust_result = None;
    let mut ruby_result = None;
    let mut ruby_time = None;

    // Run Rust implementation
    if run_rust {
        println!("\n{}", "🦀 Rust Implementation:".cyan().bold());
        let (result, times) = benchmark_rust(args.number);
        println!("The {}th Fibonacci number is: {}", args.number, result);
        print_statistics(&times, "🦀 Rust");
        rust_result = Some((result, times[times.len() / 2])); // Store median time
    }

    // Run Ruby implementation
    if run_ruby {
        println!("\n{}", "💎 Ruby Implementation:".cyan().bold());
        match run_ruby_command(args.number) {
            Ok((result, times)) => {
                print_statistics(&times, "💎 Ruby");
                ruby_result = Some(result);
                ruby_time = Some(times[times.len() / 2]); // Use median time
            }
            Err(e) => eprintln!("{}: {}", "Error running Ruby".red().bold(), e),
        }
    }

    // Compare implementations
    if let (Some((rust_result, rust_median)), Some(ruby_time)) = (rust_result, ruby_time) {
        println!("\n{}", "⚡ Performance Comparison:".green().bold());
        println!("--------------------");

        let mut table = Table::new();
        table.add_row(row!["Metric", "Value"]);
        table.add_row(row!["🔢 Fibonacci Number (N)", args.number]);
        table.add_row(row!["🦀 Rust Result", rust_result]);
        table.add_row(row![
            "💎 Ruby Result",
            ruby_result.expect("Ruby result should be available")
        ]);
        table.add_row(row![
            "🦀 Rust Median Time (ms)",
            format!("{:.6}", rust_median)
        ]);
        table.add_row(row![
            "💎 Ruby Median Time (ms)",
            format!("{:.6}", ruby_time)
        ]);
        table.add_row(row![
            "🚀 Speed Ratio",
            format!("{:.2}x (Rust faster)", ruby_time / rust_median)
        ]);

        table.printstd();

        // Add the fun comparison
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let comparison = runtime.block_on(generate_comparison_text(rust_median, ruby_time));
        println!("\n{}", comparison.cyan().italic());
    }
}
