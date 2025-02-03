use std::collections::HashMap;

use crate::commons::utils::{DexInfo, HypoMaxVal};

pub fn find_highest_dex<'a>(
    dex_infos: &'a [DexInfo],
    hypo_max_val: &'a HypoMaxVal,
) -> Option<&'a DexInfo> {
    let mut dex_scores: HashMap<u32, f64> = HashMap::new();

    for (index, dex_info) in dex_infos.iter().enumerate() {
        let score = dex_info.calculate_score(hypo_max_val.clone());
        dex_scores.insert(index as u32, score);
    }

    // Find the maximum score and the corresponding DexInfo
    dex_scores
        .iter()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(&index, _)| &dex_infos[index as usize])
}
