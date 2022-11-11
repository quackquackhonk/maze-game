# TODO

- [ ] Board should have methods for determining its width, height, and slidable
    columns and rows. 
  - [ ] NaiveStrategy should be generic using these methods
 
- [ ] Remove parallel data structure in Referee

- [ ] Creating slides should be attached to an instance of board so that the
    board can validate it before creation.

- [ ] Overhaul error handling i.e. `thiserror` and `anyhow`

- [ ] Use aliri_braid for JSONColors

- [ ] Players homes must be distinct tiles and checked.

- [ ] Fix implementation of referee to iterate over rounds instead of turns.
  - [ ] abstract running a round into a method
  - [ ] fix losing by passing
  - [ ] add missing unit tests



# Completed
- [X] Move `reachable_after_move` to a better position in the codebase?   
- [X] `common::State` should be generic over the type of playerinfo  
- [x] Player trait should return results. 
  https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/2d7cba85a331b10cce4268ccbc41baa5c3f7d282/Maze/Players/player.rs#L19-L32
- [X] add a test case for when a `NaiveStrategy` is forced to pass
  https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/2dba8ec6e82bfd632018f0835971281cee90d663/Maze/Players/strategy.rs#L299-L351


