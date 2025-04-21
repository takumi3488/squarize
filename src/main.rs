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

    /// 透過部分を白く塗りつぶす
    #[clap(short = 'w', long)]
    white_background: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // 入力画像を読み込む
    let img = image::open(&args.input)?;

    // 画像のサイズを取得
    let (width, height) = img.dimensions();

    // 正方形のサイズを計算（長辺に合わせる）
    let size = width.max(height);

    // 新しい正方形の画像を作成
    let mut squared = ImageBuffer::new(size, size);

    // 白背景で初期化（-wフラグが指定された場合）
    if args.white_background {
        for px in squared.pixels_mut() {
            *px = image::Rgba([255, 255, 255, 255]);
        }
    }

    // 中央に配置するための開始位置を計算
    let x = (size - width) / 2;
    let y = (size - height) / 2;

    // 元の画像を新しい正方形画像の中央にコピー
    for (i, j, pixel) in img.pixels() {
        // -wフラグが指定されている場合、透明部分を白く塗りつぶす
        if args.white_background && pixel[3] < 255 {
            squared.put_pixel(i + x, j + y, image::Rgba([255, 255, 255, 255]));
        } else {
            squared.put_pixel(i + x, j + y, pixel);
        }
    }

    // 出力ファイルに保存
    squared.save(&args.output)?;

    println!("画像を正方形に変換して保存しました: {:?}", args.output);
    Ok(())
}
