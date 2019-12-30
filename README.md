# Rusty which

The intent of this app is to implement a nice static `which` for Windows
(GnuWin32 has a port of `which`, but it is not maintained, and requires
DLLs, and a lot of baggage that I don't want or need). It might also be
useful for other platforms that lack a native `which`.  It probably
dosen't make much sense to use this version on Unix platforms which
usually have `which` built into the shell (where it is the most reliable

## TODO

 - better documentation in this README
 - Add travis and appveyor buttons
 - tests for finding a file
 - implement `--all` | `-a`
