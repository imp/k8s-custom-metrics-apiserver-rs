use std::collections::BTreeSet;

use super::*;

pub trait Labels {
    /// Has returns whether the provided label exists.
    ///
    fn has(label: String) -> bool;

    /// Get returns the value for the provided label.
    ///
    fn get(label: String) -> Option<String>;
}

pub trait Selector {
    type Labels: Labels;

    /// Matches returns true if this selector matches the given set of labels.
    ///
    fn matches(&self, labels: Self::Labels) -> bool;

    /// Empty returns true if this selector does not restrict the selection space.
    ///
    fn empty(&self) -> bool;

    // // String returns a human readable string that represents this selector.
    // String() string

    /// Add adds requirements to the Selector
    ///
    fn add(&mut self, requirements: impl IntoIterator<Item = Requirement>) -> Self;

    // Requirements converts this interface into Requirements to expose
    // more detailed selection information.
    // If there are querying parameters, it will return converted requirements and selectable=true.
    // If this selector doesn't want to select anything, it will return selectable=false.
    fn requirements(&self) -> Option<Vec<Requirement>>;

    // Make a deep copy of the selector.
    fn deep_copy_selector(&self) -> Self;

    // RequiresExactMatch allows a caller to introspect whether a given selector
    // requires a single specific label to be set, and if so returns the value it
    // requires.
    fn requires_exact_match(&self, label: String) -> Option<String>;
}

/// Requirement contains values, a key, and an operator that relates the key and values.
/// The zero value of Requirement is invalid.
/// Requirement implements both set based match and exact match
/// Requirement should be initialized via NewRequirement constructor for creating a valid Requirement.
/// +k8s:deepcopy-gen=true
///
#[derive(Debug)]
pub struct Requirement {
    key: String,
    operator: selection::Operator,
    values: Vec<String>,
}

impl Requirement {
    /// NewRequirement is the constructor for a Requirement.
    /// If any of these rules is violated, an error is returned:
    /// (1) The operator can only be In, NotIn, Equals, DoubleEquals, Gt, Lt, NotEquals, Exists, or DoesNotExist.
    /// (2) If the operator is In or NotIn, the values set must be non-empty.
    /// (3) If the operator is Equals, DoubleEquals, or NotEquals, the values set must contain one value.
    /// (4) If the operator is Exists or DoesNotExist, the value set must be empty.
    /// (5) If the operator is Gt or Lt, the values set must contain only one value, which will be interpreted as an integer.
    /// (6) The key is invalid due to its length, or sequence
    ///
    /// of characters. See validateLabelKey for more details.
    ///
    /// The empty string is a valid value in the input values set.
    /// Returned error, if not nil, is guaranteed to be an aggregated field.ErrorList
    ///
    pub fn new(
        key: String,
        operator: selection::Operator,
        values: Vec<String>,
    ) -> Result<Self, InvalidRequirement> {
        use selection::Operator::*;
        match operator {
            In | NotIn if values.is_empty() => Err(InvalidRequirement),
            Equals | DoubleEquals | NotEquals if values.len() != 1 => Err(InvalidRequirement),
            Exists | DoesNotExist if !values.is_empty() => Err(InvalidRequirement),
            GreaterThan | LessThan if values.len() != 1 => Err(InvalidRequirement),

            _ => Ok(Self {
                key,
                operator,
                values,
            }),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn operator(&self) -> selection::Operator {
        self.operator
    }

    pub fn values(&self) -> BTreeSet<String> {
        self.values.iter().cloned().collect()
    }
}

#[derive(Debug)]
pub struct InvalidRequirement;
