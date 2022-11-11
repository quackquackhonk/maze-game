# TODO

- [ ] Use aliri_braid for JSONColors
    - we decided that this actually wasn't worth doing, and we prefer our current solution :)

# Completed
- [X] Move `reachable_after_move` to a better position in the codebase  
  Fixed by: https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/commit/cd8f8fad4deb33e8c8b8c071ef5bd6da4211d496  
  Message:  Closes #37 by moving reachable_after_move into State  

- [X] `common::State` should be generic over the type of playerinfo  
  Fixed By: https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/commit/d454aabe8196da993d927fb9dc5e3aef7c589a44
  Which is the merge commit for #61

- [X] Player trait should return results. 
  https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/2d7cba85a331b10cce4268ccbc41baa5c3f7d282/Maze/Players/player.rs#L19-L32

- [X] add a test case for when a `NaiveStrategy` is forced to pass  
  Fixed by: [381d8d24bc95c82f1bfbb963ea2432eaa85e950a](https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/commit/381d8d24bc95c82f1bfbb963ea2432eaa85e950a)  

  Message: Closes #58  
- [X] Board should have methods for determining its width, height, and slidable
    columns and rows. 
  - [X] NaiveStrategy should be generic using these methods  
Fixed by: https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/commit/461c6705df1fbeb32f41e685fc9195046a776404
Message: Makes both Referee and NaiveStrategy more generic over board size  

- [X] Creating slides should be attached to an instance of board so that the
    board can validate it before creation.  
  Fixed By: https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/commit/7c2917c5ffb6ad54181758593f9460a330caff90
  Message: Makes both Referee and NaiveStrategy more generic over board size  

- [X] Fix implementation of referee to iterate over rounds instead of turns.
  - [X] abstract running a round into a method
  - [X] fix losing by passing
    These first two tasks are fixed by:
    https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/commit/c133fb57cc251904b78ecb1f0e8c74a4b6bf0aed  
    Message: Refactors `Referee` to be cleaner ?
  - [X] add missing unit tests
    Fixed by: https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/commit/e5347114a054243371031eb4d3d346e96f7c0394
    Message: Adds test for `run_from_state` with multiple winners

- [X] Players homes must be distinct tiles and checked.
  Fixed by: https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/commit/461c6705df1fbeb32f41e685fc9195046a776404
  Message: Makes both `Referee` and `NaiveStrategy` more generic over board size
  This commit also made the `Referee` assign unique home tiles to players

  The observer rendering is also changed by: https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/commit/ca737bfab1b5db7197bd225f1ddf737ac7983e78  
  Message: Changes observer to respect the fact that homes are now on unique tiles

- [X] Allow for non-7x7 boards.
  This was already handled in previous milestones no changes were made :)

- [X] Remove parallel data structure in Referee
  Fixed by: https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/commit/c133fb57cc251904b78ecb1f0e8c74a4b6bf0aed  
  Message: Refactors `Referee` to be cleaner ?  
  - This commit used the generic `State` changes from an earlier TODO to remove the parallel data structure we had in `Referee`
  
- [X] Overhaul error handling i.e. `thiserror` and `anyhow`  
  This is distributed over a lot of commits. The main ones are:  
  1. https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/commit/2d7cba85a331b10cce4268ccbc41baa5c3f7d282  
     Which changes the `PlayerApi` trait to always return a `Result` type indicating an error.  
  2. https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/commit/7c2917c5ffb6ad54181758593f9460a330caff90#diff-f6b918a49f593a5e0edf5b8699ca96c6744a183767f7eef1cbb8f98db544273a  
     This changes many of our `Common` components to use `anyhow` and `thiserror` for error handling, mainly `Common/json.rs`.  
