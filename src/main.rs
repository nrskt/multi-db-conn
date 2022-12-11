use multi_db_conn::{sample_logic, MultipleConnectionPool};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let ctx = MultipleConnectionPool::new(
        "postgres://postgres:password1@localhost:15432/docker",
        "postgres://postgres:password2@localhost:15433/docker",
    )?;
    sample_logic(&ctx, 1).await?;
    sample_logic(&ctx, 2).await?;
    Ok(())
}
