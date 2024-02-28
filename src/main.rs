// 特定のテキストファイルを読み込んで解析するサンプルコード
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead };
use std::thread::sleep;
use indicatif::{ProgressBar, ProgressStyle};
use env_logger;
use log::{error, info};
use std::env;
use csv::WriterBuilder;

fn main() {
    // ログ設定
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    // ログファイルのパス
    let input_file_path = "sample/sample.log";
    // CSVファイルのパス
    let output_file_path = "sample/sample.csv";

    match convert(input_file_path, output_file_path) {
        Ok(()) => {
            info!("success");
        }
        Err(err) => {
            error!("error: {}", err);
        }
    };
}

fn convert(input_file: &str, output_file: &str) -> Result<(), Box<dyn Error>> {
    // ファイルの行数を取得する
    let file = File::open(input_file)?;
    let reader = io::BufReader::new(file);
    let line_count = reader.lines().count();

    // プログレスバーの設定
    let pb = ProgressBar::new(line_count as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")?
        .progress_chars("#>-"));

    // ファイルを再度開く
    let file = File::open(input_file)?;
    let reader = io::BufReader::new(file);

    // 出力用のファイルを開く
    let mut writer = WriterBuilder::new()
        .from_path(output_file)?;
    // ヘッダーの書き込み
    writer.write_record(&["column1", "column2", "column3"])?;

    for (index, line) in reader.lines().enumerate() {
        let line = line?;

        // 実処理をここに書く
        sleep(std::time::Duration::from_millis(0));

        // プログレスバーを更新
        pb.inc(1);
    }

    // csvに適当な文字列を詰めて出力
    writer.write_record(&["a", "b", "c"])?;
    writer.flush()?;

    // プログレスバーを終了
    pb.finish_with_message("complete");
    Ok(())
}