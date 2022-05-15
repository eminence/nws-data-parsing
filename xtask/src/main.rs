mod forecast_zones;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match std::env::args().nth(1) {
        Some(x) if x == "zones" => forecast_zones::main(),
        _ => {
            println!("Usage:  cargo xtask zones");
            Ok(())
        }
    }
}
