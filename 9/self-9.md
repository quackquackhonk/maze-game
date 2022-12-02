**If you use GitHub permalinks, make sure your link points to the most recent commit before the milestone deadline.**

## Self-Evaluation Form for Milestone 9

Indicate below each bullet which file/unit takes care of each task.

Getting the new scoring function right is a nicely isolated design
task, ideally suited for an inspection that tells us whether you
(re)learned the basic lessons from Fundamentals I, II, and III. 

This piece of functionality must perform the following four tasks:

- find the player(s) that has(have) visited the highest number of goals
- compute their distances to the home tile
- pick those with the shortest distance as winners
- subtract the winners from the still-active players to determine the losers

The scoring function per se should compose these functions,
with the exception of the last, which can be accomplised with built-ins. 

1. Point to the scoring method plus the three key auxiliaries in your code. 

We did not break up the scoring tasks into separate functions. We have a single scoring function `calculate_winners`:
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/7c9c9be16ba3d9214480d714bc1b56319fd743b1/Maze/Referee/referee.rs#L334-L405

3. Point to the unit tests of these four functions.

Unit tests for `calculate_winners`:

https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/7c9c9be16ba3d9214480d714bc1b56319fd743b1/Maze/Referee/referee.rs#L645-L733

The ideal feedback for each of these three points is a GitHub
perma-link to the range of lines in a specific file or a collection of
files.

A lesser alternative is to specify paths to files and, if files are
longer than a laptop screen, positions within files are appropriate
responses.

You may wish to add a sentence that explains how you think the
specified code snippets answer the request.

If you did *not* realize these pieces of functionality, say so.

