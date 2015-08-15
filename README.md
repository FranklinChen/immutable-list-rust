# Rust implementation of immutable, persistent list for functional programming

[![Build Status](https://travis-ci.org/FranklinChen/immutable-list-rust.png)](https://travis-ci.org/FranklinChen/immutable-list-rust)

This is a sample implementation of the humble purely functional linked list.

Some notable entertaining features:

- Use of `unsafe` to allow comparison of list *identity* (versus just structural equality) by secretly exposing implementation details of `Rc`.
- Use of `unsafe` to create an iterative version of `map`, through mutation under the scenes even though everything is supposed to be immutable.
- Use of `unsafe` to get a pointer to a value stored inside an `Rc`, without using runtime checking.

I welcome any suggestions on how to do some of this stuff in a less nasty way.
