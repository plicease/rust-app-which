# Rusty which [![Build Status](https://travis-ci.org/plicease/rust-app-which.svg?branch=master)](http://travis-ci.org/plicease/rust-app-which) [![Build status](https://ci.appveyor.com/api/projects/status/inr7epn8aeb3bulx/branch/master?svg=true)](https://ci.appveyor.com/project/plicease/rust-app-which/branch/master)

If you are not familiar with the `which` command:

https://en.wikipedia.org/wiki/Which_(command)

The intent of this app is to implement a nice static `which` for Windows
(GnuWin32 has a port of `which`, but it is not maintained, and requires
DLLs, and a lot of baggage that I don't want or need). It might also be
useful for other platforms that lack a native `which`.  It probably
dosen't make much sense to use this version on Unix platforms which
usually have `which` built into the shell (where it is the most reliable),
but this app does support Unix.

## TODO

 - better documentation in this README
 - tests for finding / not finding a command
 - implement `--all` | `-a`
