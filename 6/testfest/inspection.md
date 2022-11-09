Pair: tankalav-lsoto \
Commit: [15abba8691ba9ea121265b7e50a32c460be802f6](https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/tree/15abba8691ba9ea121265b7e50a32c460be802f6) \
Self-eval: https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/15abba8691ba9ea121265b7e50a32c460be802f6/6/self-6.md \
Score: 165/165 \
Grader: Rajat Keshri

---
OBSERVER RUN: 60/60 pts 
 - [10/10pt] for getting the program to run; if that fails, subtract all points 
 - [10/10pt] for easily understanding the displayed state (player location, player home, spare)
 - [10/10pt] for clicking through the entire game via `next`
 - [10/10pt] for saving the first game state in a file of your choice (a new one too) 
 - [10/10pt] for canceling the save functionality w/o crashing the program
 - [10/10pt] for confirming that the saved file is the same as `3-in.json`

---
PROGRAMMING `observer.PP`: 60/60 pts
The `observer` function/class must have two ways of receiving data from the `referee`.
  - Method that sends the given input to the observer thread
  - Method which signals that no more `states` will be sent
- [20/20pt] for an `Observer` interface that immediately clarifies how the two pieces of functionality are implemented (10 each)

The points are distributed across the two pieces of code as follows:
The referee function must contain two calls that send across states and one call that signals that the game is over. The two state-sending calls must ensure that
  1. the initial state is sent
  2. the state _after_ each turn is sent
- [20/20pt] for the three essential calls in `Referee`.
- [20/20pt] for controlling at a single point whether the observer is even informed

---
DESIGN `interactive-player.md`: 45/45 pts
- [15/15pt] each call from the referee to a player must now go across the wire (remote proxy)
- [15/15pt] each call's arguments and results must have a JSON representation (use one of ours, or their own)
- [15/15pt] the design must specify how the client-players sign up with our server. 

\<free text\>
