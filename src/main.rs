use clap::Parser;
use image::{GenericImageView, ImageBuffer};
use std::path::PathBuf;

/// PNG画像を正方形にするツール
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// 入力PNG画像のパス
    input: PathBuf,

    /// 出力PNG画像のパス
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // 入力画像を読み込む
    let img = image::open(&args.input)?;

    // 画像のサイズを取得
    let (width, height) = img.dimensions();

    // 正方形のサイズを計算（長辺に合わせる）
    let size = width.max(height);

    // 新しい正方形の画像を作成（透明で初期化）
    let mut squared = ImageBuffer::new(size, size);

    // 中央に配置するための開始位置を計算
    let x = (size - width) / 2;
    let y = (size - height) / 2;

    // 元の画像を新しい正方形画像の中央にコピー
    for (i, j, pixel) in img.pixels() {
        squared.put_pixel(i + x, j + y, pixel);
    }

    // 出力ファイルに保存
    squared.save(&args.output)?;

    println!("画像を正方形に変換して保存しました: {:?}", args.output);
    Ok(())
}
