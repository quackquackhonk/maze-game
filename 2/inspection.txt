Pair: tankalav-lsoto \
Commit: [06bbd9827d95619bee5d9daa7179464ef154209f](https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/tree/06bbd9827d95619bee5d9daa7179464ef154209f) \
Self-eval: https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/da221c51b5014762d5228a8df682928ca4a77203/2/self-2.md \
Score: 65/105 \
Grader: Mike Delmonaco \

Good job providing links to the correct commit on your self eval.

Good purpose statements and comments. Those diagrams on your tests are helpful. Your code is pretty clear.

Note: the state design portion was not graded. It should end up being subtracted from the total (would've been worth 25 points).

Deductions:

-5 Should have an interpretation for your position type. What is (0,0)? Bottom-left? Top-left?

-10 Should have a unit test for `test_reachable` with a non-empty result.

It seems that you ensure an even row/column index for sliding by taking in an "index" in the `Slide` and multiplying it by 2.
This should be more explicitly documented in the `Slide` interpretation.

(Ungraded) state design feedback:

* Player info should include home, color, goal tile, and current tile.
* Must store previous action so the next player cannot undo it
