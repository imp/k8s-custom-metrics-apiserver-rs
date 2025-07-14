use super::*;

type K8sLabels = BTreeMap<String, String>;

impl Selector for metav1::LabelSelector {
    fn matches(&self, labels: &K8sLabels) -> bool {
        let match_labels = self
            .match_labels
            .as_ref()
            .map_or(true, |selector| match_labels(selector, labels));

        let match_expressions = self
            .match_expressions
            .as_ref()
            .map_or(true, |expressions| match_expressions(expressions, labels));

        match_labels && match_expressions
    }

    fn empty(&self) -> bool {
        let empty_expressions = self
            .match_expressions
            .as_ref()
            .map_or(true, |reqs| reqs.is_empty());

        let empty_labels = self
            .match_labels
            .as_ref()
            .map_or(true, |labels| labels.is_empty());

        empty_expressions && empty_labels
    }
}

fn match_labels(selector: &K8sLabels, labels: &K8sLabels) -> bool {
    selector
        .iter()
        .all(|(key, value)| labels.get(key) == Some(value))
}

fn match_expressions(expressions: &[metav1::LabelSelectorRequirement], labels: &K8sLabels) -> bool {
    expressions
        .iter()
        .all(|requirement| match_requirement(requirement, labels))
}

fn match_requirement(requirement: &metav1::LabelSelectorRequirement, labels: &K8sLabels) -> bool {
    let metav1::LabelSelectorRequirement {
        key,
        operator,
        values,
    } = requirement;
    let values = values
        .as_deref()
        .unwrap_or_default()
        .iter()
        .collect::<BTreeSet<&String>>();

    match operator.as_str() {
        "In" => r#in(key, &values, labels),
        "NotIn" => not_in(key, &values, labels),
        "Exists" => exists(key, labels),
        "DoesNotExist" => does_not_exist(key, labels),
        // We don't understand anything else, so failing it
        _ => false,
    }
}

fn r#in(key: &str, values: &BTreeSet<&String>, labels: &K8sLabels) -> bool {
    labels
        .get(key)
        .map_or(false, |value| values.contains(value))
}

fn not_in(key: &str, values: &BTreeSet<&String>, labels: &K8sLabels) -> bool {
    labels
        .get(key)
        .map_or(true, |value| !values.contains(value))
}

fn exists(key: &str, labels: &K8sLabels) -> bool {
    labels.contains_key(key)
}

fn does_not_exist(key: &str, labels: &K8sLabels) -> bool {
    !labels.contains_key(key)
}
