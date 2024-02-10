use super::TogglingIsALifestyle;

pub struct BooleanToggler<'a> {
    bool_to_toggle: &'a mut bool,
}

impl TogglingIsALifestyle for BooleanToggler<'_> {
    #[inline]
    fn toggle(&mut self) {
        // i'm enterprise fast
        *self.bool_to_toggle ^= true;
    }
}

pub struct BooleanTogglerFactory;

impl BooleanTogglerFactory {
    #[inline]
    pub fn create_boolean_toggler(bool_to_toggle: &mut bool) -> BooleanToggler {
        return BooleanToggler {
            bool_to_toggle: bool_to_toggle,
        };
    }
}
