/// This module contains unit tests for the fetch_dex_data function in the dex_checker library.
///
/// It includes tests for:
/// - Fetching Dex data from the blockchain using a mint address.
/// - Validating successful API response.
use dex_checker::handlers::fetch_dex_data::fetch_dex_data;

#[tokio::test]
async fn test_fetch_dex_data() {
    // Test the fetch_dex_data function to ensure it returns valid Dex data for a given mint address.
    let mint = "E6AujzX54E1ZoPDFP2CyG3HHUVKygEkp6DRqig61pump";

    // Mock the API response here if necessary or run against a test endpoint
    let result = fetch_dex_data(mint).await;
    assert!(result.is_ok(), "Should successfully fetch Dex data");
}
