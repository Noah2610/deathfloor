use crate::animation_key::AnimationKey;
use deathframe::animation::data::prelude::*;
use std::collections::HashMap;

pub mod prelude {
    pub use super::AnimationConfig;
    pub use super::AnimationsConfig;
}

pub type AnimationsConfig =
    HashMap<AnimationKey, AnimationConfig<(usize, u64)>>;

#[derive(Clone, Deserialize)]
pub enum AnimationConfig<F>
where
    F: Into<AnimationFrame> + Send + Sync,
{
    Once(Vec<F>),
    Cycle(Vec<F>),
    Rev(Vec<F>),
}

impl<F> IntoIterator for AnimationConfig<F>
where
    F: Into<AnimationFrame> + Send + Sync,
{
    type Item = AnimationFrame;
    type IntoIter = Box<dyn AnimationFramesIter>;

    fn into_iter(self) -> Self::IntoIter {
        // fn frames_to_iter<F, T>(frames: Vec<F>) -> T
        // where
        //     F: Into<AnimationFrame>,
        //     T: Iterator<Item = AnimationFrame>,
        // {
        //     frames
        //         .into_iter()
        //         .map(|f| {
        //             let f: AnimationFrame = f.into();
        //             f
        //         })
        //         .into()
        // }

        match self {
            AnimationConfig::Once(frames) => Box::new(
                frames
                    .into_iter()
                    .map(|into_frame| into_frame.into())
                    .collect::<Vec<AnimationFrame>>()
                    .into_iter(),
            ),
            AnimationConfig::Cycle(frames) => Box::new(
                frames
                    .into_iter()
                    .map(|into_frame| into_frame.into())
                    .collect::<Vec<AnimationFrame>>()
                    .into_iter()
                    .cycle(),
            ),
            AnimationConfig::Rev(frames) => Box::new(
                frames
                    .into_iter()
                    .map(|into_frame| into_frame.into())
                    .collect::<Vec<AnimationFrame>>()
                    .into_iter()
                    .rev(),
            ),
        }
    }
}
