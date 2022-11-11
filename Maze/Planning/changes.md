TO: CEO Santa Claus  
FROM: Luis Soto and Sahana Tankala  
DATE: November 10th, 2022
SUBJECT: Possible Changes

# Possible Changes to The Game

## Blank Tiles for Board
Implementing blank tiles for the board should be an easy change. We can add a
`Blank` variant to our `ConnectorShape` enum, and then update all `match`
statements on `ConnectorShape`s to have a path for the `Blank` variant. 

Difficulty: 1

## Movable Tiles for Goals
This is not too difficult of a change. First we'll have to make sure the
Referee is able to assign goals to players that are on moveable tiles. Then we
have to ensure that our `State` also updates goal tiles whenever a slide is
made on the board.

This task gets a little more difficult depending on where the goal tile is slid off of the board. 
- If the goal behaves the same as `Player`s do when they are slid off the
  board, the task doesn't get more complex.
- If a Player's goal tile is allowed to remain on the spare tile after being
  slid off, the task becomes more complex. Our data representation for a goal
  position needs to be updated to allow for some `OffTheBoard` position.

Difficulty: 2-3 depending on interpretation.

## Sequential Player Goals
This should be a fairly straightforward change. Instead of keeping track of
single goals, a `FullPlayerInfo` would contain a `Vec<Position>`. When a player
reaches their current goal (the first `Position` in the vector), the `Referee`
pops the first element from the vector, and calls `setup` again on that player.
Calculating the winner must change slightly to break ties by "fewest goals
remaining" and then by euclidian distance to home / goal tiles. 

Difficulty: 2
