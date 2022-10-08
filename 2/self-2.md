## Self-Evaluation Form for Milestone 2

Indicate below each bullet which file/unit takes care of each task:

1. point to the functinality for determining reachable tiles 

   - a data representation for "reachable tiles"
   We use a `Vec<Position>` to store the reachable tiles from a given `Position`. `BoardResult<T>` is a wrapper for the Rust `Result` type, which we are wrapping the `Vec<Position>` in:  
   https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/06bbd9827d95619bee5d9daa7179464ef154209f/Maze/Common/board.rs#L117
   Type alias for `Position`:
   https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/06bbd9827d95619bee5d9daa7179464ef154209f/Maze/Common/grid.rs#L3-L4
   
   - its signature and purpose statement
   https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/06bbd9827d95619bee5d9daa7179464ef154209f/Maze/Common/board.rs#L112-L117
   - its "cycle detection" coding (accumulator)
   We check for cycles by maintaining a `HashSet` of visited `Positions` in the body of the `reachable` method:  
   https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/06bbd9827d95619bee5d9daa7179464ef154209f/Maze/Common/board.rs#L127
   We only add neighbors of a `Position` to the worklist if they are not in the `visited` set:  
   https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/06bbd9827d95619bee5d9daa7179464ef154209f/Maze/Common/board.rs#L131-L135
   
   - its unit test(s)
   https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/06bbd9827d95619bee5d9daa7179464ef154209f/Maze/Common/board.rs#L384-L399

2. point to the functinality for shifting a row or column 

   - its signature and purpose statement  
   This method delegates the actual "sliding" of the 2D array stored in the `grid` field to methods on our `Grid` struct.  
   https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/06bbd9827d95619bee5d9daa7179464ef154209f/Maze/Common/board.rs#L21-L24
   These are the rotating methods that actually rotate the grid:  
   https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/06bbd9827d95619bee5d9daa7179464ef154209f/Maze/Common/grid.rs#L9-L41
   
   - unit tests for rows and columns
   Unit tests for `Board::slide()`:  
   https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/06bbd9827d95619bee5d9daa7179464ef154209f/Maze/Common/board.rs#L225-L278
   Unit tests for `Grid::rotate_*()`:  
   https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/06bbd9827d95619bee5d9daa7179464ef154209f/Maze/Common/grid.rs#L153-L247

3. point to the functinality for inserting a tile into the open space

   - its signature and purpose statement  
   https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/06bbd9827d95619bee5d9daa7179464ef154209f/Maze/Common/board.rs#L62-L66
   - unit tests for rows and columns  
   https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/06bbd9827d95619bee5d9daa7179464ef154209f/Maze/Common/board.rs#L279-L367

If you combined pieces of functionality or separated them, explain.

If you think the name of a method/function is _totally obvious_,
there is no need for a purpose statement. 

The ideal feedback for each of these points is a GitHub
perma-link to the range of lines in a specific file or a collection of
files.

A lesser alternative is to specify paths to files and, if files are
longer than a laptop screen, positions within files are appropriate
responses.

You may wish to add a sentence that explains how you think the
specified code snippets answer the request.

If you did *not* realize these pieces of functionality, say so.

