## RUN THE PROGRAM
$ cargo run -- searchstring example-filename.txt


## About
we’ll make our own version of the classic command line search tool grep (globally search a regular expression and print). In the simplest use case, grep searches a specified file for a specified string. To do so, grep takes as its arguments a file path and a string. Then it reads the file, finds lines in that file that contain the string argument, and prints those lines.

Along the way, we’ll show how to make our command line tool use the terminal features that many other command line tools use. We’ll read the value of an environment variable to allow the user to configure the behavior of our tool. We’ll also print error messages to the standard error console stream (stderr) instead of standard output (stdout), so, for example, the user can redirect successful output to a file while still seeing error messages onscreen.

One Rust community member, Andrew Gallant, has already created a fully featured, very fast version of grep, called ripgrep. By comparison, our version will be fairly simple, but this chapter will give you some of the background knowledge you need to understand a real-world project such as ripgrep.
