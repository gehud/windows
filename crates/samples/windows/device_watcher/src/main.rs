use windows::{core::*, Devices::Enumeration::*};

fn main() -> Result<()> {
    let watcher = DeviceInformation::CreateWatcher()?;

    watcher.Added(Some(
        |_, info: Ref<DeviceInformation>| {
            println!("{}", info.as_ref().expect("info").Name()?);
            Ok(())
        },
    ))?;

    watcher.EnumerationCompleted(Some(|_, _| {
        println!("done!");
        Ok(())
    }))?;

    watcher.Start()?;
    std::thread::sleep(std::time::Duration::new(10, 0));
    Ok(())
}
