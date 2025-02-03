pub const SOL_PRICE: u64 = 250;
#[derive(Debug, Clone)]
pub struct Liquidity {
    pub base: f64,
    pub quote: f64,
    pub usd: f64,
}

impl Default for Liquidity {
    fn default() -> Self {
        Liquidity {
            base: 100.0,
            quote: 200.0,
            usd: 300.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Volume {
    pub h1: f64,
    pub h24: f64,
    pub h6: f64,
    pub m5: f64,
}

impl Default for Volume {
    fn default() -> Self {
        Volume {
            h1: 10.0,
            h24: 100.0,
            h6: 50.0,
            m5: 25.0,
        }
    }
}

#[derive(Clone)]
pub struct HypoMaxVal {
    pub max_price: f64,
    pub max_marketcap: f64,
    pub max_liquidity: f64,
    pub max_volume: f64,
}

impl Default for HypoMaxVal {
    fn default() -> Self {
        HypoMaxVal {
            max_price: 10.0,
            max_marketcap: 1_000_000.0,
            max_liquidity: 500_000.0,
            max_volume: 100_000.0,
        }
    }
}
/// Represents a token on a DEX (Decentralized Exchange)
#[derive(Debug, Clone)]
pub struct DexInfo {
    /// Unique identifier of the DEX
    pub dex_id: String,
    /// Price of the token in USD
    pub price_usd: f64,
    /// Market capitalization of the token
    pub marketcap: f64,
    /// Trading liquidity of the token
    pub liquidity: Liquidity,
    /// Trading volume of the token
    pub volume: Volume,
    /// Pair address of the token
    pub pair_address: String,
}

impl DexInfo {
    pub fn new(
        dex_id: String,
        price_usd: f64,
        marketcap: f64,
        liquidity: Liquidity,
        volume: Volume,
        pair_address: String,
    ) -> Result<Self, &'static str> {
        if price_usd <= 0.0 {
            return Err("Price must be greater than zero");
        }
        if marketcap <= 0.0 {
            return Err("Market capitalization must be greater than zero");
        }
        Ok(Self {
            dex_id,
            price_usd,
            marketcap,
            liquidity,
            volume,
            pair_address,
        })
    }

    pub fn calculate_score(&self, hypo_max_val: HypoMaxVal) -> f64 {
        // Assign weights
        let price_weight: f64 = 0.2;
        let marketcap_weight: f64 = 0.3;
        let liquidity_weight: f64 = 0.2;
        let volume_weight: f64 = 0.3;

        let normalized_price = self.price_usd / hypo_max_val.max_price;
        let normalized_marketcap = self.marketcap / hypo_max_val.max_marketcap;
        // Use a composite measure for liquidity, e.g., the usd liquidity
        let normalized_liquidity = self.liquidity.usd / hypo_max_val.max_liquidity;
        // Use a composite measure for volume, e.g., the h24 volume
        let normalized_volume = self.volume.h24 / hypo_max_val.max_volume;

        // Calculate the weighted sum
        let score = (price_weight * normalized_price)
            + (marketcap_weight * normalized_marketcap)
            + (liquidity_weight * normalized_liquidity)
            + (volume_weight * normalized_volume);

        // Clamp the score to ensure it's between 0 and 1
        score.clamp(0.0, 1.0)
    }
}

impl HypoMaxVal {
    pub fn calculate_hypo_max_val(dex_infos: Vec<DexInfo>) -> Self {
        let max_price = dex_infos
            .iter()
            .map(|t| t.price_usd)
            .fold(f64::MIN, f64::max);
        let max_marketcap = dex_infos
            .iter()
            .map(|t: &DexInfo| t.marketcap)
            .fold(f64::MIN, f64::max);
        let max_liquidity_usd = dex_infos
            .iter()
            .map(|t| t.liquidity.usd)
            .fold(f64::MIN, f64::max);
        let max_volume_h24 = dex_infos
            .iter()
            .map(|t| t.volume.h24)
            .fold(f64::MIN, f64::max);
        HypoMaxVal {
            max_price,
            max_marketcap,
            max_liquidity: max_liquidity_usd,
            max_volume: max_volume_h24,
        }
    }
}
