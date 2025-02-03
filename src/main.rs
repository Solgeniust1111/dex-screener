use dex_checker::{
    commons::utils::HypoMaxVal,
    handlers::{
        fetch_dex_data::fetch_dex_data, find_highest_dex::find_highest_dex,
        parse_dex_pairs::parse_dex_pairs,
    },
};
use std::env;
use tokio::time::Instant;

#[tokio::main]
async fn main() {
    // Get the arguments passed to the program
    let args: Vec<String> = env::args().collect();

    // Ensure the mint address is provided
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <mint_address>");
        return;
    }

    let mint = &args[1]; // Get the mint address from the command line
    let start_time = Instant::now();

    match fetch_dex_data(mint).await {
        Ok(json) => {
            if let Some(pairs) = json["pairs"].as_array() {
                let dex_infos = parse_dex_pairs(pairs);
                if !dex_infos.is_empty() {
                    let hypo_max_val = HypoMaxVal::calculate_hypo_max_val(dex_infos.clone());
                    if let Some(highest_dex_info) = find_highest_dex(&dex_infos, &hypo_max_val) {
                        println!("Dex with the highest score: {:#?}", highest_dex_info);
                    } else {
                        println!("No DexInfo entries found.");
                    }
                } else {
                    println!("No DEX information could be parsed.");
                }
            } else {
                eprintln!("No 'pairs' found in the JSON response");
            }
        }
        Err(e) => eprintln!("Error retrieving dex data: {}", e),
    }

    println!("Process has completed in {:?}", start_time.elapsed());
}
