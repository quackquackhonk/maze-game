**If you use GitHub permalinks, make sure your link points to the most recent commit before the milestone deadline.**

## Self-Evaluation Form for Milestone 7

Indicate below each bullet which file/unit takes care of each task:

The require revision calls for

    - the relaxation of the constraints on the board size
    - a suitability check for the board size vs player number 

1. Which unit tests validate the implementation of the relaxation?

Our implementation has always supported NxM sized boards. There are unit tests in `Common/board.rs` that run on smaller boards:  
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/3f4e5cda8bc63c2302f44ebbe3c9bce644121cc4/Maze/Common/board.rs#L369-L407  

2. Which unit tests validate the suitability of the board/player combination? 

Boards now have a method to get an iterator over all possible home positions:  
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/3f4e5cda8bc63c2302f44ebbe3c9bce644121cc4/Maze/Common/board.rs#L53-L61

The Referee uses this method to assign goals to players in `make_initial_state()`:  
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/3f4e5cda8bc63c2302f44ebbe3c9bce644121cc4/Maze/Referee/referee.rs#L188-L221  

We did not implement a check (or have a unit test) that ensures that the number of players in the game is less than or equal to the number of possible homes for the board.
   
The ideal feedback for each of these three points is a GitHub
perma-link to the range of lines in a specific file or a collection of
files.

A lesser alternative is to specify paths to files and, if files are
longer than a laptop screen, positions within files are appropriate
responses.

You may wish to add a sentence that explains how you think the
specified code snippets answer the request.

If you did *not* realize these pieces of functionality, say so.

