/*!
This module contains unit tests for the dex_checker library's functionality, specifically focusing on the `find_highest_dex` function.

It includes tests for:
- Finding the highest DEX based on given data.
- Verification of DEX properties such as ID and trading volume.
- Ensuring the DEX selection logic works correctly with different input scenarios.
*/

use dex_checker::{
    commons::utils::{DexInfo, HypoMaxVal, Liquidity, Volume},
    handlers::find_highest_dex::find_highest_dex,
};

#[test]
fn test_find_highest_dex() {
    /*!
    This test checks the functionality of the `find_highest_dex` function to ensure it correctly identifies
    the DEX with the highest volume from a set of DEX information.

    The test creates two DEX instances with predefined characteristics and verifies that the
    function correctly returns the one with the highest volume.
    */

    let hypo_max_val = HypoMaxVal::default(); // Create a default instance
    let dex_infos = vec![
        DexInfo::new(
            "dex1".to_string(),
            1.0,
            500000.0,
            Liquidity::default(),
            Volume::default(),
            "0x123".to_string(),
        )
        .unwrap(),
        DexInfo::new(
            "dex2".to_string(),
            2.0,
            1000000.0,
            Liquidity::default(),
            Volume::default(),
            "0x456".to_string(),
        )
        .unwrap(),
    ];

    let highest_dex_info = find_highest_dex(&dex_infos, &hypo_max_val);
    assert!(
        highest_dex_info.is_some(),
        "Expected to find a DEX with the highest volume."
    );
    assert_eq!(
        highest_dex_info.unwrap().dex_id,
        "dex2",
        "Expected dex2 to be the DEX with the highest volume."
    );
}
