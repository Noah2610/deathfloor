pub mod prelude {
    pub use super::add_entity_config::add_entity_config;
    pub use super::insert_components::insert_components;
}

mod add_entity_config;
mod insert_components;

use super::system_prelude;
