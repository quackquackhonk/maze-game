## Self-Evaluation Form for Milestone 3

Indicate below each bullet which file/unit takes care of each task:

1. rotate the spare tile by some number of degrees

Instead of rotating by a set number of degrees, we take in a number representing
the number of 90 degree turns to rotate the tile by:  
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/ba34699765c857059c8e1d05d1902ec0fec81b2e/Maze/Common/state.rs#L72-L78

2. shift a row/column and insert the spare tile

Shifting + inserting:  
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/ba34699765c857059c8e1d05d1902ec0fec81b2e/Maze/Common/state.rs#L143-L177

Unit tests for shifting + inserting:  
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/ba34699765c857059c8e1d05d1902ec0fec81b2e/Maze/Common/state.rs#L303-L313
   
3. move the avatar of the currently active player to a designated spot:  
Since `State` is a `pub` struct, the `State.player_info` field can be updated
directly to change the position of a player. This combined with the
`State::can_reach_position` means the `Referee` can check if the currently
active player can reach a position before actually updating that Player's
current position.  

Perma-link to the `State::can_reach_position` method:  
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/ba34699765c857059c8e1d05d1902ec0fec81b2e/Maze/Common/state.rs#L179-L186

4. check whether the active player's move has returned its avatar home
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/ba34699765c857059c8e1d05d1902ec0fec81b2e/Maze/Common/state.rs#L196-L201

5. kick out the currently active player
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/ba34699765c857059c8e1d05d1902ec0fec81b2e/Maze/Common/state.rs#L215-L220

The ideal feedback for each of these points is a GitHub
perma-link to the range of lines in a specific file or a collection of
files.

A lesser alternative is to specify paths to files and, if files are
longer than a laptop screen, positions within files are appropriate
responses.

You may wish to add a sentence that explains how you think the
specified code snippets answer the request.

If you did *not* realize these pieces of functionality, say so.

