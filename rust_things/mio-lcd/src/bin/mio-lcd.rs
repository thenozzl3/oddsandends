
use clap::Parser;
use mio::{Poll, PollOpt, Events, Token, Ready};
use mio::unix::UnixReady;
use std::io::Read;
use std::sync::mpsc;
use std::env;
use crossbeam_utils::thread;
use mio_lcd::packet;

const SERIAL_TOKEN: Token = Token(0);

#[derive(Parser,Debug)]
struct Args {
  #[clap(short, long, value_parser, default_value = "/dev/tty.usbserial-00003")]
  arg_device: String
}

pub fn main() {

  let (thread_tx, thread_rx) = mpsc::channel::<Vec<u8>>();
  let mut args = Args::parse();

  let tty_path = args.arg_device;

  let poll = Poll::new().unwrap();
  let mut events = Events::with_capacity(1024);

  // Create the listener
  let mut settings = mio_serial::SerialPortSettings::default();
  settings.baud_rate = mio_serial::BaudRate::Baud115200;

  let mut rx = mio_serial::Serial::from_path(&tty_path, &settings).unwrap();

  // Disable exclusive mode
  rx.set_exclusive(false)
    .expect("Unable to set serial port into non-exclusive mode.");

  let mut wx = mio_serial::Serial::from_path(&tty_path, &settings).unwrap();
  // Disable exclusive mode
  wx.set_exclusive(false)
    .expect("Unable to set serial port into non-exclusive mode.");

  poll.register(&rx,
    SERIAL_TOKEN,
    Ready::readable() |
    UnixReady::hup() |
    UnixReady::error(),
    PollOpt::edge()).unwrap();

  let mut rx_buf = [0u8; 128];

  let mut some_packet = packet::CommandPacket::new(0x06);

  // receiving thread ..
  //match thread::scope(|scope| {
  thread::scope(|scope| {
    scope.spawn(move |_| {
      loop {
        match thread_rx.recv() {
          Ok(ref some_buf) => {
           /* for elem in some_buf {
              print!(" {:x} ", elem );
            } */
            match some_buf[0] {
              0x80 => {
                //occasionally we get incomplete packets
                //2 bytes is the minimum packet size that
                //would be returned under any circumstance..
                if some_buf.len() > 2 {
                  match some_buf[2] {
                    0x06 => {
                      println!("exit key pressed");
                      some_packet.reset();
                      some_packet.set_command(0x06);
                    },
                    0x05 => {
                      println!("enter key pressed");
                      some_packet.reset();
                      some_packet.set_command(0x0c);
                      some_packet.add_data(&[0x01]);
                    },
                    0x04 => print!("right key pressed"),
                    0x03 => print!("left key pressed"),
                    0x01 => {
                      println!("up key pressed");
                      some_packet.reset();
                      some_packet.set_command(0x0b);
                      some_packet.add_data(&[0x01,0x01]);
                    },
                    0x02 => print!("down key pressed"),
                    _    => print!("key released")
                  }
                }
                some_packet.assemble();
                match packet::interact(&mut wx,&some_packet){
                  Err(_) => print!("bad stuff happened when sending.."),
                  Ok(_)  => ()
                };
              },
              0x40 => println!("ack .. "),
              _    => print!("ppbly and ack .. ")
            };
          },
          Err(_) => print!("bad stuff happened.."),
        }
      }
    });

    loop {
      poll.poll(&mut events, None).unwrap();

      if events.is_empty() {
        println!("Read timed out!");
        continue;
      }

      for event in events.iter() {
        match event.token() {
          SERIAL_TOKEN => {
            let ready = event.readiness();

            if ready.contains(UnixReady::hup() | UnixReady::error()) {
              println!("Quitting due to event: {:?}", ready);
              break ;
            }

            if ready.is_readable() {
              match rx.read(&mut rx_buf) {
                Ok(b) => {
                  match b {
                    b if b > 0 => {
                      println!("");
                      //transmit stuff here to the writing loop...
                      if rx_buf[0] >> 4 == 8  {
                        let mut v: Vec<u8> = Vec::new();
                        for elem in rx_buf[..b].iter() {
                          v.push(*elem);
                        }
                        thread_tx.send(v).unwrap();
                      }
                    }
                    _ => println!("Read would have blocked."),
                  }
                }
                Err(e) => println!("Error:  {}", e),
              }
            }
          }
          t => unreachable!("Unexpected token: {:?}", t),
        }
      }
    }
  }).expect("cannot spawn receiving thread..");
}

