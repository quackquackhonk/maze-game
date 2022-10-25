Pair: tankalav-lsoto \
Commit: [67b9c1ad74920599bd674fde29c9f77170e3fcb5](https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/tree/67b9c1ad74920599bd674fde29c9f77170e3fcb5) \
Self-eval: https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/8a9582f3aca529c63ef105d7e9392807a257747a/4/self-4.md \
Score: 100/110 \
Grader: Mike Delmonaco

## Programming (100 pts code, 10 pts self-eval)

For task 1, you should've linked to `find_move_to_reach_alt_goal`. Although `get_move` is your top-level method, it does not compose tasks 2 and 3.
The method that composes tasks 2 and 3 is `find_move_to_reach_alt_goal`, not `get_move`. Since you call it in your top-level method and your organization is clear,
I'll let it slide.

You should also check that the move does not undo the previous slide.

Good data definition!

-10 Should test a situation where the strategy is forced to pass.
I see that you tested this for `find_move_to_reach`, but you should've also tested the case of the whole strategy
not being able to find a single alternative goal that is reachable, not just a single goal search failure.

Good job on code overall!

## Design (Ungraded, just feedback)

Great job on the protocol! Good sequence diagram.

A few notes:

- You don't need to go into so much detail about the internals of the referee and the player.
- You should explicitly specify the methods that are being called.

The key bits are the sequence of method calls and how many times each method can be called, which you specified.
