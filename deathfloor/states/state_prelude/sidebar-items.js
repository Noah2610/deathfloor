initSidebarItems({"enum":[["StateEvent","The enum holding the different types of event that can be received in a `State` in the `handle_event` method."],["Trans","Types of state transitions. T is the type of shared data between states. E is the type of events"]],"struct":[["Camera","Camera struct."],["CustomGameData",""],["CustomGameDataBuilder","Builder struct for `CustomGameData`."],["Entity","`Entity` type, as seen by the user."],["StateData","State data encapsulates the data sent to all state functions from the application main loop."],["World","A [Resource] container, which provides methods to insert, access and manage the contained resources."]],"trait":[["Builder","A common trait for `EntityBuilder` and `LazyBuilder`, allowing either to be used. Entity is definitely alive, but the components may or may not exist before a call to `World::maintain`."],["State","A trait which defines game states that can be used by the state machine."],["WorldExt","This trait provides some extension methods to make working with shred's [World] easier."]]});