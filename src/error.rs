pub trait ScannerError {
  fn error(&mut self, line: usize, msg: String);
  fn report(&mut self, line: usize, where_: String, msg: String);
}