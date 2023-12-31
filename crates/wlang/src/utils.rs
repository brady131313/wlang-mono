use crate::lexer::TokenKind;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TokenSet(u32);

impl TokenSet {
    pub const fn with_kind(self, kind: TokenKind) -> Self {
        Self(self.0 | (1 << kind as u32))
    }

    pub const fn is_set(self, kind: TokenKind) -> bool {
        (self.0 & (1 << kind as u32)) != 0
    }

    pub const fn from_array<const N: usize>(kinds: [TokenKind; N]) -> Self {
        token_set_from_array(&kinds, 0, TokenSet(0))
    }

    pub fn kinds(&self) -> Vec<TokenKind> {
        let mut kinds = Vec::new();
        for i in 0..(TokenKind::Error as u8 + 1) {
            let kind = unsafe { std::mem::transmute::<u8, TokenKind>(i) };
            if self.is_set(kind) {
                kinds.push(kind);
            }
        }

        kinds
    }
}

const fn token_set_from_array(kinds: &[TokenKind], idx: usize, current: TokenSet) -> TokenSet {
    if idx == kinds.len() {
        current
    } else {
        token_set_from_array(kinds, idx + 1, current.with_kind(kinds[idx]))
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for TokenSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let kinds = self.kinds();
        kinds.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for TokenSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let kinds = Vec::<TokenKind>::deserialize(deserializer)?;
        let mut set = TokenSet::default();
        for kind in kinds {
            set = set.with_kind(kind);
        }

        Ok(set)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_set() {
        let set = TokenSet::default();
        assert!(!set.is_set(TokenKind::Ident));

        let set = set.with_kind(TokenKind::Ident);
        assert!(set.is_set(TokenKind::Ident));
    }

    #[test]
    fn token_set_from_array() {
        let set = TokenSet::from_array([TokenKind::Ident, TokenKind::Float]);
        assert!(set.is_set(TokenKind::Ident));
        assert!(set.is_set(TokenKind::Float));
        assert!(!set.is_set(TokenKind::Hour));
        assert!(!set.is_set(TokenKind::Newline));
    }
}
