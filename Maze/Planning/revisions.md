TO: CEO Santa Claus  
FROM: Sahana Tankala and Luis Soto
DATE: December 1st, 2022
SUBJECT: Revisions to Labyrinth

## State Changes
1. The `PrivatePlayerInfo` trait defined in `Common/state.rs` was changed to allow for tracking and incrementing the number of goals a Player has reached.
2. The `FullPlayerInfo` structure was changed to have an additional `goals_reached` field so that it can implement the updated `PrivatePlayerInfo` trait.

## Referee Changes

1. The `Referee` has a `Config` struct that indicates whether or not to run a game with the multiple goals revisions.
2. We added the `get_initial_goals` method to the `Referee` to generate the list
  of alternate goals at the start of the game.
3. The `run_round` method takes in a list of remaining goals. When a `Player`
  reaches its goal, we assign them a new goal from this list. If the list is
  empty, the `Player` is assigned its home as its next goal. The
  `run_from_state` method initializes this list using `get_initial_goals`.
4. Because of the changes made to the state, the Referee no longer needed to maintain a `HashSet<Color>` of Players that have reached a goal. Instead, we can use the `goals_reached` of each player to track the same information.
5. The `calculate_winners` method was changed to only use the scoring from the revision spec. Our previous scoring algorithm is equivalent when the game is run without multiple goals, so the implementation was updated to the more general algorithm.
