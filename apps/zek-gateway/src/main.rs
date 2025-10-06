use anyhow::Result;
use axum::{
    routing::{get, get_service},
    Router,
    http::StatusCode,
    response::Html,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();
    
    // Serve static files from the web UI directory
    let serve_dir = ServeDir::new("../clients/check-web-ui/dist");
    
    let app = Router::new()
        .route("/", get(root))
        .route("/dashboard", get(dashboard))
        .nest_service("/static", serve_dir.clone())
        .fallback_service(serve_dir)
        .route("/api/health", get(health_check));
    
    let addr: SocketAddr = "127.0.0.1:7001".parse()?;
    tracing::info!("Gateway http://{addr}");
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
    Ok(())
}

async fn root() -> Html<&'static str> {
    Html(r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Zek System Monitor</title>
        <style>
            body { font-family: Arial, sans-serif; margin: 40px; }
            .header { color: #333; }
            .nav { margin: 20px 0; }
            .nav a { margin-right: 15px; text-decoration: none; color: #007bff; }
            .nav a:hover { text-decoration: underline; }
        </style>
    </head>
    <body>
        <h1 class="header">Zek System Monitor</h1>
        <div class="nav">
            <a href="/dashboard">Dashboard</a>
            <a href="/api/health">API Health</a>
        </div>
        <p>Welcome to Zek System Monitor. This is the gateway service for the monitoring system.</p>
        <p>To access the web dashboard, click on the Dashboard link above or navigate to <a href="/dashboard">/dashboard</a>.</p>
    </body>
    </html>
    "#)
}

async fn dashboard() -> Html<&'static str> {
    Html(r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Zek Dashboard</title>
        <style>
            body { font-family: Arial, sans-serif; margin: 0; padding: 20px; background-color: #f5f5f5; }
            .container { max-width: 1200px; margin: 0 auto; }
            .header { background: #333; color: white; padding: 20px; border-radius: 5px; margin-bottom: 20px; }
            .dashboard { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; }
            .card { background: white; padding: 20px; border-radius: 5px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
            .metric { margin: 10px 0; }
            .metric-label { font-weight: bold; }
            .metric-value { font-size: 1.2em; color: #007bff; }
        </style>
    </head>
    <body>
        <div class="container">
            <div class="header">
                <h1>Zek System Monitor Dashboard</h1>
                <p>Real-time system metrics visualization</p>
            </div>
            <div class="dashboard">
                <div class="card">
                    <h2>CPU Usage</h2>
                    <div class="metric">
                        <div class="metric-label">Overall:</div>
                        <div class="metric-value" id="cpu-total">--%</div>
                    </div>
                    <div class="metric">
                        <div class="metric-label">Core 1:</div>
                        <div class="metric-value" id="cpu-core-1">--%</div>
                    </div>
                </div>
                <div class="card">
                    <h2>Memory</h2>
                    <div class="metric">
                        <div class="metric-label">Used:</div>
                        <div class="metric-value" id="mem-used">-- MB</div>
                    </div>
                    <div class="metric">
                        <div class="metric-label">Total:</div>
                        <div class="metric-value" id="mem-total">-- MB</div>
                    </div>
                </div>
                <div class="card">
                    <h2>Network</h2>
                    <div class="metric">
                        <div class="metric-label">Interface:</div>
                        <div class="metric-value" id="net-interface">--</div>
                    </div>
                    <div class="metric">
                        <div class="metric-label">RX:</div>
                        <div class="metric-value" id="net-rx">-- KB/s</div>
                    </div>
                    <div class="metric">
                        <div class="metric-label">TX:</div>
                        <div class="metric-value" id="net-tx">-- KB/s</div>
                    </div>
                </div>
            </div>
        </div>
        <script>
            // Simple polling to update dashboard
            async function updateDashboard() {
                try {
                    const response = await fetch('/api/snapshot');
                    const data = await response.json();
                    
                    if (data.status !== "no-data-yet") {
                        document.getElementById('cpu-total').textContent = data.cpu_total_pct.toFixed(1) + '%';
                        if (data.cpu_per_core && data.cpu_per_core.length > 0) {
                            document.getElementById('cpu-core-1').textContent = data.cpu_per_core[0].toFixed(1) + '%';
                        }
                        
                        const memUsed = Math.round(data.mem_used_bytes / (1024 * 1024));
                        const memTotal = Math.round(data.mem_total_bytes / (1024 * 1024));
                        document.getElementById('mem-used').textContent = memUsed + ' MB';
                        document.getElementById('mem-total').textContent = memTotal + ' MB';
                        
                        if (data.network && data.network.length > 0) {
                            const net = data.network[0];
                            document.getElementById('net-interface').textContent = net.interface;
                            document.getElementById('net-rx').textContent = (net.rx_throughput / 1024).toFixed(2) + ' KB/s';
                            document.getElementById('net-tx').textContent = (net.tx_throughput / 1024).toFixed(2) + ' KB/s';
                        }
                    }
                } catch (error) {
                    console.error('Error updating dashboard:', error);
                }
            }
            
            // Update every 2 seconds
            setInterval(updateDashboard, 2000);
            // Initial update
            updateDashboard();
        </script>
    </body>
    </html>
    "#)
}

async fn health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "OK")
}