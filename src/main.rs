#[tokio::main]
async fn main() {
    simple_logger::SimpleLogger::new().with_colors(true).with_level(log::LevelFilter::Info).init().unwrap();
}
