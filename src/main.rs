// File: src/main.rs
// Simple Rust Web Application - Complete and Working

use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Serialize)]
struct ApiResponse {
    message: String,
    result: i32,
}

#[derive(Deserialize)]
struct NumberRequest {
    number: i32,
}

// Simple function to calculate square of a number
fn calculate_square(number: i32) -> i32 {
    number * number
}

#[tokio::main]
async fn main() {
    // Serve static HTML page at root
    let html_page = warp::path::end()
        .map(|| {
            warp::reply::html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Rust API - Modern Calculator</title>
    
    <!-- Google Fonts -->
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&family=JetBrains+Mono:wght@400;500;600&display=swap" rel="stylesheet">
    
    <!-- Tailwind CSS -->
    <script src="https://cdn.tailwindcss.com"></script>
    
    <!-- Tailwind Config -->
    <script>
        tailwind.config = {
            theme: {
                extend: {
                    fontFamily: {
                        'sans': ['Inter', 'system-ui', 'sans-serif'],
                        'mono': ['JetBrains Mono', 'Monaco', 'monospace'],
                    },
                    animation: {
                        'fade-in': 'fadeIn 0.5s ease-in-out',
                        'slide-up': 'slideUp 0.3s ease-out',
                        'pulse-soft': 'pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite',
                    }
                }
            }
        }
    </script>
    
    <style>
        @keyframes fadeIn {
            from { opacity: 0; }
            to { opacity: 1; }
        }
        @keyframes slideUp {
            from { transform: translateY(10px); opacity: 0; }
            to { transform: translateY(0); opacity: 1; }
        }
    </style>
</head>
<body class="bg-gradient-to-br from-blue-50 via-white to-purple-50 min-h-screen font-sans">
    <!-- Navigation Bar -->
    <nav class="bg-white/80 backdrop-blur-md border-b border-gray-200/50 sticky top-0 z-50">
        <div class="max-w-4xl mx-auto px-6 py-4">
            <div class="flex items-center justify-between">
                <div class="flex items-center space-x-3">
                    <div class="w-8 h-8 bg-gradient-to-br from-orange-400 to-red-500 rounded-lg flex items-center justify-center">
                        <span class="text-white font-bold text-sm">🦀</span>
                    </div>
                    <h1 class="text-xl font-semibold text-gray-900">Rust API</h1>
                </div>
                <div class="flex items-center space-x-2 text-sm text-gray-600">
                    <span class="bg-green-100 text-green-700 px-2 py-1 rounded-full font-medium">Online</span>
                    <span>v1.0.0</span>
                </div>
            </div>
        </div>
    </nav>

    <!-- Main Content -->
    <div class="max-w-4xl mx-auto px-6 py-8 animate-fade-in">
        <!-- Hero Section -->
        <div class="text-center mb-12">
            <h1 class="text-4xl md:text-5xl font-bold text-gray-900 mb-4">
                Modern <span class="bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">Calculator API</span>
            </h1>
            <p class="text-xl text-gray-600 max-w-2xl mx-auto">
                A sleek, fast calculator built with Rust and powered by the Warp framework
            </p>
        </div>

        <!-- Calculator Card -->
        <div class="bg-white/70 backdrop-blur-sm rounded-3xl shadow-xl border border-gray-200/50 p-8 mb-8 animate-slide-up">
            <div class="text-center">
                <div class="w-16 h-16 bg-gradient-to-br from-blue-500 to-purple-600 rounded-2xl flex items-center justify-center mx-auto mb-6">
                    <svg class="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 7h6m0 10v-3m-3 3h.01M9 17h.01M9 14h.01M12 14h.01M15 11h.01M12 11h.01M9 11h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z"></path>
                    </svg>
                </div>
                
                <h2 class="text-2xl font-semibold text-gray-900 mb-2">Square Calculator</h2>
                <p class="text-gray-600 mb-8">Enter any number to calculate its square instantly</p>
                
                <!-- Input Section -->
                <div class="max-w-md mx-auto space-y-6">
                    <div class="relative">
                        <input 
                            type="number" 
                            id="numberInput" 
                            placeholder="Enter a number..."
                            class="w-full px-6 py-4 text-lg text-center border-2 border-gray-200 rounded-2xl focus:border-blue-500 focus:ring-4 focus:ring-blue-100 transition-all duration-200 outline-none font-medium"
                        />
                    </div>
                    
                    <button 
                        onclick="calculateSquare()"
                        class="w-full bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-700 hover:to-purple-700 text-white font-semibold py-4 px-8 rounded-2xl transition-all duration-200 transform hover:scale-[1.02] hover:shadow-lg active:scale-[0.98]"
                    >
                        <span class="flex items-center justify-center space-x-2">
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
                            </svg>
                            <span>Calculate Square</span>
                        </span>
                    </button>
                </div>
                
                <!-- Result Display -->
                <div id="result" class="mt-8 p-6 rounded-2xl font-semibold text-lg animate-slide-up" style="display: none;"></div>
            </div>
        </div>

        <!-- API Information Card -->
        <div class="bg-white/70 backdrop-blur-sm rounded-3xl shadow-xl border border-gray-200/50 p-8">
            <div class="flex items-center space-x-3 mb-6">
                <div class="w-12 h-12 bg-gradient-to-br from-green-500 to-teal-600 rounded-xl flex items-center justify-center">
                    <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.367 2.684 3 3 0 00-5.367-2.684z"></path>
                    </svg>
                </div>
                <h3 class="text-2xl font-semibold text-gray-900">API Documentation</h3>
            </div>
            
            <div class="grid md:grid-cols-2 gap-6">
                <!-- Server Info -->
                <div class="space-y-4">
                    <h4 class="text-lg font-semibold text-gray-800 flex items-center space-x-2">
                        <svg class="w-5 h-5 text-blue-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01"></path>
                        </svg>
                        <span>Server Details</span>
                    </h4>
                    <div class="space-y-2 text-gray-600">
                        <p><span class="font-medium">Framework:</span> Rust + Warp</p>
                        <p><span class="font-medium">Version:</span> 1.0.0</p>
                        <p><span class="font-medium">Port:</span> 8002</p>
                        <p><span class="font-medium">Status:</span> <span class="text-green-600 font-medium">Active</span></p>
                    </div>
                </div>
                
                <!-- Endpoints -->
                <div class="space-y-4">
                    <h4 class="text-lg font-semibold text-gray-800 flex items-center space-x-2">
                        <svg class="w-5 h-5 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1"></path>
                        </svg>
                        <span>Endpoints</span>
                    </h4>
                    <div class="space-y-3">
                        <div class="bg-gray-50 rounded-xl p-4 font-mono text-sm">
                            <div class="flex items-center space-x-2 mb-1">
                                <span class="bg-blue-100 text-blue-700 px-2 py-1 rounded text-xs font-semibold">GET</span>
                                <span class="text-gray-800">/</span>
                            </div>
                            <p class="text-gray-600 text-xs">This beautiful web interface</p>
                        </div>
                        <div class="bg-gray-50 rounded-xl p-4 font-mono text-sm">
                            <div class="flex items-center space-x-2 mb-1">
                                <span class="bg-green-100 text-green-700 px-2 py-1 rounded text-xs font-semibold">POST</span>
                                <span class="text-gray-800">/api/square</span>
                            </div>
                            <p class="text-gray-600 text-xs">Calculate square of a number</p>
                        </div>
                    </div>
                </div>
            </div>
            
            <!-- Code Example -->
            <div class="mt-8">
                <h4 class="text-lg font-semibold text-gray-800 mb-4 flex items-center space-x-2">
                    <svg class="w-5 h-5 text-purple-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"></path>
                    </svg>
                    <span>Example Usage</span>
                </h4>
                <div class="bg-gray-900 rounded-2xl p-6 font-mono text-sm overflow-x-auto">
                    <div class="text-gray-300">
                        <span class="text-green-400">curl</span> <span class="text-blue-400">-X POST</span> <span class="text-yellow-300">http://localhost:8002/api/square</span> <span class="text-gray-500">\</span><br>
                        &nbsp;&nbsp;<span class="text-blue-400">-H</span> <span class="text-yellow-300">"Content-Type: application/json"</span> <span class="text-gray-500">\</span><br>
                        &nbsp;&nbsp;<span class="text-blue-400">-d</span> <span class="text-yellow-300">'{"number": 7}'</span>
                    </div>
                </div>
            </div>
        </div>
    </div>

    <!-- Footer -->
    <footer class="mt-16 py-8 text-center text-gray-500">
        <p class="flex items-center justify-center space-x-2">
            <span>Built with</span>
            <span class="text-red-500">❤️</span>
            <span>using Rust, Tailwind CSS, and Google Fonts</span>
        </p>
    </footer>


    <script>
        async function calculateSquare() {
            const input = document.getElementById('numberInput');
            const resultDiv = document.getElementById('result');
            
            const number = parseInt(input.value);
            
            if (isNaN(number)) {
                resultDiv.innerHTML = `
                    <div class="flex items-center justify-center space-x-3">
                        <div class="w-10 h-10 bg-red-100 rounded-full flex items-center justify-center">
                            <svg class="w-5 h-5 text-red-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                            </svg>
                        </div>
                        <div>
                            <p class="text-red-700 font-semibold">Invalid Input</p>
                            <p class="text-red-600 text-sm">Please enter a valid number</p>
                        </div>
                    </div>
                `;
                resultDiv.className = 'mt-8 p-6 bg-red-50 border border-red-200 rounded-2xl font-semibold text-lg animate-slide-up';
                resultDiv.style.display = 'block';
                return;
            }

            // Show loading state
            resultDiv.innerHTML = `
                <div class="flex items-center justify-center space-x-3">
                    <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
                    <span class="text-gray-600">Calculating...</span>
                </div>
            `;
            resultDiv.className = 'mt-8 p-6 bg-gray-50 border border-gray-200 rounded-2xl font-semibold text-lg animate-slide-up';
            resultDiv.style.display = 'block';

            try {
                const response = await fetch('/api/square', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({ number: number })
                });

                const data = await response.json();
                
                setTimeout(() => {
                    resultDiv.innerHTML = `
                        <div class="flex items-center justify-center space-x-4">
                            <div class="w-12 h-12 bg-green-100 rounded-full flex items-center justify-center">
                                <svg class="w-6 h-6 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                                </svg>
                            </div>
                            <div class="text-center">
                                <p class="text-green-700 font-semibold text-lg">Success!</p>
                                <p class="text-gray-600 text-sm mb-2">${data.message}</p>
                                <p class="text-3xl font-bold text-gray-900">${number}² = <span class="text-blue-600">${data.result}</span></p>
                            </div>
                        </div>
                    `;
                    resultDiv.className = 'mt-8 p-8 bg-green-50 border border-green-200 rounded-2xl font-semibold text-lg animate-slide-up';
                }, 500);
                
            } catch (error) {
                resultDiv.innerHTML = `
                    <div class="flex items-center justify-center space-x-3">
                        <div class="w-10 h-10 bg-red-100 rounded-full flex items-center justify-center">
                            <svg class="w-5 h-5 text-red-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                            </svg>
                        </div>
                        <div>
                            <p class="text-red-700 font-semibold">Error</p>
                            <p class="text-red-600 text-sm">Failed to calculate result</p>
                        </div>
                    </div>
                `;
                resultDiv.className = 'mt-8 p-6 bg-red-50 border border-red-200 rounded-2xl font-semibold text-lg animate-slide-up';
            }
        }

        // Allow Enter key to calculate
        document.getElementById('numberInput').addEventListener('keypress', function(e) {
            if (e.key === 'Enter') {
                calculateSquare();
            }
        });

        // Add some nice input animations
        document.getElementById('numberInput').addEventListener('focus', function(e) {
            e.target.classList.add('scale-105');
        });

        document.getElementById('numberInput').addEventListener('blur', function(e) {
            e.target.classList.remove('scale-105');
        });
    </script>
</body>
</html>
            "#)
        });

    // API endpoint to calculate square
    let api_square = warp::path!("api" / "square")
        .and(warp::post())
        .and(warp::body::json())
        .map(|req: NumberRequest| {
            let result = calculate_square(req.number);
            let response = ApiResponse {
                message: format!("Square of {} is calculated successfully", req.number),
                result,
            };
            warp::reply::json(&response)
        });

    // CORS for web requests
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST"]);

    // Combine routes
    let routes = html_page.or(api_square).with(cors);

    println!("🚀 Rust API Server Starting...");
    println!("📍 Server running on: http://localhost:8002");
    println!("🌐 Open your browser and visit: http://localhost:8002");
    println!("📱 Simple Square Calculator is ready!");
    println!("💡 Try the API: POST /api/square with JSON body");

    warp::serve(routes).bind(([0, 0, 0, 0], 8002)).await;
}
