mod entries;
mod apis;
// mod secrets;

use std::fmt::format;
use std::string::ToString;
use cliclack::{intro, outro, select, set_theme};
use crate::apis::curse::CurseApi;
use crate::entries::{Entry, EntryKind};
use crate::entries::mcp_entry::MCPEntry;
use crate::entries::mods_entry::ModsEntry;

const BANNER : &str = r#"
  _____ _               _   _           _       _
  |  ___| | _____      _| | | |_ __   __| | __ _| |_ ___ _ __
  | |_  | |/ _ \ \ /\ / / | | | '_ \ / _` |/ _` | __/ _ \ '__|
  |  _| | | (_) \ V  V /| |_| | |_) | (_| | (_| | ||  __/ |
  |_|   |_|\___/ \_/\_/  \___/| .__/ \__,_|\__,_|\__\___|_|
     _ ____   ___  _   _    |_|_ ____  _____    _  _____ ___  ____
    | / ___| / _ \| \ | |  / ___|  _ \| ____|  / \|_   _/ _ \|  _ \
 _  | \___ \| | | |  \| | | |   | |_) |  _|   / _ \ | || | | | |_) |
| |_| |___) | |_| | |\  | | |___|  _ <| |___ / ___ \| || |_| |  _ <
 \___/|____/ \___/|_| \_|  \____|_| \_\_____/_/   \_\_| \___/|_| \_\
"#;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // let subtitle: String = format!(":: {} ::\t\t\t\t\t\t     (v{})", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).to_string();
    
    // intro(format!("{}{}", BANNER, subtitle))?;
    
    // let selected_entry: EntryKind = select("What kind of JSON do you want to generate ?")
    //     .item(EntryKind::ModEntry, "Mods (mods.json)", "")
    //     .item(EntryKind::MCPEntry, "MCP (mcp.json)", "")
    //     .interact()?;
    
    // let entry: Box<dyn Entry> = match selected_entry {
    //     EntryKind::ModEntry => Box::new(ModsEntry{}),
    //     EntryKind::MCPEntry => Box::new(MCPEntry{}),
    // };
    
    // entry.start();
    
    // outro("Thanks you for using");

    let client = CurseApi::new();
    let _ = client.get_minecraft_versions().await?.data.iter().for_each(|version| {
        println!("{}", version.version_string);
    });

    Ok(())
}
