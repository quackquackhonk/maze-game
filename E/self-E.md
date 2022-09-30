## Self-Evaluation Form for TAHBPL/E

A fundamental guideline of Fundamentals I and II is "one task, one
function" or, more generally, separate distinct tasks into distinct
program units. Even exploratory code benefits from this much proper
program design. 

This assignment comes with three distinct, unrelated tasks.

So, indicate below each bullet which file/unit takes care of each task:


1. dealing with the command-line argument (PORT)
2. 
Definition of the port command-line argument, and validation for the port:
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/9d32fa2ae0122f59db94763c413962e6dbc171b0/E/Other/src/main.rs#L10-L24

Function call in `main` to get and validate the port the user entered
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/9d32fa2ae0122f59db94763c413962e6dbc171b0/E/Other/src/main.rs#L27


2. connecting the client on the specified port to the functionality

https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/9d32fa2ae0122f59db94763c413962e6dbc171b0/E/Other/src/main.rs#L29-L31


3. core functionality (either copied or imported from `C`)

We import our `Corner` data definition from `xjson`
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/9d32fa2ae0122f59db94763c413962e6dbc171b0/E/Other/src/main.rs#L8

Our `read_from_client` handles reading from the client until the connection
is closed, `write_to_client` sends our response back to the client.
`read_from_client` is mostly copied from the `main` function from `C`.
https://github.khoury.northeastern.edu/CS4500-F22/tankalav-lsoto/blob/9d32fa2ae0122f59db94763c413962e6dbc171b0/E/Other/src/main.rs#L40-L52

The ideal feedback for each of these three points is a GitHub
perma-link to the range of lines in a specific file or a collection of
files.

A lesser alternative is to specify paths to files and, if files are
longer than a laptop screen, positions within files are appropriate
responses.

You may wish to add a sentence that explains how you think the
specified code snippets answer the request. If you did *not* factor
out these pieces of functionality into separate functions/methods, say
so.

