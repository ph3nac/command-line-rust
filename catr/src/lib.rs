use std::{borrow::Borrow, error::Error};

use clap::{App, Arg};

// 構造体の定義はuse分の後に書くのが一般的
#[derive(Debug)] // deriveマクロを利用して構造体を表示できるようにする
pub struct Config {
  files: Vec<String>,
  number_lines:bool,
  number_nonblank_lines:bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> MyResult<()> {
  println!("Hello world!");
  Ok(())
}

// 成功時にConfigを含むOk，そうでなければErrorを返す
pub fn get_args() -> MyResult<Config>{
  let matches = App::new("catr")
    .version("0.1.0")
    .author("ph3nac <ph3nac@gmail.com")
    .about("Rust cat")
    .arg(
      Arg::with_name("files")
      .value_name("FILE")
      .help("Input files")
      .required(true)
      .min_values(1),
    )
    .arg(
      Arg::with_name("number_lines")
      .short("n")
      .help("Print number lines")
      .takes_value(false)
    )
    .arg(
      Arg::with_name("number_nonblank_lines")
      .short("b")
      .help("Print number nonblank lines")
      .takes_value(false)
    )
    .get_matches();
  let files = matches.values_of_lossy("files").unwrap();
  let number_lines = matches.is_present("number_lines");
  let number_nonblank_lines = matches.is_present("number_nonblank_lines");

  Ok(Config{
    files,
    number_lines,
    number_nonblank_lines,
  })

}
