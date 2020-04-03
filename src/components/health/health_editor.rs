use super::component_prelude::*;
use super::health_action::HealthAction;

/// Can manipulate the entity's `Health` component via `HealthAction`s.
/// Through this component, the entity can _lose_ and _gain_ health.
#[derive(Component, Default)]
#[storage(DenseVecStorage)]
pub struct HealthEditor {
    actions: Vec<HealthAction>,
}

impl HealthEditor {
    pub fn gain(&mut self, hitpoints: HitPoints) {
        self.add_action(HealthAction::Gain(hitpoints))
    }

    pub fn lose(&mut self, hitpoints: HitPoints) {
        self.add_action(HealthAction::Lose(hitpoints))
    }

    pub fn add_action(&mut self, action: HealthAction) {
        self.actions.push(action);
    }

    pub fn drain_actions(&mut self) -> std::vec::Drain<HealthAction> {
        self.actions.drain(..)
    }
}
