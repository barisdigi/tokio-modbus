// SPDX-FileCopyrightText: Copyright (c) 2017-2023 slowtec GmbH <post@slowtec.de>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Synchronous RTU over TCP client example

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use tokio_modbus::prelude::*;

    let socket_addr = "127.0.0.1:502".parse().unwrap();

    let mut ctx = sync::rtuovertcp::connect(socket_addr)?;

    println!("Fetching the coupler ID");
    ctx.set_slave(tokio_modbus::Slave(2));
    let data = ctx.read_holding_registers(0x0000, 2)?;
    println!("{:?}", data);
    let bytes: Vec<u8> = data.iter().fold(vec![], |mut x, elem| {
        x.push((elem & 0xff) as u8);
        x.push((elem >> 8) as u8);
        x
    });
    let id = String::from_utf8(bytes).unwrap();
    println!("The coupler ID is '{id}'");

    Ok(())
}
