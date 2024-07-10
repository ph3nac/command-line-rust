fn main() {
    // 戻り値がErr(e)に一致するかどうか
    // eはErrorトレイトを実装した値
    if let Err(e) = catr::run(){
        // 標準エラー出力
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
