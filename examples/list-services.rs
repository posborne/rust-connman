// Copyright (c) 2018, the rust-connman authors.

extern crate connman;

fn main() -> Result<(), connman::Error> {
    let mut manager = connman::Manager::new()?;
    for service in manager.services()? {
        println!("{}", service.name()?);
    }

    Ok(())
}
