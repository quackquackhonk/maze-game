TO: CEO Santa Claus  
FROM: Luis Soto and Sahana Tankala  
DATE: October 25th, 2022  
SUBJECT: Interactive Player  

# Board
```
  ↓ ↓ ↓ ↓
 ┌───────┐   ┌─┐
→│─│└┌┐┘┴│ ← │┼│
 │├┬┤┼─│└│   └─┘🗘
→│┌┐┘┴├┬┤│ ← 
 │┼─│└┌┐┘│
→│┴├┬┤┼─││ ← 
 │└┌┐┘┴├┬│
→│┤┼─│└┌┐│ ← 🗙
 └───────┘
  ↑ ↑ ↑ ↑
```

# Tile
  ```
┌───┐
│◆│⌂│
│─┼─│
│🯅│♢│
└───┘
```

## Tile Graphical Representation
* The upper left and lower right corners are reserved for displaying the two
	gems of the tiles Treasure.
* The lower left corner will be reserved for displaying any number of players occupying the tile.
* The upper right corner will be reserved for displaying the home of a player if it is on this tile.
* If this tile is the given players goal tile the tile should have a different color border.

## UI
* A button on either slide of each slide-able row or column to indicate an
	intention to slide that row/col
* A rotate button on the spare tile
* Each tile that's reachable after a slide should be highlighted and clickable
  * A click indicates the intention to move to that tile after the slide
  * This click confirms your move and input will be locked until it is the
	  interactive players turn again
* A cancel button to cancel a move before a destination tile is picked but
	after a slide row/col has been chosen
