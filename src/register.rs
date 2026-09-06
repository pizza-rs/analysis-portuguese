//! Register Portuguese analysis components into [`AnalysisFactory`].

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use pizza_engine::analysis::AnalysisFactory;
use pizza_engine::analysis::Analyzer;
use pizza_engine::analysis::LowercaseNormalizer;
use pizza_engine::analysis::Normalizer;
use pizza_engine::analysis::StandardTokenizer;
use pizza_engine::analysis::TokenFilter;
use pizza_engine::analysis::Tokenizer;

use crate::PortugueseLightStemFilter;
use crate::PortugueseStopFilter;

/// Register Portuguese token filters and the `"portuguese"` analyzer.
pub fn register_all(factory: &mut AnalysisFactory) {
    factory.register_token_filter(
        "portuguese_light_stem",
        Box::new(PortugueseLightStemFilter::new()),
    );
    factory.register_token_filter("portuguese_stop", Box::new(PortugueseStopFilter::new()));

    let normalizers: Vec<Box<dyn Normalizer>> = vec![Box::new(LowercaseNormalizer::new())];
    let tokenizer: Box<dyn Tokenizer> = Box::new(StandardTokenizer::new());
    let filters: Vec<Box<dyn TokenFilter>> = vec![
        Box::new(PortugueseStopFilter::new()),
        Box::new(PortugueseLightStemFilter::new()),
    ];
    factory.register_analyzer("portuguese", Analyzer::new(normalizers, tokenizer, filters));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_all_no_panic() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
    }

    #[test]
    fn test_filters_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_token_filter("portuguese_light_stem").is_some());
        assert!(factory.get_token_filter("portuguese_stop").is_some());
    }

    #[test]
    fn test_analyzer_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_analyzer("portuguese").is_some());
    }

    #[test]
    fn test_analyzer_pipeline() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        let analyzer = factory.get_analyzer("portuguese").unwrap();
        let mut input = String::from("O gato está na casa");
        let tokens = analyzer.analyze_and_return_tokens(&mut input);
        assert!(!tokens.iter().any(|t| t.term == "o"));
        assert!(!tokens.iter().any(|t| t.term == "na"));
        assert!(tokens.len() >= 2);
    }
}
