use hyper::{body::HttpBody, Client};
use hyperlocal::{UnixClientExt, Uri};
use std::error::Error;
use tokio::io::{self, AsyncWriteExt as _};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    async_std::task::block_on(async {
        let url = Uri::new("/tmp/hyperlocal.sock", "/").into();

        let client = Client::unix();

    let mut response = client.get(url).await?;

    while let Some(next) = response.data().await {
        let chunk = next?;
        io::stdout().write_all(&chunk).await?;
    }

        Ok(())
    })
}
