pub use bundle::EventHandlersBundle;

mod on_spawn;

mod bundle {
    use amethyst::core::bundle::SystemBundle;
    use amethyst::ecs::{DispatcherBuilder, World};
    use deathframe::core::amethyst;

    pub struct EventHandlersBundle<'a> {
        deps: &'a [&'a str],
    }

    impl<'a> EventHandlersBundle<'a> {
        pub fn with_deps(mut self, deps: &'a [&'a str]) -> Self {
            self.deps = deps;
            self
        }
    }

    impl<'a, 'b, 'c> SystemBundle<'a, 'b> for EventHandlersBundle<'c> {
        fn build(
            self,
            _world: &mut World,
            builder: &mut DispatcherBuilder<'a, 'b>,
        ) -> Result<(), amethyst::Error> {
            builder.add(
                super::on_spawn::HandleEventOnSpawn::default(),
                "handle_event_on_spawn_system",
                self.deps,
            );
            Ok(())
        }
    }

    impl<'a> Default for EventHandlersBundle<'a> {
        fn default() -> Self {
            Self {
                deps: Default::default(),
            }
        }
    }
}
