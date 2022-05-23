mod forecast_zones;
mod product_list;

fn main() -> Result<(), anyhow::Error> {
    match std::env::args().nth(1) {
        Some(x) if x == "zones" => forecast_zones::main(),
        Some(x) if x == "products" => product_list::main(),
        _ => {
            println!("Usage:  cargo xtask <task>");
            println!();
            println!("Tasks is one of the following:  zones products");
            Ok(())
        }
    }
}
