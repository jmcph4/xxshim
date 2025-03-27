# xxshim Tutorial #

In this tutorial, we're going to write an xxshim that displays the transaction hashes of all new smart contract deployments to Ethereum.

## Setting up ##

Firstly, this tutorial assumes that you have a local Rust installation that both works and is recent enough.

```
$ cargo new deploy-listener
$ cd deploy-listener
$ cargo add xxshim
```

Now that we have our Cargo project initalised and the library added as a dependency, we can start writing our xxshim.

## Writing the shim ##

All xxshims begin with the `exex!` macro. This macro entirely defines the behaviour of our xxshim: all we need to do is name it and provide the necessary hooks.

We have to handle three possible cases (these are all defined by the [`ExexNotification`](https://reth.rs/docs/reth_exex/enum.ExExNotification.html) type, by the way):

 - What do we do when a new block becomes canonical?
 - What do we do when a reorg occurs?
 - What do we do when the underlying Reth database rolls back?

Our macro handles all of these like so:

```rust
extern crate xxshim;
use xxshim::*;

exex!("Deployment Listener", on_new, on_reorg, on_revert);
```

### Handling new blocks ###

Let's consider our first hook, which handles the case of a new block becoming canonical. For our xxshim, we want to:

 1. Retrieve all of the transactions in this new block
 2. Filter out the ones that are *not* contract creations
 3. Print their hashes to standard output

Let's stub these out:

```rust
extern crate xxshim;
use xxshim::*;

pub async fn on_new<N: NodePrimitives>(new: &Blockchain<N>) -> eyre::Result<()> {
    todo!()
}

pub async fn on_reorg<N: NodePrimitives>(
    _old: &Blockchain<N>,
    _new: &Blockchain<N>,
) -> eyre::Result<()> {
    todo!()
}

pub async fn on_revert<N: NodePrimitives>(_old: &Blockchain<N>) -> eyre::Result<()> {
    todo!()
}

exex!("Deployment Listener", on_new, on_reorg, on_revert);
```

Unfortunately, some of Reth's generics have leaked out into our API. Any time we need to accept blockchain state, we'll need to carry a generic over what flavour of node we're using. xxshim exposes the `Blockchain<N>` type alias to minimise the pain for you, but we'll need explicit type bounds on our hook functions for now.

As for the actual business logic of our xxshim, we want to iterate over each of the transactions in the latest block. Because this tutorial is a fairly contrived example, we won't bother doing anything in neither the reorg nor the revert case (we'll make these no-ops).

```rust
extern crate xxshim;
use xxshim::*;

pub async fn on_new<N: NodePrimitives>(new: &Blockchain<N>) -> eyre::Result<()> {
    new.tip()
        .clone_transactions_recovered()
        .filter(|tx| reth::rpc::types::TransactionTrait::to(tx.inner()).is_none())
        .for_each(|tx| println!("{tx:?}"));
    Ok(())
}

pub async fn on_reorg<N: NodePrimitives>(
    _old: &Blockchain<N>,
    _new: &Blockchain<N>,
) -> eyre::Result<()> {
    Ok(())
}

pub async fn on_revert<N: NodePrimitives>(_old: &Blockchain<N>) -> eyre::Result<()> {
    Ok(())
}

exex!("Deployment Listener", on_new, on_reorg, on_revert);
```

That's it! When you compile and run our xxshim you should be met with the familar Reth startup logging output; but there's a bit more going on with our xxshim. Let's print the help message:

```
$ cargo run -- -h
Usage: deploy-listener [OPTIONS]

Options:
  -d, --dev      
  -h, --help     Print help
  -V, --version  Print version
```

If we need to test our xxshim, we can run it on the [Hoodi](https://hoodi.ethpandaops.io) testnet by specifying the `-d` flag.

