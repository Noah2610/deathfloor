pub use bundle::TriggerEnemyEventsBundle;

mod on_spawn;

mod bundle {
    use super::*;
    use amethyst::core::bundle::SystemBundle;
    use amethyst::ecs::{DispatcherBuilder, World};
    use deathframe::core::amethyst;

    pub struct TriggerEnemyEventsBundle<'a> {
        deps: &'a [&'a str],
    }

    impl<'a> TriggerEnemyEventsBundle<'a> {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_deps(mut self, deps: &'a [&'a str]) -> Self {
            self.deps = deps;
            self
        }
    }

    impl<'a, 'b, 'c> SystemBundle<'a, 'b> for TriggerEnemyEventsBundle<'c> {
        fn build(
            self,
            _world: &mut World,
            builder: &mut DispatcherBuilder<'a, 'b>,
        ) -> Result<(), amethyst::Error> {
            builder.add(
                super::on_spawn::TriggerEnemyEventOnSpawn::default(),
                "trigger_enemy_event_on_spawn",
                self.deps,
            );
            Ok(())
        }
    }

    impl<'a> Default for TriggerEnemyEventsBundle<'a> {
        fn default() -> Self {
            Self {
                deps: Default::default(),
            }
        }
    }
}
