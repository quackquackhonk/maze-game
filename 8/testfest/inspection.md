Pair: tankalav-lsoto \
Commit: [f72afa558452e751f9d441fdc510e2e6ca7739a9](https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/tree/f72afa558452e751f9d441fdc510e2e6ca7739a9) \
Self-eval: https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/53a077e971b825a6a1b4b5af4befccda510dd2a6/8/self-8.md \
Score: 110/110
Grader: Somtoo Chukwurah

#### TEST FEST

No deductions; `xbad2` does not directly use the input spec to form the cheaters output.

#### PROGRAM INSPECTION 

- [110/110]

- [20/20] for an accurate and helpful self evaluation. 

- `Maze/Remote/player`
  - [20/20] `Maze/Remote/player` must implement the same interface as `Maze/Player/player`, that is:
    - [5/5] accepting `setup` calls, turn them into JSON, get result in JSON, return when done
    - [5/5] accepting `take-turn` calls, turn them into JSON, receive CHOICE in JSON, return as value
    - [5/5] accepting `win` calls, turn them into JSON, get result in JSON, return when done
    - [5/5] These methods must not do more (or less) than exactly that.
  - [10/10] Constructor must receive handles for sending/receiving over TCP. 
  - [10/10] Unit tests for `Maze/Remote/player`

- `Maze/Remote/referee`
  - [10/10] `Maze/Remote/referee` must implement the same "context" as `Maze/Player/referee`.
    - All or nothing:
    - making `setup` calls
    - making `take-turn` calls
    - making `win` calls
  - [10/10] Receive handles for sending/receiving over TCP.
  - [10/10] Unit tests for `Maze/Remote/referee`
    Unit tests for the proxy player may just check whether the method can read JSON arguments from a mock input device. 
   
  
Client-Server

- [10/10] If someone starts the client before the server is up, the client must wait or shut down gracefully.
  - You say "the code expects the creator of the client to catch or suppress this error" but what if the creator of the client doesn't do it 
  - Since you technically do not have a graceful shutdown implemented but mentioned this on your self-eval with "The Client component will raise an OSError when socket.create_connection fails" I will give 60% credit
- [10/10] The two waiting periods in the server are not hardwired (copy pasted codeblocks).

