pub struct Success {
  pub code: String,
  pub msg: String,
}

impl Success {
  pub fn res() -> Success {
    Success {
      code: String::from("00"),
      msg: String::from("성공"),
    }
  }
}
