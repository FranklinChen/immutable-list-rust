# Rust implementation of immutable, persistent list for functional programming

[![Build Status](https://travis-ci.org/FranklinChen/immutable-list-rust.png)](https://travis-ci.org/FranklinChen/immutable-list-rust)

This is a sample implementation of the humble purely functional linked list.

Some notable features:

- Use of `unsafe` to allow comparison of list *identity* (versus just structural equality).
- Use of `unsafe` to create an iterative version of `map`.
