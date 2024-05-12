use env_logger::Env;
use shylock_data::AuctionState;
use shylock_parser::http::UrlFetcher;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    let http_client = &UrlFetcher::new();
    let auction_link =
        "https://subastas.boe.es/detalleSubasta.php?idSub=SUB-JA-2024-221182".to_string();
    let auction_info = (auction_link, AuctionState::Ongoing);

    let result = shylock_parser::scraper::process_auction_link(http_client, &auction_info).await?;

    println!("Auction: {:?}", result.0);

    for (i, asset) in result.1.iter().enumerate() {
        println!("asset {}: {:?}", i, asset);
    }

    Ok(())
}
