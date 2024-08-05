use clap::App;
use clap::Arg;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// 構造体の定義はuse分の後に書くのが一般的
#[derive(Debug)] // deriveマクロを利用して構造体を表示できるようにする
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut line_num = 0;
                for line_result in file.lines() {
                    // 行の読み込み中にエラーが発生した場合，Result<Error>を返す
                    let line = line_result?;
                    line_num += 1;
                    if config.number_lines {
                        println!("{:>6}\t{}", line_num, line);
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

//

// 成功時にConfigを含むOk，そうでなければErrorを返す
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("ph3nac <ph3nac@gmail.com")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"), // echo "xxx" | cat　などで標準出力を受け取る時に使う
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank"), // -n -b を同時指定できない
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Number non-blank lines")
                .takes_value(false),
        )
        .get_matches();
    let files = matches.values_of_lossy("files").unwrap();
    let number_lines = matches.is_present("number_lines");
    let number_nonblank_lines = matches.is_present("number_nonblank");

    Ok(Config {
        files,
        number_lines,
        number_nonblank_lines,
    })
}

// dyn BufReadの大きさが不定なためBoxでヒープに保持する
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))), // 読み込めなかった場合はエラーを伝播させる
    }
}
