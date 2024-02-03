use anyhow::Error;
use portable_pty::{native_pty_system, CommandBuilder, PtySize, PtySystem};
use std::io::{self, Read, Write};

fn main() -> anyhow::Result<()> {
    // Use the native pty implementation for the system
    let pty_system = native_pty_system();

    // Create a new pty
    let mut pair = pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        // Not all systems support pixel_width, pixel_height,
        // but it is good practice to set it to something
        // that matches the size of the selected font.  That
        // is more complex than can be shown here in this
        // brief example though!
        pixel_width: 0,
        pixel_height: 0,
    })?;

    // Spawn a shell into the pty
    let cmd = CommandBuilder::new("bash");
    let child = pair.slave.spawn_command(cmd)?;

    println!("Spawned shell");

    // Read and parse output from the pty with reader
    let mut reader = pair.master.try_clone_reader()?;
    let mut writer = pair.master.take_writer()?;
    println!("writing data");

    // Send data to the pty by writing to the master
    writeln!(writer, "echo hi\r\n")?;

    println!("reading");

    // let mut output = [0; 10];
    // reader.read(&mut output);
    let mut b = reader.bytes();

    loop {
        // let c = std::io::stdin().bytes().next();
        // if let Some(c_ok) = c {
        //     write!(writer, "{}", c_ok? as char)?;
        // }
        print!("{}", b.next().unwrap()? as char);
        io::stdout().flush();
    }

    Ok(())
}
