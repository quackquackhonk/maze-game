**If you use GitHub permalinks, make sure your link points to the most recent commit before the milestone deadline.**

## Self-Evaluation Form for Milestone 5

Indicate below each bullet which file/unit takes care of each task:

The player should support five pieces of functionality: 

- `name`
In the trait definition.  
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/5530a34836c1047302a4a98f2b1a49a5f79fa4e9/Maze/Players/player.rs#L8

In our `LocalPlayer` struct:
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/5530a34836c1047302a4a98f2b1a49a5f79fa4e9/Maze/Players/player.rs#L41-L43

- `propose board` (okay to be `void`)
In the trait definition.  
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/5530a34836c1047302a4a98f2b1a49a5f79fa4e9/Maze/Players/player.rs#L10

In our `LocalPlayer` struct:
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/5530a34836c1047302a4a98f2b1a49a5f79fa4e9/Maze/Players/player.rs#L45-L48

- `setting up`
In the trait definition.  
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/5530a34836c1047302a4a98f2b1a49a5f79fa4e9/Maze/Players/player.rs#L13

In our `LocalPlayer` struct:
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/5530a34836c1047302a4a98f2b1a49a5f79fa4e9/Maze/Players/player.rs#L50-L54

- `take a turn`
In the trait definition.  
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/5530a34836c1047302a4a98f2b1a49a5f79fa4e9/Maze/Players/player.rs#L15

In our `LocalPlayer` struct:
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/5530a34836c1047302a4a98f2b1a49a5f79fa4e9/Maze/Players/player.rs#L56-L64

- `did I win`
In the trait definition.  
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/5530a34836c1047302a4a98f2b1a49a5f79fa4e9/Maze/Players/player.rs#L17

In our `LocalPlayer` struct:
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/5530a34836c1047302a4a98f2b1a49a5f79fa4e9/Maze/Players/player.rs#L66-L67


Provide links. 

The referee functionality should compose at least four functions:

- setting up the player with initial information

We create the initial information here:
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/5530a34836c1047302a4a98f2b1a49a5f79fa4e9/Maze/Referee/referee.rs#L39-L59

And send it to the players here:
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/5530a34836c1047302a4a98f2b1a49a5f79fa4e9/Maze/Referee/referee.rs#L61-L70

- running rounds until termination

One iteration of this loop is a turn the current player needs to take:
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/5530a34836c1047302a4a98f2b1a49a5f79fa4e9/Maze/Referee/referee.rs#L184-L259
We also `break` at any point where:
1. A player has won.
2. The limit of rounds has passed.
3. There are no players left because they all cheated.
4. Every player passed their turn.

- running a single round (part of the preceding bullet)
The round is tracked by this variable:
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/5530a34836c1047302a4a98f2b1a49a5f79fa4e9/Maze/Referee/referee.rs#L181
which is updated whenever we reach the first player again:
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/5530a34836c1047302a4a98f2b1a49a5f79fa4e9/Maze/Referee/referee.rs#L250-L258
We also make sure to update who the first player is in case the first player gets kicked.
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/5530a34836c1047302a4a98f2b1a49a5f79fa4e9/Maze/Referee/referee.rs#L216-L225

- scoring the game

Calculating the winners of the game happens here:
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/5530a34836c1047302a4a98f2b1a49a5f79fa4e9/Maze/Referee/referee.rs#L83-L151

Point to two unit tests for the testing referee:

1. a unit test for the referee function that returns a unique winner
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/5530a34836c1047302a4a98f2b1a49a5f79fa4e9/Maze/Referee/referee.rs#L442-L456
2. a unit test for the scoring function that returns several winners
We do not have a test that tests returning several winners.

The ideal feedback for each of these points is a GitHub
perma-link to the range of lines in a specific file or a collection of
files -- in the last git-commit from Thursday evening. 

A lesser alternative is to specify paths to files and, if files are
longer than a laptop screen, positions within files are appropriate
responses.

You may wish to add a sentence that explains how you think the
specified code snippets answer the request.

If you did *not* realize these pieces of functionality, say so.

