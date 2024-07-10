fn main() {
    // 戻り値がErr(e)に一致するかどうか
    // eはErrorトレイトを実装した値
    // get_argsがOkを返した時，返り値をrunに渡す．どちらかがエラーならばエラー出力して終了
    if let Err(e) = catr::get_args().and_then(catr::run){
        // 標準エラー出力
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
