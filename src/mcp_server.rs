use crate::fibonacci_ffi;
use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use futures::stream::{self, Stream};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::convert::Infallible;
use std::pin::Pin;
use std::{
    error::Error,
    io::{self, BufRead, BufReader, Write},
    process::Command,
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
    time::Instant,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct FibonacciMetrics {
    pub number: u32,
    pub result: u64,
    pub execution_time_ms: f64,
    pub implementation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComparisonMetrics {
    pub rust_metrics: FibonacciMetrics,
    pub ruby_metrics: FibonacciMetrics,
    pub rust_ruby_ffi_metrics: FibonacciMetrics,
    pub ruby_rust_ffi_metrics: FibonacciMetrics,
    pub speedup_vs_rust: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FibonacciRequest {
    pub number: u32,
    pub implementation: String, // "rust", "ruby", "rust_ruby_ffi", or "ruby_rust_ffi"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FibonacciResponse {
    pub result: u64,
    pub execution_time_ms: f64,
    pub implementation: String,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ContextRequest {
    Calculate(FibonacciRequest),
    Benchmark(FibonacciRequest),
}

pub struct MetricsServer {
    port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub parameters: Vec<ToolParameter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolParameter {
    pub name: String,
    pub description: String,
    pub r#type: String,
    pub required: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolRequest {
    pub tool: String,
    pub parameters: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPTool {
    pub name: String,
    pub description: String,
    pub parameters: Vec<MCPParameter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPParameter {
    pub name: String,
    pub description: String,
    pub r#type: String,
    pub required: bool,
}

#[get("/fibonacci/mcp")]
async fn get_info() -> impl Responder {
    let tool = Tool {
        name: "rusty_fib".to_string(),
        description: "Calculate Fibonacci numbers using Rust implementation".to_string(),
        parameters: vec![ToolParameter {
            name: "N".to_string(),
            description: "The Fibonacci number to calculate".to_string(),
            r#type: "integer".to_string(),
            required: true,
        }],
    };

    let info = serde_json::json!({
        "name": "Fibonacci MCP Server",
        "version": "1.0.0",
        "tools": [tool],
        "endpoints": {
            "GET /fibonacci/mcp": "This documentation",
            "POST /fibonacci/mcp": "Calculate Fibonacci numbers",
            "GET /fibonacci/mcp/sse": "Server-Sent Events endpoint for real-time calculations"
        }
    });

    HttpResponse::Ok().json(info)
}

#[get("/fibonacci/mcp/sse")]
async fn sse_endpoint(running: web::Data<Arc<AtomicBool>>) -> impl Responder {
    // Define the tool
    let tool = MCPTool {
        name: "rusty_fib".to_string(),
        description: "Calculate Fibonacci numbers using Rust implementation".to_string(),
        parameters: vec![MCPParameter {
            name: "N".to_string(),
            description: "The Fibonacci number to calculate".to_string(),
            r#type: "integer".to_string(),
            required: true,
        }],
    };

    // Create the tools advertisement message following MCP protocol
    let tools_message = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "advertise",
        "id": "1",
        "type": "tools",
        "tools": [tool]
    });

    let initial_message = format!("data: {}\n\n", tools_message.to_string());
    let running = running.clone();

    // Create a stream that sends the initial tools message and then keeps alive with periodic pings
    let stream = stream::once(
        async move { Ok::<_, Infallible>(web::Bytes::from(initial_message)) },
    )
    .chain(stream::unfold(0, move |id| {
        let running = running.clone();
        async move {
            if !running.load(Ordering::SeqCst) {
                return None;
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
            // Format ping as a proper JSON-RPC message
            let ping_message = serde_json::json!({
                "jsonrpc": "2.0",
                "method": "ping",
                "id": id.to_string(),
                "type": "ping"
            });
            Some((
                Ok::<_, Infallible>(web::Bytes::from(format!("data: {}\n\n", ping_message))),
                id + 1,
            ))
        }
    }));

    HttpResponse::Ok()
        .insert_header(("Content-Type", "text/event-stream"))
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(("Connection", "keep-alive"))
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .streaming(Box::pin(stream) as Pin<Box<dyn Stream<Item = Result<web::Bytes, Infallible>>>>)
}

#[post("/fibonacci/mcp")]
async fn handle_request(request: web::Json<ContextRequest>) -> impl Responder {
    let response = match request.into_inner() {
        ContextRequest::Calculate(req) => match handle_calculation(&req) {
            Ok(response) => response,
            Err(e) => FibonacciResponse {
                result: 0,
                execution_time_ms: 0.0,
                implementation: req.implementation,
                error: Some(e.to_string()),
            },
        },
        ContextRequest::Benchmark(req) => match handle_benchmark(&req) {
            Ok(response) => response,
            Err(e) => FibonacciResponse {
                result: 0,
                execution_time_ms: 0.0,
                implementation: req.implementation,
                error: Some(e.to_string()),
            },
        },
    };

    HttpResponse::Ok().json(response)
}

#[post("/fibonacci/mcp/tool")]
async fn handle_tool_request(request: web::Json<ToolRequest>) -> impl Responder {
    match request.tool.as_str() {
        "rusty_fib" => {
            if let Some(n) = request.parameters.get("N") {
                if let Ok(n) = n.parse::<u32>() {
                    let result = handle_calculation(&FibonacciRequest {
                        number: n,
                        implementation: "rust".to_string(),
                    });

                    match result {
                        Ok(response) => {
                            let response_json = serde_json::json!({
                                "jsonrpc": "2.0",
                                "id": "1",
                                "type": "response",
                                "success": true,
                                "data": response
                            });
                            HttpResponse::Ok().json(response_json)
                        }
                        Err(e) => {
                            let error_json = serde_json::json!({
                                "jsonrpc": "2.0",
                                "id": "1",
                                "error": {
                                    "code": -32000,
                                    "message": e.to_string()
                                }
                            });
                            HttpResponse::InternalServerError().json(error_json)
                        }
                    }
                } else {
                    let error_json = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": "1",
                        "error": {
                            "code": -32602,
                            "message": "Invalid N parameter: must be a positive integer"
                        }
                    });
                    HttpResponse::BadRequest().json(error_json)
                }
            } else {
                let error_json = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": "1",
                    "error": {
                        "code": -32602,
                        "message": "Missing required parameter: N"
                    }
                });
                HttpResponse::BadRequest().json(error_json)
            }
        }
        _ => {
            let error_json = serde_json::json!({
                "jsonrpc": "2.0",
                "id": "1",
                "error": {
                    "code": -32601,
                    "message": "Tool not found"
                }
            });
            HttpResponse::NotFound().json(error_json)
        }
    }
}

impl MetricsServer {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        println!(
            "Starting MCP Server at: http://localhost:{}/fibonacci/mcp",
            self.port
        );
        println!(
            "SSE endpoint at: http://localhost:{}/fibonacci/mcp/sse",
            self.port
        );

        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();

        let server = HttpServer::new(move || {
            let cors = Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .max_age(3600);

            App::new()
                .wrap(cors)
                .app_data(web::Data::new(running_clone.clone()))
                .service(get_info)
                .service(handle_request)
                .service(sse_endpoint)
                .service(handle_tool_request)
        })
        .bind(("0.0.0.0", self.port))?
        .workers(2) // Limit worker threads
        .shutdown_timeout(5) // 5 seconds shutdown timeout
        .run();

        // Set up Ctrl+C handler
        let running_clone = running.clone();
        tokio::spawn(async move {
            tokio::signal::ctrl_c().await.unwrap();
            println!("\nReceived Ctrl+C, initiating graceful shutdown...");
            running_clone.store(false, Ordering::SeqCst);
        });

        server.await?;
        println!("Server shutdown complete.");
        Ok(())
    }

    pub fn report_comparison(&self, comparison: &ComparisonMetrics) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string(comparison)?;
        println!("{}", json);
        Ok(())
    }
}

fn handle_calculation(request: &FibonacciRequest) -> Result<FibonacciResponse, Box<dyn Error>> {
    match request.implementation.as_str() {
        "rust" => {
            let start = Instant::now();
            let result = fibonacci_ffi(request.number);
            let duration = start.elapsed().as_secs_f64() * 1000.0;

            Ok(FibonacciResponse {
                result,
                execution_time_ms: duration,
                implementation: "rust".to_string(),
                error: None,
            })
        }
        "ruby" => match run_ruby_calculation(request) {
            Ok(response) => Ok(response),
            Err(e) => Err(e.into()),
        },
        "rust_ruby_ffi" => match run_ruby_ffi_calculation(request) {
            Ok(response) => Ok(response),
            Err(e) => Err(e.into()),
        },
        "ruby_rust_ffi" => {
            let start = Instant::now();
            let result = fibonacci_ffi(request.number);
            let duration = start.elapsed().as_secs_f64() * 1000.0;

            Ok(FibonacciResponse {
                result,
                execution_time_ms: duration,
                implementation: "ruby_rust_ffi".to_string(),
                error: None,
            })
        }
        _ => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid implementation specified",
        ))),
    }
}

fn handle_benchmark(request: &FibonacciRequest) -> Result<FibonacciResponse, Box<dyn Error>> {
    let mut times = Vec::new();
    let mut result = None;

    for _ in 0..100 {
        match handle_calculation(&FibonacciRequest {
            number: request.number,
            implementation: request.implementation.clone(),
        })? {
            response => {
                if result.is_none() {
                    result = Some(response.result);
                }
                times.push(response.execution_time_ms);
            }
        }
    }

    if let Some(result) = result {
        times.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mean = times.iter().sum::<f64>() / times.len() as f64;

        Ok(FibonacciResponse {
            result,
            execution_time_ms: mean,
            implementation: request.implementation.clone(),
            error: None,
        })
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "No successful calculations",
        )))
    }
}

fn run_ruby_calculation(request: &FibonacciRequest) -> Result<FibonacciResponse, Box<dyn Error>> {
    let start = Instant::now();
    let output = Command::new("ruby")
        .args(["fibonacci.rb", &request.number.to_string()])
        .output()
        .map_err(|e| format!("Failed to execute Ruby: {}", e))?;

    if !output.status.success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            String::from_utf8_lossy(&output.stderr).to_string(),
        )));
    }

    let result = String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<u64>()
        .map_err(|e| format!("Failed to parse Ruby output: {}", e))?;

    Ok(FibonacciResponse {
        result,
        execution_time_ms: start.elapsed().as_secs_f64() * 1000.0,
        implementation: "ruby".to_string(),
        error: None,
    })
}

fn run_ruby_ffi_calculation(
    request: &FibonacciRequest,
) -> Result<FibonacciResponse, Box<dyn Error>> {
    let start = Instant::now();
    let output = Command::new("ruby")
        .args(["fibonacci_ffi.rb", &request.number.to_string()])
        .output()
        .map_err(|e| format!("Failed to execute Ruby FFI: {}", e))?;

    if !output.status.success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            String::from_utf8_lossy(&output.stderr).to_string(),
        )));
    }

    let result = String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<u64>()
        .map_err(|e| format!("Failed to parse Ruby FFI output: {}", e))?;

    Ok(FibonacciResponse {
        result,
        execution_time_ms: start.elapsed().as_secs_f64() * 1000.0,
        implementation: "ruby_rust_ffi".to_string(),
        error: None,
    })
}

pub fn start_metrics_server() -> Result<MetricsServer, Box<dyn Error>> {
    Ok(MetricsServer::new(8080))
}
