Pair: tankalav-lsoto \
Commit: [8ac8529bcb4a04e60c017176b516831c53c14af3](https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/tree/8ac8529bcb4a04e60c017176b516831c53c14af3) \
Self-eval: https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/939fc1884d8a5c978c7568cfdedda84b5cd41885/5/self-5.md \
Score: 136/160 \
Grader: Megan Li

# Self-Eval

- [20/20] for a helpful and accurate self-eval 

# Programming

[86/110]

## Player

[50/50]

Names, signatures/types, purpose statements for:

- [10/10] `name`
- [10/10] `propose board`
- [10/10] `setting up`
- [10/10] `take a turn`
- [10/10] `did I win`

## Referee

[20/40]

The _testing referee_ must perform the following tasks in order and hence must have separate functions:

- [10/10] setting up the player with initial information
- [0/10] running rounds until the game is over
  - This should be factored out into a separate method. Then the top-level method in the referee would compose the multiple tasks a referee must perform.
  - This loop is hard to read and 80 lines long.
  - Why use a `loop` and `break`s when there are `while` loops?
- [0/10] running a round
  - This functionality is lumped into an 80 line long loop instead of getting factored out.
  - Your loop loops over _turns_ instead of _rounds_ which is incorrect. There must be a method to run a _round_ because one of the conditions for the game ending is "every player chooses not to move during one round". This is a different condition than `n` players consecutively passing.
- [10/10] determine the winners
  - WARNING: you should have a purpose statement for this method that describes how winners are determined.

Notes / feedback:
- Your purpose statement for your referee component is supposed to `explicitly state what kind of abnormal interactions that referee takes care of now and what kind are left to the project phase that adds in remote communication.`
- From the assignment spec: `For testing purposes, equip the referee with an interface that accepts a game state and runs the game to completion starting from this state.` Your referee implementation does not seem to have this.
- Every referee call on the player should be protected against illegal behavior and infinite loops/timeouts/exceptions. This should be factored out into a single point of control.

## Tests

[16/20]

- [10/10] a unit test for the referee function that returns a unique winner
- [6/10] a unit test for the scoring function that returns several winners
  - Does not exist but said so on self-eval

# Design

[30/30]

Nice diagram!

Describes gestures for:
- [10/10] rotates the tile before insertion
- [10/10] selects a row or column to be shifted and in which direction
- [10/10] selects the next place for the player's avatar