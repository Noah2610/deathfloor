use std::cmp;
use std::fmt;

/// An `ExpressionValue` is the value that an entity config variable can have.
/// It is based on JSON values.
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum ExpressionValue {
    #[serde(alias = "null")]
    Null,
    Bool(bool),
    Num(f32),
    Str(String),
}

impl ExpressionValue {
    /// Returns `true` if this value is considered to be "truthy".
    /// For us, only `Null` and `false` are "falsy" values,
    /// every other value is considered "truthy".
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Null => false,
            Self::Bool(b) => *b,
            Self::Num(_) => true,
            Self::Str(_) => true,
        }
    }
}

impl PartialEq for ExpressionValue {
    fn eq(&self, other: &Self) -> bool {
        if let Some(cmp::Ordering::Equal) = self.partial_cmp(other) {
            true
        } else {
            false
        }
    }
}

impl Eq for ExpressionValue {
}

impl PartialOrd for ExpressionValue {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match (self, other) {
            (Self::Null, Self::Null) => Some(cmp::Ordering::Equal),
            (Self::Bool(a), Self::Bool(b)) => a.partial_cmp(b),
            (Self::Num(a), Self::Num(b)) => a.partial_cmp(b),
            (Self::Str(a), Self::Str(b)) => a.partial_cmp(b),
            (Self::Null, _) | (_, Self::Null) => None,
            (_, _) => {
                eprintln!(
                    "[WARNING]\n    Can't compare `ExpressionValue`s of \
                     different types:\n    `{:?}` and `{:?}`",
                    self, other,
                );
                None
            }
        }
    }
}

impl Ord for ExpressionValue {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if let Some(ordering) = self.partial_cmp(other) {
            ordering
        } else {
            panic!(
                "Can't compare `ExpressionValue`s:\n`{:?}` and `{:?}`",
                self, other,
            );
        }
    }
}

impl fmt::Display for ExpressionValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Null => write!(f, "Null"),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Num(n) => write!(f, "{}", n),
            Self::Str(s) => write!(f, "\"{}\"", s),
        }
    }
}
