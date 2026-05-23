//! Portuguese light stemmer.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::{Token, TokenFilter};

/// Portuguese light stemmer — removes plural and inflectional suffixes.
#[derive(Clone, Debug, Default)]
pub struct PortugueseLightStemFilter;

impl PortugueseLightStemFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for PortugueseLightStemFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        if text.len() < 5 {
            return (false, None);
        }
        let stemmed = stem_portuguese_light(text);
        if stemmed != text {
            token.term = Cow::Owned(stemmed);
        }
        (false, None)
    }
}

fn stem_portuguese_light(word: &str) -> String {
    let mut result = String::from(word);

    if result.ends_with("ões") || result.ends_with("ães") {
        result.truncate(result.len() - "ões".len());
        result.push_str("ão");
        return result;
    }
    if result.ends_with("ais") {
        result.truncate(result.len() - 3);
        result.push_str("al");
        return result;
    }
    if result.ends_with("éis") {
        result.truncate(result.len() - "éis".len());
        result.push_str("el");
        return result;
    }
    if result.ends_with("eis") {
        result.truncate(result.len() - 3);
        result.push_str("el");
        return result;
    }
    if result.ends_with("os") || result.ends_with("as") || result.ends_with("es") {
        result.truncate(result.len() - 2);
        return result;
    }
    if result.ends_with('s') {
        result.pop();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plural_oes() {
        let f = PortugueseLightStemFilter::new();
        let mut token = Token::new("nações", 0, 9, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "nação");
    }

    #[test]
    fn test_plural_os() {
        let f = PortugueseLightStemFilter::new();
        let mut token = Token::new("gatos", 0, 5, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "gato");
    }
}
