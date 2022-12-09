# players

## Library Layout
### Player
Contains the trait [`player::PlayerApi`] and an implementation of it [`LocalPlayer`]. This
implementation relies on the strategies in [`strategy`] to decide its turns.

### Strategy
Within this module are the data definitions for the decisions a player can make in terms of
making a move.

Also contains the trait [`strategy::Strategy`] which describes a type that can provide a
decision on a move given a state.

Also contains an implementation that relies on enumerating alternate goals if the player's is
unreachable and trying to reach those in an order defined in its implementation.

### Bad Player
Contains a few implementations of [`player::PlayerApi`] that intentionaly misbehave for testing
`Referee`s.

#### Json
Contains data definitions for Json for integration tests but also most importantly
[`json::JsonChoice`] which is the data definition for a move sent over the network
