
pub struct screenState {
  data: Vec<u8>
}

impl CommandPacket {

  pub fn new(command: u8) -> Self {
    CommandPacket{data : vec![command]}
  }

}
