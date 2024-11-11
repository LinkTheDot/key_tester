use std::io;
use std::io::{Read, Write};
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

pub fn get_user_input() -> String {
  let stdin = 0;
  let mut termios = Termios::from_fd(stdin).unwrap();
  let mut buffer = vec![0; 1]; // read exactly one byte

  termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
  tcsetattr(stdin, TCSANOW, &termios).unwrap();

  io::stdout().lock().flush().unwrap();
  let _ = io::stdin().read_exact(&mut buffer);

  tcsetattr(stdin, TCSANOW, &termios).unwrap(); // reset the stdin to original termios data

  String::from_utf8(buffer.to_vec()).unwrap()
}
