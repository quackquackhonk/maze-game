Pair: tankalav-lsoto \
Commit: [ba34699765c857059c8e1d05d1902ec0fec81b2e](https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/tree/ba34699765c857059c8e1d05d1902ec0fec81b2e) \
Self-eval: https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/43fa1a2219b7ecbb5193b473d0861844aa7e1480/3/self-3.md \
Score: 73/85 \
Grader: Mike Delmonaco

## Programming (20 pts self-eval, 45 pts code):

Good job providing links to the right commit on your self eval.

-2 Should include a funciton/method to move the avatar of the currently active player to a designated spot. The referee shouldn't have to mutate fields of fields, it should
just have to call methods. Additionally, the state should be responsible for its own integrity. If its fields can be mutated from the outside, the state cannot mantain its own
integrity. Received partial credit for honesty.

Good job on the coding portion!

## Design (20 pts):

The interface should include methods that the referee calls on the player, not methods the player calls on some referee-like object. The referee will drive communication.

-5 No method for the referee to request a player's move

You should explicitly say that a player is initialized with a `Player` struct's information. I am charitably assuming this is what you meant.

-5 How will the player know their own position? The only information a player may receive after initialization based on the interface described is the `BoardState`, which only includes
Connectors, gems, and the spare tile. A player's avatar may be moved during other players' turns, which cannot be predicted or tracked by the player.
