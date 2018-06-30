// mbus_write : commandline tool for executing a Modbus RTU
// Function 6: write_single_register command

// import clap, a library for parsing commandline arguments
extern crate clap;

use clap::{Arg, App};

// import tokio-modbus libraries
extern crate futures;
extern crate tokio_core;
extern crate tokio_modbus;
extern crate tokio_serial;

use tokio_core::reactor::Core;
use futures::future::Future;
use tokio_serial::*;
use tokio_modbus::*;

fn main() {
    let matches = App::new("mbus_write")
        .version("0.1.0")
        .author("Andrew Reid <gnomad@cryptolab.net>")
        .about("Execute Modbus RTU write_single_register command")
        .arg(Arg::with_name("MODBUS_ADDRESS")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("modbus address to write to"))
        .arg(Arg::with_name("REGISTER")
                 .required(true)
                 .takes_value(true)
                 .index(2)
                 .help("register to write to"))
        .arg(Arg::with_name("VALUE")
                 .required(true)
                 .takes_value(true)
                 .index(3)
                 .help("value to write to the register"))
        .get_matches();
    // Assign variables based on input parameters
    let modbus_address = matches.value_of("MODBUS_ADDRESS").unwrap();
    let register = matches.value_of("REGISTER").unwrap();
    let value = matches.value_of("VALUE").unwrap();
    // Print values to consider for error-checking (dev feature)
    println!("Modbus address: {}", modbus_address);
    println!("Register: {}", register);
    println!("Value: {}", value);
    println!("------------------------------");
    // Parse strings to unsigned integers (required for tokio-modbus)
    let modbus_addr: u8 = modbus_address
        .trim()
        .parse()
        .expect("Failed to parse");
    let register: u16 = register.parse().unwrap();
    let value: u16 = value.parse().unwrap();

// Function to perform Modbus RTU write_holding_register command
// pub fn write_registers() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let tty_path = "/dev/serial0";
    //let server_addr = 0x01;

    let mut settings = SerialPortSettings::default();
    settings.baud_rate = BaudRate::Baud19200;
    
    let mut port = Serial::from_path_with_handle(tty_path, &settings, &handle.new_tokio_handle())
        .expect(&format!("Unable to open serial device '{}'", tty_path));
    port.set_exclusive(false)
        .expect("Unable to set serial port exlusive");
    
    //let mut port = Serial::from_path(tty_path, &settings).unwrap();
    //port.set_exclusive(false).unwrap();

    let task = Client::connect_rtu(port, modbus_addr, &handle).and_then(|client| {
        println!("Writing register");
        client
            .write_single_register(register, value)
            .and_then(move |res| {
                println!("Response: {:?}", res);
                Ok(())
            })
    });

    core.run(task).unwrap();
}
