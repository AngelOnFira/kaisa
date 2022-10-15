use anyhow::Error;
use rillrate::prime::{Click, ClickOpts};
use std::fs::File;
use std::io::copy;
use tempfile::Builder;

mod dashboard;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Add update button
    let click = Click::new(
        "app.dashboard-1.controls-2.click-1",
        ClickOpts::default().label("Click Me!"),
    );
    let this = click.clone();
    click.sync_callback(move |envelope| {
        if let Some(action) = envelope.action {
            this.apply();
            // TODO: Rename to `feed`
        }
        Ok(())
    });
    click.flush()

    let tmp_dir = Builder::new().prefix("example").tempdir()?;
    // let target = "https://github.com/AngelOnFira/rusty-halloween/releases/latest/download/rusty-halloween";
    // https://api.github.com/repos/jgm/pandoc/releases/latest
    // useragent: AngelOnFira/kaisa
    let target = "https://github.com/AngelOnFira/rusty-halloween/releases";

    let octocrab = octocrab::instance();

    let page = octocrab
        .repos("angelonfira", "rusty-halloween")
        .releases()
        .list()
        // Optional Parameters
        .per_page(100)
        // Send the request
        .send()
        .await?;

    for release in page {
        println!("{:#?}", release.assets);
    }

    // // https://github.com/AngelOnFira/rusty-halloween/releases/download/0.1.3/rusty-halloween
    // let response = reqwest::Client::new()
    //     .get(target)
    //     .header("User-Agent", "AngelOnFira/kaisa")
    //     .send()
    //     .await?;

    // // Print out the response
    // println!("{:#?}", response);

    // let mut dest = {
    //     let fname = response
    //         .url()
    //         .path_segments()
    //         .and_then(|segments| segments.last())
    //         .and_then(|name| if name.is_empty() { None } else { Some(name) })
    //         .unwrap_or("tmp.bin");

    //     println!("file to download: '{}'", fname);
    //     let fname = tmp_dir.path().join(fname);
    //     println!("will be located under: '{:?}'", fname);
    //     File::create(fname)?
    // };
    // let content = response.text().await?;
    // copy(&mut content.as_bytes(), &mut dest)?;

    // // Copy the file to the current directory
    // copy(&mut content.as_bytes(), &mut File::create("tmp.bin")?)?;

    Ok(())
}
