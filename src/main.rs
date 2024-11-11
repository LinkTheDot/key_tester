use key_tester::get_user_input;

const TEST_COUNT: u32 = 100;

fn main() {
  println!("Choose a key to test.");

  let test_key = get_user_input();
  let test_key = test_key.trim();

  println!("{:?} was chosen as the test key.", test_key);
  println!("Press the key {} times.", TEST_COUNT);
  println!("{}", "\n".repeat(5));

  let mut press_count = 0;

  while press_count < TEST_COUNT {
    let input = get_user_input();
    let input = input.trim();

    if input == test_key {
      press_count += 1;

      print!("{}", termion::cursor::Up(1));
      print!("{}", termion::clear::CurrentLine);
      println!("Current count: {}", press_count);
    } else {
      println!("Failed input of {:?}\n", input);
    }
  }

  println!("Completed.");
}
