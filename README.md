# actiondb

## Rust things

### Run only one test without muting stdout

```
cargo test -- --nocapture matcher::trie::node::node::given_empty_trie_when_literals_are_inserted_then_they_can_be_looked_up
```

### You need to move out a resource from &mut self

You can do this by destructoring it via a `let` binding. The destructoring
function (like `split()`) takes `self`, not a reference. Then it can destructor
it.

### Reference has a longer lifetime than the data it references

You can extend a lifetime with the following syntax:

```rust
struct LiteralLookupHit<'a, 'b: 'a, 'c>(&'a mut Node<'b>, &'c str);
```

### Cannot borrow mutable X multiple times, but there is only one active borrow
Check your lifetimes. If there is one on `self` it can cause problems. If a trait
doesn't need a litetime just one of its methods, then place the lifetime on the method
and not on the trait.

## Grammar
The grammar is generated by the awesome `rust-peg` crate. Unfortunately it's
compatible only with nightly Rust and we stick to `1.0` stable, so we need
a way to generate the grammar.

I use a `docker` container to generate the grammar files. Here are the steps:

1. Clone `rust-peg` into a directory (`~/workspace/rust-peg`)
2. Start a docker container wich has the current nightly Rust installed

```
docker run --rm -it -v ~/workspace/rust-peg:/source schickling/rust
cargo build
```

This will build the crate and produce a `peg` binary under
`/source/target/debug`. This `peg` binary reads a grammar definition from a
file and prints to the stdout the generated Rust code.

3. Start a docker container which has access to `actiondb`'s source code and to the `peg` binary:

```
docker run -it -v ~/workspace/rust-peg:/source -v ~/workspace/actiondb:/actiondb schickling/rust /bin/bash
```

4. Generate the grammar files with `peg`:

```
target/debug/peg /actiondb/src/grammar/pattern.rustpeg > /actiondb/src/grammar/pattern_parser.rs; echo $?
```

5. Rebuild `actiondb` with `cargo`

```
cargo build
```
