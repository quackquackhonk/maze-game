**If you use GitHub permalinks, make sure your links points to the most recent commit before the milestone deadline.**

## Self-Evaluation Form for Milestone 4

The milestone asks for a function that performs six identifiable
separate tasks. We are looking for four of them and will overlook that
some of you may have written deep loop nests (which are in all
likelihood difficult to understand for anyone who is to maintain this
code.)

Indicate below each bullet which file/unit takes care of each task:

Firstly, we have a `trait Strategy` (basically an interface) that
has the `make_move` method signature:

https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/67b9c1ad74920599bd674fde29c9f77170e3fcb5/Maze/Players/strategy.rs#L16-L25

We then implement `Strategy` for this `NaiveStrategy` enum,
which will handle the logic specific to the `Reimann` and `Euclid` 
strategies:

https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/67b9c1ad74920599bd674fde29c9f77170e3fcb5/Maze/Players/strategy.rs#L44-L53

1. the "top-level" function/method, which composes tasks 2 and 3

The `make_move` method for `NaiveStrategy` composes all our other methods.

https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/67b9c1ad74920599bd674fde29c9f77170e3fcb5/Maze/Players/strategy.rs#L182-L192

2. a method that `generates` the sequence of spots the player may wish to move to

`NaiveStrategy::get_alt_moves` returns a vector containing all destinations a
Player might want to go to, ordered depending on if the method was called on a
`Reimann` or a `Euclid`.

https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/67b9c1ad74920599bd674fde29c9f77170e3fcb5/Maze/Players/strategy.rs#L93-L116

3. a method that `searches` rows,  columns, etcetc. 

`find_move_to_reach` searches through all rows and columns for slides
and insertions that would let them move from `start` to `destination`.

https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/67b9c1ad74920599bd674fde29c9f77170e3fcb5/Maze/Players/strategy.rs#L140-L179

4. a method that ensure that the location of the avatar _after_ the
   insertion and rotation is a good one and makes the target reachable
  
  `reachable_after_move` checks if a Player can go from `start` to `destination`
   after making the given slide and insert move. This method considers
   rotations of the spare tile and wrapping around the board due to a slide.
   https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/67b9c1ad74920599bd674fde29c9f77170e3fcb5/Maze/Players/strategy.rs#L118-L138

ALSO point to

- the data def. for what the strategy returns
The `Strategy::make_move(...)` method returns a `PlayerAction`, 
which represents an action made by the player (making a move or passing).

https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/67b9c1ad74920599bd674fde29c9f77170e3fcb5/Maze/Players/strategy.rs#L27-L41

- unit tests for the strategy

Unit tests for all `NaiveStrategy` methods (including `make_move`) are unit tested in the `StrategyTests` module.

https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/67b9c1ad74920599bd674fde29c9f77170e3fcb5/Maze/Players/strategy.rs#L195

The ideal feedback for each of these points is a GitHub
perma-link to the range of lines in a specific file or a collection of
files.

A lesser alternative is to specify paths to files and, if files are
longer than a laptop screen, positions within files are appropriate
responses.

You may wish to add a sentence that explains how you think the
specified code snippets answer the request.

If you did *not* realize these pieces of functionality or realized
them differently, say so and explain yourself.


