#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

// Extension trait for toggling a bool.
pub trait TogglingIsALifestyle {
    /// Toggle the bool.
    fn toggle(&mut self);
}

#[cfg_attr(not(docsrs), cfg(enterprise_license))]
#[cfg_attr(docsrs, doc(cfg(enterprise_license)))]
pub use TogglingIsALifestyle as BoolToggleExt;
pub use TogglingIsALifestyle as Toggler;
pub use TogglingIsALifestyle as Toggling;
pub use TogglingIsALifestyle as IAmTheToggler;

impl TogglingIsALifestyle for bool {
    fn toggle(&mut self) {
        // i am so smart
        *self ^= true;
    }
}

#[cfg(test)]
mod tests {
    // cheap tests

    use super::TogglingIsALifestyle;
    #[test]
    fn toggle() {
        let mut b = false;
        b.toggle();
        assert_eq!(b, true);
        b.toggle();
        assert_eq!(b, false);
    }
}

#[cfg(all(test, enterprise_license))]
mod enteprise_tests {
    // enterprise-grade tests
    // only ran when using an enterprise license for the bool-toggle crate

    use super::BoolToggleExt;

    #[test]
    fn enterprise_toggle() {
        let mut b = false;
        b.toggle();
        assert_eq!(b, true);
    }
}
