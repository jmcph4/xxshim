use std::sync::Arc;

pub use alloy_primitives;
pub use clap;
pub use eyre;
pub use futures::{Future, TryStreamExt};
pub use reth;
pub use reth::providers::Chain;
pub use reth_exex::{ExExContext, ExExEvent, ExExNotification};
pub use reth_node_api::FullNodeComponents;
pub use reth_node_api::{FullNodeTypes, NodeTypes};
pub use reth_node_ethereum::EthereumNode;
pub use reth_tracing::tracing::info;

pub type Blockchain = Arc<Chain<<<EthereumNode as FullNodeTypes>::Types as NodeTypes>::Primitives>>;

#[macro_export]
macro_rules! exex {
    ($s:literal, $on_new:expr, $on_reorg:expr, $on_revert:expr) => {
        use alloy_primitives::Address;
        use clap::Parser;

        pub async fn exex_init<Node: FullNodeComponents>(
            ctx: ExExContext<Node>,
        ) -> eyre::Result<impl Future<Output = eyre::Result<()>>> {
            Ok(exex_entrypoint(ctx))
        }

        async fn exex_entrypoint<Node: FullNodeComponents>(mut ctx: ExExContext<Node>) -> eyre::Result<()> {
            while let Some(notification) = ctx.notifications.try_next().await? {
                match &notification {
                    ExExNotification::ChainCommitted { new } => {
                        info!(committed_chain = ?new.range(), "Received commit");
                        $on_new(new).await
                    }
                    ExExNotification::ChainReorged { old, new } => {
                        info!(from_chain = ?old.range(), to_chain = ?new.range(), "Received reorg");
                        $on_reorg(old, new).await
                    }
                    ExExNotification::ChainReverted { old } => {
                        info!(reverted_chain = ?old.range(), "Received revert");
                        $on_revert(old).await
                    }
                };

                if let Some(committed_chain) = notification.committed_chain() {
                    ctx.events.send(ExExEvent::FinishedHeight(committed_chain.tip().num_hash()))?;
                }
            }

            Ok(())
        }

        pub const DEFAULT_RETH_ARGS: [&str; 2] = ["reth", "node"];
        pub const DEFAULT_TESTNET: &str = "hoodi";

        #[derive(Clone, Debug, Parser)]
        #[clap(author, version, about)]
        pub struct Opts {
            #[clap(short, long, action)]
            pub dev: bool,
        }

        fn main() -> eyre::Result<()> {
            let opts = Opts::parse();

            let reth_args = if opts.dev {
                let mut xs = DEFAULT_RETH_ARGS.to_vec();
                xs.extend_from_slice(&["--chain", DEFAULT_TESTNET]);
                xs
            } else {
                DEFAULT_RETH_ARGS.to_vec()
            };

            reth::cli::Cli::try_parse_args_from(reth_args)?.run(
                |builder, _| async move {
                    let handle = builder
                        .node(EthereumNode::default())
                        .install_exex($s, exex_init)
                        .launch()
                        .await?;

                    handle.wait_for_node_exit().await
                },
            )
        }
    };
}
