# HexGame

[![Rust](https://github.com/joepigott/hexgame-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/joepigott/hexgame-rs/actions/workflows/rust.yml)

This is a rewrite in Rust of one of my university assignments. The point of the
assignment (originally in Java) was to implement a `DisjointSet` class, and use
it to determine which player had won the game. To make grading easier and 
deterministic, a sequence of moves were given in a text file and used to
construct the game. In this version, I've made it a sort of two-player CLI
game.

It is basically a direct translation from Java to Rust, so the code is not very
idiomatic. I will continue to update this codebase as I learn and practice 
Rust.
