# xxshim #

A minimalist hooking library for Reth.

## Usage ##

Use the `exex!` macro to define your ExEx.

```rust
extern crate xxshim;
use xxshim::*;

exex!(
    "My First xxshim!",
    async |x| { println!("{x:?}") },
    async |x, y| { println!("{x:?} {y:?}") },
    async |x| { println!("{x:?}") }
);
```

This produces a fully-fledged command-line application that can be invoked like so:

```
Usage: myshim [OPTIONS]

Options:
  -d, --dev      
  -h, --help     Print help
  -V, --version  Print version
```

By default, your xxshim just runs Reth with an installed ExEx containing the logic that you defined via the `exex!` macro.

## Background ##

When you write an ExEx manually, you're provided with an event stream from Reth that exposes a reorg-aware blockchain state. There are three messages that you need to handle:

 - A new block, `ChainCommitted`
 - A reorg, `ChainReorged`,
 - A reversion, `ChainReverted`

As such, the `exex!` macro accepts four parameters:

 - The name of your ExEx
 - Three functions: one per case above

