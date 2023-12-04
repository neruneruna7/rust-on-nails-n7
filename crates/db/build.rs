use std::env;
use std::path::Path;

fn main() {
    // Compile our SQL
    cornucopia();
}

fn cornucopia() {
    // 簡単にするために，この例ではデフォルトを使用します
    let queries_path = "queries";

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let file_path = Path::new(&out_dir).join("cornucopia.rs");

    let db_url = env::var_os("DATABASE_URL").unwrap();

    // クエリまたはマイグレーションが変更された場合，このビルドスクリプトを再実行します．
    println!("cargo:rerun-if-changed={queries_path}");

    let output = std::process::Command::new("cornucopia")
        .arg("-q")
        .arg(queries_path)
        .arg("--serialize")
        .arg("-d")
        .arg(&file_path)
        .arg("live")
        .arg(db_url)
        .output()
        .unwrap();

    // Cornucopiaが適切に実行できなかった場合，エラーを表示してみる．
    if !output.status.success() {
        panic!("{}", &std::str::from_utf8(&output.stderr).unwrap());
    }
}

