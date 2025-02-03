use dex_checker::handlers::parse_dex_pairs::parse_dex_pairs;
/// This module contains unit tests for the `parse_dex_pairs` function in the dex_checker library.
///
/// It includes tests for:
/// - Parsing of DEX pair data.
/// - Validation of expected attributes such as price and liquidity.
use serde_json::json;

#[test]
fn test_parse_dex_pairs() {
    // Test parsing DEX pairs for validity and correctness.
    //
    // This test verifies that the function `parse_dex_pairs` correctly parses a list of DEX pair
    // JSON objects, extracts crucial information, and ensures the data can be accurately
    // retrieved using its expected structure.
    let pairs = vec![
        json!({
            "dexId": "dex1",
            "pairAddress": "0x123",
            "priceUsd": "1.0",
            "marketCap": 500000.0,
            "liquidity": {
                "base": 1000.0,
                "quote": 2000.0,
                "usd": 3000.0
            },
            "volume": {
                "h1": 100.0,
                "h24": 200.0,
                "h6": 150.0,
                "m5": 50.0
            }
        }),
        json!({
            "dexId": "dex2",
            "pairAddress": "0x456",
            "priceUsd": "2.0",
            "marketCap": 1000000.0,
            "liquidity": {
                "base": 2000.0,
                "quote": 3000.0,
                "usd": 4000.0
            },
            "volume": {
                "h1": 200.0,
                "h24": 300.0,
                "h6": 250.0,
                "m5": 150.0
            }
        }),
    ];

    let dex_infos = parse_dex_pairs(&pairs);
    assert_eq!(dex_infos.len(), 2);
    assert_eq!(
        dex_infos[0].price_usd, 1.0,
        "Expected price for dex1 not matched."
    );
    assert_eq!(
        dex_infos[1].liquidity.base, 2000.0,
        "Expected liquidity base for dex2 not matched."
    );
}
