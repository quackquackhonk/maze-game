TO: CEO Santa Claus
FROM: Luis Soto and Sahana Tankala
DATE: October 7th, 2022
SUBJECT: Referee Game State

# Data
We believe the Referee state should store:
 - The board (which stores its spare tile)
 - The list of active players
    - And their positions
 - The location of all players home and goals

# Functionality
We believe the Referee should be able to
 - Initialize the board and its state alongside its own state.
 - Rotate its spare tile by 90 degrees at a time
 - Validate an entire player move, from slide to insert to movement
   - Ensure the proposed row to slide is a slide-able row
   - Check if the given position is reachable from their position
 - Perform valid moves 
 - Removes an active player for misbehaving
 - Check for player wins
 - Handle every player passing as a loss for every player
