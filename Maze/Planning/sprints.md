TO: CEO Santa Claus  
FROM: Luis Soto and Sahana Tankala  
DATE: September 30th, 2022  
SUBJECT: Proposed Sprints  

# Sprint 1
## Deliverables
### Player-Referee Protocol (4 hours)
The start of the first sprint will be spent defining the `Player-Referee
protocol`. We need a spec that we can give to `Player`s that defines how they
should communicate with the `Referee` at each stage of the game. When
implementing the `Referee`, we need to follow the `Player-Referee protocol` so
it must be designed first.

### Labyrinth Model (8 hours)
We'll start by creating the data definitions needed for holding the state of a
`Labyrinth` game. This includes definitions for the game board, `Tile`s on the
board and the spare `Tile`, and `Piece`s (like a player's avatar, homebase, or goal).

### Observer / View (4 hours)
Once our `Labyrinth` model is defined, we can start implementing a way to
communicate the game state to observers. We aim to have a textual
representation of the game done by the end of the sprint, with a graphical
representation being a stretch goal.

# Sprint 2
## Deliverables
### Referee (16 hours)
The `Referee` is responsible for facilitating the game and communicating the
game state to the player. The `Referee` has 3 distinct jobs. It starts the
game, runs the game, and scores the `Player`s once the game is done. The bulk
of this sprint will be spent on running the game.

# Sprint 3
## Deliverables
### Player (12 hours)
Finally, we need our own `Player` implementations to use in testing and as
"house players". Some players will be naive and therefore simple to implement,
but we will want some sort of "optimal" player to pit against real `Players`.

### Game Server (4 hours)
Finally, we have to tie these components together using TCP. The game server
will connect to a `Player` over TCP and send those players to the `Referee`.
The game server then relays messages from the `Referee` to `Players` over TCP.
Once the `Referee` ends the game, the game server closes the TCP connections to
the `Player`, and listens for more `Player`s.
