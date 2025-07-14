use super::*;

mod impls;
mod selector;

type K8sLabels = BTreeMap<String, String>;

pub trait Selector<Labels = K8sLabels> {
    /// Matches returns true if this selector matches the given set of labels.
    ///
    fn matches(&self, labels: &Labels) -> bool;

    /// Empty returns true if this selector does not restrict the selection space.
    ///
    fn empty(&self) -> bool;
}
