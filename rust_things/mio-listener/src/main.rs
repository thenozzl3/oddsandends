extern crate mio;
extern crate bytes;
extern crate getopts;
extern crate env_logger;

use getopts::Options;
use std::env;
use std::str;
#[macro_use] extern crate log;
use mio::*;
use mio::tcp::*;
use bytes::{Buf, ByteBuf, MutByteBuf};
use std::io;
use std::thread;
use std::io::Write;
use std::net::SocketAddr;
use std::str::FromStr;
use std::collections::*;
////use slab;

struct WebSocketServer {
  socket: TcpListener,
  clients: HashMap<Token, TcpStream>,
  token_counter: usize,
  token: Token
}

impl Handler for WebSocketServer{
  type Timeout = ();
  type Message  = ();

  fn ready(&mut self,
    event_loop: &mut EventLoop<WebSocketServer>,
    token: Token,
    events: EventSet){

    if events.is_hup(){
      let  bye_msg = "leaving the party";
        let mut msg_buf = ByteBuf::mut_with_capacity(2048);
        msg_buf.write_slice(bye_msg.as_bytes());
      self.send_resp(&mut msg_buf, &token);
      self.clients.remove(&token);
      return;
    }

    if events.is_error(){
      let bye_msg = "error...";
        let mut msg_buf = ByteBuf::mut_with_capacity(2048);
        msg_buf.write_slice(bye_msg.as_bytes());
      self.send_resp(&mut msg_buf, &token);
      self.clients.remove(&token);
      return;
    }

    match token {
      Token(1) => {

        println!("incoming connection!");
        let client_socket = match self.socket.accept(){
          Err(e) => { println! ("Accept error: {}",e);return;},
          Ok(None) => unreachable!(),
          Ok(Some((sock,_))) => sock
        };
        self.token_counter += 1;
        let new_token = Token(self.token_counter);
        //self.clients.insert(new_token, WebSocketClient::new(client_socket));
        self.clients.insert(new_token, client_socket);


        let mut interest = EventSet::hup();
        interest.insert(EventSet::readable());


        event_loop.register(&self.clients[&new_token],
          new_token,
          interest,
          PollOpt::edge()|PollOpt::oneshot()).unwrap();
      },
      token => {
        println!("incoming data ..");
        let mut recv_buf = ByteBuf::mut_with_capacity(2048);

          {
            let client = self.clients.get_mut(&token).unwrap();

            loop {
                match client.try_read_buf(&mut recv_buf){


                Ok(Some(n)) => {

                  println!("we got {} bytes .. ", n);
                  println!("from client : {}", client.peer_addr().unwrap());

                  if n < 2048 {
                    break;
                  }
                },
                Ok(None) => {
                  break;
                },
                Err(_) => {
                  break;
                }
            }
          };
        }
        self.send_resp(&recv_buf, &token);

        let mut client = self.clients.get_mut(&token).unwrap();

        let mut interest = EventSet::hup();
        interest.insert(EventSet::readable());
        interest.insert(EventSet::error());


        event_loop.reregister(client ,
        token,
        interest,
        PollOpt::edge()|PollOpt::oneshot()).unwrap();
      }
    }
  }
}

impl WebSocketServer {
  fn new(socket: TcpListener) -> WebSocketServer{
    WebSocketServer {
      socket: socket,
      token_counter: 1,
      clients: HashMap::<Token, TcpStream>::new(),
      token: Token(1)
    }
  }

  fn send_resp(&mut self, buf: &MutByteBuf, token: &Token){


    let client = self.clients.get_mut(&token).unwrap().peer_addr().unwrap();
    for (loc_token, socket) in self.clients.iter_mut(){
      if loc_token.ne(token) {
          let client_addr = format!("{} : ",client);

          let full_string: &str = &(client_addr + &match str::from_utf8(buf.bytes()) {
            Ok(m) => {m}
            //Err(f) => {panic!(f.to_string())}
            Err(_) => {"huh ? \n"}
          });
          socket.write(full_string.as_bytes());
      }
    }
  }

  fn register(&mut self, event_loop: &mut EventLoop<WebSocketServer>) -> io::Result<()>{
    event_loop.register(&mut self.socket,
      self.token,
      EventSet::readable(),
      PollOpt::edge()).or_else(|e|{
        error!("reg failed {:?} {:?}", self.token, e);
        Err(e)
      })
  }
}

/*
impl WebSocketClient {
  fn new(socket: TcpStream){
  }
}
*/
fn main() {
  let args: Vec<String> = env::args().collect();
  let mut opts = Options::new();

  opts.optopt("p","","set port to listen on", "NUMBER");
  let matches = match opts.parse(&args[1..]){
    Ok(m) => {m}
    Err(f) => {panic!(f.to_string())}
  };

  let port = match matches.opt_str("p") {
    Some(m) => {m}
    None => {panic!("no valid port")}
  };

  env_logger::init().ok().expect("Failed to start loggy things");

  let addr: SocketAddr = FromStr::from_str(&format!("0.0.0.0:{}", port))
    .ok().expect("no address for you !");
  let sock = TcpListener::bind(&addr).ok().expect("no socket for you !");

  let mut event_loop = EventLoop::new().ok().expect("no event loop !");
  let mut server = WebSocketServer::new(sock);
  server.register(&mut event_loop).ok().expect("failed to reg");
  event_loop.run(&mut server).ok().expect("not loopy!");
}
