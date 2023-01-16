use super::*;

/// NamespacedName comprises a resource name, with a mandatory namespace,
/// rendered as "<namespace>/<name>".  Being a type captures intent and
/// helps make sure that UIDs, namespaced names and non-namespaced names
/// do not get conflated in code.  For most use cases, namespace and name
/// will already have been format validated at the API entry point, so we
/// don't do that here.  Where that's not the case (e.g. in testing),
/// consider using NamespacedNameOrDie() in testing.go in this package.
///
#[derive(Debug)]
pub struct NamespacedName {
    pub namespace: String,
    pub name: String,
}

impl fmt::Display for NamespacedName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.namespace, self.name)
    }
}
