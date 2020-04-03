use super::system_prelude::*;
use climer::Timer;
use std::collections::HashMap;
use std::time::Duration;

struct TimedAction {
    timer:  Timer,
    action: ActionType,
}

#[derive(Default)]
pub struct HandleActionDelay {
    registered: HashMap<Entity, Vec<TimedAction>>,
}

impl<'a> System<'a> for HandleActionDelay {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, ActionTrigger<action::Delay>>,
        WriteStorage<'a, ActionTypeTrigger>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut action_trigger_store,
            mut action_type_trigger_store,
        ): Self::SystemData,
    ) {
        for (entity, action_trigger, action_type_trigger) in (
            &entities,
            &mut action_trigger_store,
            &mut action_type_trigger_store,
        )
            .join()
        {
            let should_remove_entry;

            {
                let timed_actions = self.registered.entry(entity).or_default();

                // Register timer.
                for action in action_trigger.drain() {
                    let mut timer = Timer::new(
                        Some(Duration::from_millis(action.delay_ms).into()),
                        None,
                    );
                    timer.start().unwrap();
                    timed_actions.push(TimedAction {
                        timer:  timer,
                        action: *action.action,
                    });
                }

                // Update timers.
                let mut to_trigger = Vec::new();
                for (idx, timed_action) in timed_actions.iter_mut().enumerate()
                {
                    timed_action.timer.update().unwrap();
                    if timed_action.timer.state.is_finished() {
                        to_trigger.push(idx);
                    }
                }

                // Trigger delayed actions.
                for to_trigger_idx in to_trigger.into_iter().rev() {
                    let timed_action = timed_actions.remove(to_trigger_idx);
                    action_type_trigger.trigger(timed_action.action);
                }

                should_remove_entry = timed_actions.is_empty();
            }

            if should_remove_entry {
                let _ = self.registered.remove(&entity);
            }
        }
    }
}
