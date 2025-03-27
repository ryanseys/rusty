use clap::Parser;
use colored::*;
use std::process::{exit, Command};
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name to use in greeting
    #[arg(short, long, default_value = "friend")]
    name: String,

    /// Fibonacci number to calculate
    #[arg(short = 'N', long, default_value_t = 10)]
    number: u32,

    /// Use Rust implementation
    #[arg(long)]
    rust: bool,

    /// Use Ruby implementation
    #[arg(long)]
    ruby: bool,

    /// Run in batch mode
    #[arg(long)]
    batch: bool,

    /// Batch size for testing
    #[arg(long, default_value_t = 10)]
    batch_size: u32,
}

fn build_rust_lib() -> Result<(), String> {
    println!("{}", "Building Rust library...".yellow());
    let output = Command::new("cargo")
        .args(["build", "--release"])
        .output()
        .map_err(|e| format!("Failed to build Rust library: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    Ok(())
}

fn install_ruby_deps() -> Result<(), String> {
    println!("{}", "Installing Ruby dependencies...".yellow());
    let output = Command::new("bundle")
        .args(["install", "--quiet"])
        .output()
        .map_err(|e| format!("Failed to install Ruby dependencies: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    Ok(())
}

fn run_ruby_command(args: &Args) -> Result<(), String> {
    let mut cmd = Command::new("ruby");
    cmd.arg("main.rb");

    if args.batch {
        cmd.arg("--batch");
        cmd.arg(args.batch_size.to_string());
    }

    cmd.arg("--name").arg(&args.name);
    cmd.arg("--number").arg(args.number.to_string());

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to run Ruby script: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    print!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

fn main() {
    let args = Args::parse();

    // If neither implementation is specified, default to Ruby
    if !args.rust && !args.ruby {
        println!(
            "{}",
            "No implementation specified, defaulting to Ruby.".yellow()
        );
        let args = Args { ruby: true, ..args };
        if let Err(e) = run(args) {
            eprintln!("{}: {}", "Error".red().bold(), e);
            exit(1);
        }
        return;
    }

    if let Err(e) = run(args) {
        eprintln!("{}: {}", "Error".red().bold(), e);
        exit(1);
    }
}

fn run(args: Args) -> Result<(), String> {
    println!("Configuration:");
    println!("  Build: {}", "release".green());
    println!("  Number: {}", args.number.to_string().green());
    println!(
        "  Mode: {}",
        if args.batch {
            "batch".green()
        } else {
            "single".green()
        }
    );

    let start = Instant::now();

    if args.rust {
        build_rust_lib()?;
    }

    if args.ruby {
        install_ruby_deps()?;
        run_ruby_command(&args)?;
    }

    let total_time = start.elapsed();
    println!(
        "\nTotal script time: {:.3} seconds",
        total_time.as_secs_f64()
    );

    Ok(())
}
