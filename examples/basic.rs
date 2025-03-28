extern crate xxshim;
use xxshim::*;

pub async fn on_new<N: NodePrimitives>(new: &Blockchain<N>) -> eyre::Result<()> {
    println!("New block: {}", new.tip().hash());
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

exex!("My First xxshim!", on_new, on_reorg, on_revert);
