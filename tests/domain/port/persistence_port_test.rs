#![forbid(unsafe_code)]

use std::mem::size_of;
use std::sync::Arc;

use humanizar_units::domain::port::{MunicipioPort, UnitPort};

#[test]
fn asynchronous_ports_are_compatible_with_arc_trait_objects() {
    assert!(size_of::<Arc<dyn UnitPort>>() > 0);
    assert!(size_of::<Arc<dyn MunicipioPort>>() > 0);
}
