#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::gift::recommendation::{GiftRecommendation, GiftItem};

    #[test]
    fn test_search_results() {
        let recommendation_system = GiftRecommendation::new();
        let results = recommendation_system.search("就職祝い", "友人", 30000);
        assert!(!results.is_empty(), "検索結果は空ではないこと");
    }

    #[test]
    fn test_filter_criteria() {
        let recommendation_system = GiftRecommendation::new();
        let mut results = recommendation_system.search("就職祝い", "上司", 50000);
        results = recommendation_system.filter_results(results, |item| item.price <= 50000);
        assert!(
            results.iter().all(|item| item.price <= 50000),
            "すべてのアイテムが指定された価格範囲内であること"
        );
    }

    #[test]
    fn test_proposal_count() {
        let recommendation_system = GiftRecommendation::new();
        let results = recommendation_system.search("就職祝い", "親戚", 20000);
        assert_eq!(
            results.len(),
            3,
            "提案されるギフトの数は正確に3つであること"
        );
    }
};