//! Pretty print host interfaces.

use nl80211::Socket;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut socket = Socket::connect()?;

    for interface in socket.get_interfaces_info()? {
        dbg!(&interface);

        if let Some(index) = &interface.index {
            dbg!(socket.get_station_info(index)?);
            dbg!(socket.get_bss_info(index)?);
        }

        eprintln!("---");
    }

    Ok(())
}
