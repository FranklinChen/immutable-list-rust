# Rust implementation of immutable, persistent list for functional programming

![Continuous integration](https://github.com/FranklinChen/immutable-list-rust/workflows/Continuous%20integration/badge.svg)

This is a sample implementation of the humble purely functional linked list.

Some notable entertaining features:

- Use of `unsafe` to create an iterative version of `map`, through mutation under the scenes through raw pointers.

## Inspiration

I was inspired to do this little project by Alexis Beingessner's boo
on ["Too Many Lists"](http://cglab.ca/~abeinges/blah/too-many-lists/book/) (and copied some of the
starter code from its
[repo](https://github.com/Gankro/too-many-lists)), as well as his book
on
[unsafe Rust](https://doc.rust-lang.org/nightly/nomicon/). Fantastic
resources for anyone wanting to get deep into Rust!

## Please don't overuse linked lists

I should note that this little project is mostly for instruction and
entertainment. I have not yet personally encountered a situation in
which I wanted to use an immutable, persistent list in Rust. Rust is
not Clojure or Haskell or OCaml or Scala or Erlang or
(...), so the list is usually not the best data structure to use.

(In those other languages also, use other data structures when storing
a lot of data for the purpose of lookup or combination later.)

If *not* caring about lookup, and only caring about *pulling*,

- `Vec` is the way to store stuff
- `Iterator` is a great trait to implement as needed
- slices are the way to get a view and in some situations simulate the
  structural sharing an immutable list gives you

Here is a sample project that [uses these features](https://github.com/FranklinChen/number-words-rust) where in other
languages I might use a list instead.
