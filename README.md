# Maze`.`com

Maze`.`com is an online multiplayer "board" game centered around a grid of tiles that make up a maze. It is the objective of the player to slide the tiles that make up the board to create paths they can navigate to and from their objective tiles and back to their home tile in order to score points and win the game.

## Pieces

The board consists of a grid of tile and a spare tile. On any of the tiles that do not live on a slidable row or column a players home piece may be placed, or the tile may be assigned as a player's goal. Homes are picked by the `Referee`. 

## Game Rules

Every player has the chance to make a move which consists of three parts. The first part is to choose the orientation that the spare tile will be inserted in, alongside the second which is to pick which of the slideable rows or colums it will be slid into. Once the slide is complete, whatever tile was slid off the board becomes the new spare tile. The third part of a move is to pick where the player avatar should move to from all the reachable tiles in its vicinity. A tile is reachable if the tile between the start and destination are connected. A player may also choose to skip their turn but they may not make a move that does not consist of all three parts mentioned before. If any of the three parts is invalid, the whole move is considered invalid.

## Project Structure
This project is laid out in 3 parts. Directories named after a capital letter include exploratory code to feel out the capabilities that Rust has for implmenting Maze`.`com. Directories that are numbered are integration tests for the Maze`.`com implementation that lives in the `Maze` directory.

Within the `Maze` Directory lives a few libraries and two binary crates. The main library is the `Common` library crate which contains most of the data types and functions used by all of the subsequent crates such as definitions for the `Board` and a `State` aka the state of the game.

The `Players` crate contains the definition for the API of player interactions so that any implementation of a player that follows this API should be able to function within the context of our implementation of the `Referee`. It exists as the `PlayerApi` trait. It also contains a reference implementation of a player and a few misbehaving players for the purpose of testing.

The `Referee` crate contains the definition for our implementation of the `Referee` the referee is designed to create or recieve a `State` and a list of players in order to play a game and ensure the rules are being followed.

The `Remote` crate is another library crate used to define how to adapt our implementation of the Maze`.`com to function across computer boundaries i.e. the network using the Remote Proxy adapter pattern. It defines a `RefereeProxy` that a remote player would use in order to recieve commands from the real `Referee` in the form of JSON, and a `PlayerProxy` that takes the referee's commands, converts them to JSON and sends it over TCP to a remote player. It also contains the JSON data definitions that are sent over TCP.

Finally the Server and Client binary crates are two executables that use the types in `Remote`. For the server, a `TcpListener` is created to accept connections to live within a `PlayerProxy` to then spin up a referee and play a game, where as the client creates a `RefereeProxy` which attempts to connect to the server and sign up for a game. If the client is successful it then waits for commands to be sent as JSON over TCP from the server in order to respond and play the game.

The documentation for each crate goes more in depth for the mechanisms of each implementation.