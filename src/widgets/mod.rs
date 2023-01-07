use adw::prelude::*;

use self::{upgrader::Upgrader, welcome::Welcome};

pub mod upgrader;
pub mod welcome;

/// Ensure all widget types are loaded
pub fn init_widgets() {
    Welcome::ensure_type();
    Upgrader::ensure_type();
}
