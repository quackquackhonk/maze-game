# TODO

- [ ] Board should have methods for determining its width, height, and slidable
    columns and rows. 
  - [ ] NaiveStrategy should be generic using these methods  
Fixed by: https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/commit/461c6705df1fbeb32f41e685fc9195046a776404
Message: Makes both Referee and NaiveStrategy more generic over board size 

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
  Fixed by: [381d8d24bc95c82f1bfbb963ea2432eaa85e950a](https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/commit/381d8d24bc95c82f1bfbb963ea2432eaa85e950a)  
  Message: Closes #58


