use std::cmp;

/// A `VariableValue` is the value that a entity config variable can have.
/// It is based on JSON values.
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum VariableValue {
    #[serde(alias = "null")]
    Null,
    Bool(bool),
    Num(f32),
    Str(String),
}

impl PartialEq for VariableValue {
    fn eq(&self, other: &Self) -> bool {
        if let Some(cmp::Ordering::Equal) = self.partial_cmp(other) {
            true
        } else {
            false
        }
    }
}

impl Eq for VariableValue {
}

impl PartialOrd for VariableValue {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match (self, other) {
            (Self::Null, Self::Null) => Some(cmp::Ordering::Equal),
            (Self::Bool(a), Self::Bool(b)) => a.partial_cmp(b),
            (Self::Num(a), Self::Num(b)) => a.partial_cmp(b),
            (Self::Str(a), Self::Str(b)) => a.partial_cmp(b),
            (Self::Null, _) | (_, Self::Null) => None,
            (_, _) => {
                eprintln!(
                    "[WARNING]\n    Can't compare `VariableValue`s of \
                     different types:\n    `{:?}` and `{:?}`",
                    self, other,
                );
                None
            }
        }
    }
}

impl Ord for VariableValue {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if let Some(ordering) = self.partial_cmp(other) {
            ordering
        } else {
            panic!(
                "Can't compare `VariableValue`s:\n`{:?}` and `{:?}`",
                self, other,
            );
        }
    }
}
