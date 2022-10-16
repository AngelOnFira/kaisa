mod layout;

use anyhow::Error;

use tokio::sync::mpsc::{self};


pub struct Dashboard {}

impl Dashboard {
    pub async fn init() -> Result<(), Error> {
        env_logger::try_init()?;
        rillrate::install("rusty-halloween")?;
        layout::add();

        Ok(())
    }
}
