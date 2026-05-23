//! Comprehensive tests for pizza-analysis-portuguese.

use pizza_analysis_portuguese::*;
use pizza_engine::analysis::{AnalysisFactory, Token, TokenFilter};

fn make_token(term: &str) -> Token<'_> {
    Token::new(term, 0, term.len() as u32, 0)
}

// ═══════════════════════════════════════════════════════════════════════════════
// PortugueseLightStemFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stem_construction() {
    let _f = PortugueseLightStemFilter::new();
}

#[test]
fn stem_plural_s() {
    let f = PortugueseLightStemFilter::new();
    // "gatos" (cats) → stem
    let mut token = make_token("gatos");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_ne!(token.term.as_ref(), "gatos");
}

#[test]
fn stem_plural_es() {
    let f = PortugueseLightStemFilter::new();
    // "flores" (flowers) → stem
    let mut token = make_token("flores");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_feminine() {
    let f = PortugueseLightStemFilter::new();
    // "bonita" (pretty, f.) → stem
    let mut token = make_token("bonita");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_verb_form() {
    let f = PortugueseLightStemFilter::new();
    let mut token = make_token("correndo");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_augmentative() {
    let f = PortugueseLightStemFilter::new();
    let mut token = make_token("casarão");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_diminutive() {
    let f = PortugueseLightStemFilter::new();
    // "gatinho" (little cat) → stem
    let mut token = make_token("gatinho");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_short_word() {
    let f = PortugueseLightStemFilter::new();
    let mut token = make_token("eu");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_empty_string() {
    let f = PortugueseLightStemFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_single_char() {
    let f = PortugueseLightStemFilter::new();
    let mut token = make_token("a");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// PortugueseStopFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stop_construction() {
    let _f = PortugueseStopFilter::new();
}

#[test]
fn stop_filters_common_words() {
    let f = PortugueseStopFilter::new();
    let stop_words = ["de", "a", "o", "que", "e", "do", "da", "em", "um", "para", "com", "não"];
    for word in &stop_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted, "stop word '{}' should be filtered", word);
    }
}

#[test]
fn stop_keeps_content_words() {
    let f = PortugueseStopFilter::new();
    let content_words = ["casa", "livro", "escola", "cidade"];
    for word in &content_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted, "content word '{}' should be kept", word);
    }
}

#[test]
fn stop_empty_string() {
    let f = PortugueseStopFilter::new();
    let mut token = make_token("");
    let _ = f.filter(&mut token);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Registration
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn register_all_no_panic() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
}

#[test]
fn register_all_filters_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_token_filter("portuguese_light_stem").is_some());
    assert!(factory.get_token_filter("portuguese_stop").is_some());
}

#[test]
fn register_all_analyzer_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_analyzer("portuguese").is_some());
}

#[test]
fn analyzer_pipeline_produces_tokens() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("portuguese").unwrap();
    let mut input = String::from("O gato está na casa grande");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}

#[test]
fn analyzer_pipeline_removes_stops() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("portuguese").unwrap();
    let mut input = String::from("o livro da escola");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    let terms: Vec<&str> = tokens.iter().map(|t| t.term.as_ref()).collect();
    assert!(!terms.contains(&"o"));
    assert!(!terms.contains(&"da"));
}

#[test]
fn analyzer_pipeline_empty_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("portuguese").unwrap();
    let mut input = String::from("");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(tokens.is_empty());
}

#[test]
fn analyzer_pipeline_single_word() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("portuguese").unwrap();
    let mut input = String::from("Lisboa");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}

#[test]
fn analyzer_pipeline_ascii_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("portuguese").unwrap();
    let mut input = String::from("hello world");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}
