use anyhow::Error;
use dashboard::Dashboard;
use octocrab::models::repos::Release;
use rillrate::prime::{Click, ClickOpts};
use tokio::time::sleep;
use std::fs::File;
use std::io::copy;
use std::time::Duration;
use tempfile::Builder;

mod dashboard;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Add update button

    let tmp_dir = Builder::new().prefix("example").tempdir()?;
    // let target = "https://github.com/AngelOnFira/rusty-halloween/releases/latest/download/rusty-halloween";
    // https://api.github.com/repos/jgm/pandoc/releases/latest
    // useragent: AngelOnFira/kaisa
    let target = "https://github.com/alacritty/alacritty/releases";

    Dashboard::init().await?;

    let releases = get_releases().await?;

    let mut clicks = Vec::new();

    // Print the releases
    for release in releases {
        println!("{}", release.tag_name);
        // See if there is an asset named rusty-halloween
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == "alacritty.bash");

        if let Some(asset) = asset {
            println!("Found asset: {}", asset.name);

            let release_name = format!("Release {}", release.tag_name);
            let click = Click::new(
                format!("app.dashboard.releases.{}", release_name),
                ClickOpts::default().label(release_name),
            );
            let this = click.clone();
            click.sync_callback(move |envelope| {
                if let Some(action) = envelope.action {
                    this.apply();
                    // TODO: Rename to `feed`
                }
                Ok(())
            });

            clicks.push(click);
        }
    }

    sleep(Duration::from_secs(100)).await;

    Ok(())
}

async fn get_releases() -> Result<Vec<Release>, Error> {
    let octocrab = octocrab::instance();

    let page = octocrab
        .repos("alacritty", "alacritty")
        .releases()
        .list()
        // Optional Parameters
        .per_page(10)
        // Send the request
        .send()
        .await?;

    Ok(page.into_iter().collect())
}
