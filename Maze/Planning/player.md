TO: CEO Santa Claus  
FROM: Luis Soto and Sahana Tankala  
DATE: October 11th, 2022  
SUBJECT: Player  

# Data Representation

We separated the data definition of the Player type into 2 categories: data
that is the internal state of the player and data that the referee informs the
player of whenever it is the current players turn.

[These](#stores) are the pieces of data a Player can keep track off, either
because they are static, i.e. the home and goal tiles never change, or can be
trivially updated i.e. the players own position is updated by the referee but
can be updated internally after the player makes a move.

[These](#is-informed-of) are the pieces of information the player ***must*** be
informed of. This is because the player is not aware of what moves other
players attempt and so cannot update its internal state based on the external
state in the referee.
```rust
struct Player {
	home: Position,
	goal: Position,
	last_known_pos: Position,
}
```

## Stores
- Home Tile
- Goal Gem
- Avatar Position

## Is informed of
- Board State
  - Connectors/Gem
  - Spare Tile 
- Other Player Info
  - Positions
  - Home Tile
- Current position

# Player Interface

The player trait masks all the `tcp`/`json` implementation but represents a step in the
chain of making a move. 

First, a player requests the board state i.e. `request_board_state()`. At this point it waits for the referee to
respond, which it will when it is this players turn.

Then, informed of the board state, the player must choose to pass or make a move.

 - In the first case it sends a pass command i.e. `send_pass_command()`.

 - In the second case it starts by sending a slide command i.e.
   `send_slide_command()` it will also receive

   Then it sends a rotate and insert command to inform the referee of many
   degrees to rotate the spare tile, before it inserts it on the empty spot on
   the board.

   Then it sends a move command to inform the referee the intended destination
   of the players avatar.

```rust
// in common::interface
/// Valid rotations for a spare_tile
enum Rotation_Degrees {
	d0,
	d90,
	d180,
	d270,
}

// The player implementation
trait Player_Interface {

	/// BoardState is the representation of the Board the player uses
	fn request_board_state() -> BoardState;

	/// communicates to the Referee that it will not be taking a move
	fn send_pass_command();
	
	/// slide comes from our Common Library
	/// TileState is a representation of a tile that the player uses
	fn send_slide_command(slide: common::board::Slide) -> TileState;

	/// rot comes from out Common Library
	fn send_rotate_spare_tile_and_insert_command(rot: common::interface::Rotation_Degrees);

	/// pos comes from our Common Library
	fn send_move_command(pos: common::grid::Position);

}
```
