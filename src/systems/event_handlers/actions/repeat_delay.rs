use super::system_prelude::*;
use climer::Timer;
use std::collections::HashMap;
use std::time::Duration;

struct TimedAction {
    timer:  Timer,
    action: ActionType,
}

#[derive(Default)]
pub struct HandleActionRepeatDelay {
    registered: HashMap<Entity, Vec<TimedAction>>,
}

impl<'a> System<'a> for HandleActionRepeatDelay {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, ActionTrigger<action::RepeatDelay>>,
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

                // REGISTER TIMERS AND ACTIONS
                for repeat_delay in action_trigger.drain_actions() {
                    for i in 0 .. repeat_delay.loops {
                        let delay_ms = i as u64 * repeat_delay.delay_ms;
                        let mut timer = Timer::new(
                            Some(Duration::from_millis(delay_ms).into()),
                            None,
                        );
                        timer.start().unwrap();
                        timed_actions.push(TimedAction {
                            timer:  timer,
                            action: *repeat_delay.action.clone(),
                        });
                    }
                }

                // UPDATE TIMERS
                let mut to_trigger = Vec::new();
                for (idx, timed_action) in timed_actions.iter_mut().enumerate()
                {
                    timed_action.timer.update().unwrap();
                    if timed_action.timer.state.is_finished() {
                        to_trigger.push(idx);
                    }
                }

                for idx in to_trigger.into_iter().rev() {
                    let timed_action = timed_actions.remove(idx);
                    action_type_trigger.add_action(timed_action.action.clone());
                }

                should_remove_entry = timed_actions.is_empty();
            }

            if should_remove_entry {
                let _ = self.registered.remove(&entity);
            }
        }
    }
}
