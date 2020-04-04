use super::AnimationKey;

/// Actions for changing animation.
pub enum AnimationAction {
    /// Play the animation with the given `AnimationKey`.
    /// This will be played last on the animation stack,
    /// _pushed_ animations are played first.
    /// Animations to _play_ should always cycle endlessly
    /// (`AnimationTypeWrapper::Cycle`).
    /// If an animation push is attempted, but the last animation on the
    /// stack has the same key, then the new animation is _not_ pushed.
    Play(AnimationKey),
    /// Push the animation with the given `AnimationKey`
    /// on top off the animation stack.
    /// So this will be played next.
    /// These animations should _not_ cycle endlessly
    /// (`AnimationTypeWrapper::Once`).
    Push(AnimationKey),
    /// Pop off an animation from the top of the animation stack.
    /// Use with caution: when the final animation is popped off,
    /// unexpected behaviour might occur.
    Pop,
}
