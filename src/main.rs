use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use union_back::{config, create_app};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let config = config::load_config()?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        //.connect(&config.database_url)
        .connect("postgres://spl:spl@localhost/news_db")
        .await?;

    let app = create_app(pool);

    //let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = TcpListener::bind(addr).await?;

    println!("🚀 Server running on http://{}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
