use anyhow::Result;
use util::is_idf;

mod util;

fn main() -> Result<()> {
    if is_idf()? {
        // Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641
        embuild::build::CfgArgs::output_propagated("ESP_IDF")?;
        embuild::build::LinkArgs::output_propagated("ESP_IDF")?;
    }

    Ok(())
}
