# Basic Solana Program Examples

This repository contains the code used in these two Solana Tutorials Youtube videos:

- [Solana Program Entrypoint Explained](https://www.youtube.com/watch?v=4qzMc4Wc54U&ab_channel=SolanaTutorials)
- [Solana Hello World Program in Rust](https://www.youtube.com/watch?v=x7SbnkOzc18&ab_channel=SolanaTutorials).

## Prerequisites

- Follow the instructions to install the [Solana CLI and Anchor CLI](https://solana.com/docs/intro/installation).

## Programs

- [Native Program](native_program) - A basic Rust program using the standard Solana entrypoint.
- [Pinocchio Program](pinocchio_program) - A program that uses an optimized custom entrypoint implementation (from the `pinocchio` crate). Moving forward, you should use `pinocchio` when building your own programs.
- [Anchor Program](anchor_program) - A basic program built with the Anchor framework
