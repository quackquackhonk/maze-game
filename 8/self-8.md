**If you use GitHub permalinks, make sure your link points to the most recent commit before the milestone deadline.**

## Self-Evaluation Form for Milestone 8

Indicate below each bullet which file/unit takes care of each task.

For `Maze/Remote/player`,

- explain how it implements the exact same interface as `Maze/Player/player`

https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/f72afa558452e751f9d441fdc510e2e6ca7739a9/Maze/Remote/player.rs#L74-L118

We have an `impl` block that implements the `PlayerApi` trait for a `PlayerProxy`, meaning that the referee can use a `PlayerProxy` in the exact same way as a `LocalPlayer` or anything else that implements `PlayerApi`.

- explain how it receives the TCP connection that enables it to communicate with a client

https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/f72afa558452e751f9d441fdc510e2e6ca7739a9/Maze/Remote/player.rs#L19-L53

When creating a `PlayerProxy` we can either use the `new` associated function to create a `PlayerProxy` from arbitrary data types that implement `Read + Send` and `Write + Send`. The `Send` trait is marker trait for some of Rust's concurrency model, while `Read` and `Write` are the traits describing types that can be used to read data from and write data to, respectfully. For handling `TCP` connections, we use the `from_tcp` associated function. This will assign the `out` field to be the given `TcpStream`, and will assign the `r#in` field a `Deserializer` constructed from that same TCP stream. Logically, these fields represent the stream of data being sent `out` to the actual remote player, and the stream of JSON being read `r#in` from that player.

- point to unit tests that check whether it writes JSON to a mock output device

https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/f72afa558452e751f9d441fdc510e2e6ca7739a9/Maze/Remote/player.rs#L280-L316

All of our unit tests test JSON writing using a `Vec<u8>` (a vector of bytes) as a mock output stream. On line 282, the last argument to `PlayerProxy::new` is the initially empty output stream. In the case for testing the `win` method, we first call the method on the `PlayerProxy` we created. After that method gets called, we then assert that the output vector contains the correct `JsonFunctionCall` data (line 285).

For `Maze/Remote/referee`,

- explain how it implements the same interface as `Maze/Referee/referee`

It doesn't. The remote referee is only alike in name to the actual Referee, since the Remote Referee just relays messages to and from the player, it does not actually run a game.

- explain how it receives the TCP connection that enables it to communicate with a server

https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/f72afa558452e751f9d441fdc510e2e6ca7739a9/Maze/Remote/referee.rs#L20-L37

We handle this the same way as we do in `Remote/player.rs`. We have a `new` associated function for testing, and a `from_tcp` associated function for actual use. The only difference is that the remote Referee doesn't require the input and output streams to be `Send` as the remote referee doesn't use concurrency.

- point to unit tests that check whether it reads JSON from a mock input device

https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/f72afa558452e751f9d441fdc510e2e6ca7739a9/Maze/Remote/referee.rs#L93-L130

We use a slice of bytes as a mock input stream when unit testing the remote Referee. The mock input is constructed on lines 113-118, and then passed to `RefereeProxy::new`. We then call the `listen()` method to run the referee, compare the output in `RefereeProxy.out` to the output constructed on line 120.

For `Maze/Client/client`, explain what happens when the client is started _before_ the server is up and running:

- does it wait until the server is up (best solution)

We do not wait for the server to start.

- does it shut down gracefully (acceptable now, but switch to the first option for 9)

We do shutdown gracefully! If the client fails to connect to the server, then we simply exit the client program. We can easily switch this for milestone 9.

For `Maze/Server/server`, explain how the code implements the two waiting periods:

- is it baked in? (unacceptable after Milestone 7)
- parameterized by a constant (correct).

https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/f72afa558452e751f9d441fdc510e2e6ca7739a9/Maze/Server/server.rs#L19-L22

We first have an async function that will recieve continuously try and receive connections from Players.

https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/f72afa558452e751f9d441fdc510e2e6ca7739a9/Maze/Server/server.rs#L54-L57

Then the `main` function in the Server creates a `time_out` future using that `receive_connections` function. We do this twice since we have a single retry period when awaiting connections. The timeout used is defined as a constant:

https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/f72afa558452e751f9d441fdc510e2e6ca7739a9/Maze/Server/server.rs#L11

The ideal feedback for each of these three points is a GitHub
perma-link to the range of lines in a specific file or a collection of
files.

A lesser alternative is to specify paths to files and, if files are
longer than a laptop screen, positions within files are appropriate
responses.

You may wish to add a sentence that explains how you think the
specified code snippets answer the request.

If you did *not* realize these pieces of functionality, say so.

