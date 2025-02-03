use serde_json::Value;

use crate::commons::utils::{DexInfo, Liquidity, Volume};

pub fn parse_dex_pairs(pairs: &Vec<Value>) -> Vec<DexInfo> {
    let mut dex_infos = Vec::new();

    for pair in pairs {
        let dex_id = pair["dexId"].as_str().unwrap_or_default().to_string();
        let pair_address = pair["pairAddress"].as_str().unwrap_or_default().to_string();

        let price_usd = pair["priceUsd"]
            .as_str()
            .unwrap_or("0")
            .parse::<f64>()
            .unwrap_or(0.0);
        let market_cap = pair["marketCap"].as_f64().unwrap_or_default();

        let liquidity = Liquidity {
            base: pair["liquidity"]["base"].as_f64().unwrap_or_default(),
            quote: pair["liquidity"]["quote"].as_f64().unwrap_or_default(),
            usd: pair["liquidity"]["usd"].as_f64().unwrap_or_default(),
        };

        let volume = Volume {
            h1: pair["volume"]["h1"].as_f64().unwrap_or_default(),
            h24: pair["volume"]["h24"].as_f64().unwrap_or_default(),
            h6: pair["volume"]["h6"].as_f64().unwrap_or_default(),
            m5: pair["volume"]["m5"].as_f64().unwrap_or_default(),
        };

        // Attempt to create DexInfo and add it to the list
        if let Ok(dex_info) = DexInfo::new(
            dex_id,
            price_usd,
            market_cap,
            liquidity,
            volume,
            pair_address,
        ) {
            dex_infos.push(dex_info);
        } else {
            eprintln!("Error creating dex info");
        }
    }

    dex_infos
}
