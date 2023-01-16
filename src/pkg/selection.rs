use super::*;

/// Operator represents a key/field's relationship to value(s).
/// See labels.Requirement and fields.Requirement for more details.
///

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    DoesNotExist,
    Equals,
    DoubleEquals,
    In,
    NotEquals,
    NotIn,
    Exists,
    GreaterThan,
    LessThan,
}

impl Operator {
    pub fn as_str(&self) -> &str {
        match self {
            Self::DoesNotExist => "!",
            Self::Equals => "=",
            Self::DoubleEquals => "==",
            Self::In => "in",
            Self::NotEquals => "!=",
            Self::NotIn => "notin",
            Self::Exists => "exists",
            Self::GreaterThan => "gt",
            Self::LessThan => "lt",
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl str::FromStr for Operator {
    type Err = InvalidOperator;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "!" => Ok(Self::DoesNotExist),
            "=" => Ok(Self::Equals),
            "==" => Ok(Self::DoubleEquals),
            "in" => Ok(Self::In),
            "!=" => Ok(Self::NotEquals),
            "notin" => Ok(Self::NotIn),
            "exists" => Ok(Self::Exists),
            "gt" => Ok(Self::GreaterThan),
            "lt" => Ok(Self::LessThan),
            _ => Err(InvalidOperator),
        }
    }
}

#[derive(Debug)]
pub struct InvalidOperator;
