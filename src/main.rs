use std::{io::Write, path::PathBuf};

use bindings::{
    Windows::Graphics::Imaging::BitmapDecoder,
    Windows::Media::Ocr::*,
    Windows::Storage::{FileAccessMode, StorageFile},
};

use serde::Serialize;
use structopt::StructOpt;

/// 识别PNG,JPG文件并转换为文字
#[derive(Debug, Clone, StructOpt)]
struct Command {
    /// 源文件路径
    source: PathBuf,

    /// 输出文件路径
    #[structopt(short, long)]
    output: Option<PathBuf>,

    /// 输出是否带位置信息
    #[structopt(short, long)]
    complete: bool,
}

#[derive(Debug, Serialize)]
struct Word {
    text: String,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

fn main() -> windows::Result<()> {
    let args = Command::from_args();
    futures::executor::block_on(main_async(args))
}

async fn main_async(args: Command) -> windows::Result<()> {
    let Command {
        source,
        output,
        complete,
    } = args;
    let mut message = std::env::current_dir().unwrap();
    message.push("message.png");

    let source_file =
        StorageFile::GetFileFromPathAsync(source.to_str().expect("源文件路径错误"))?.await?;
    let stream = source_file.OpenAsync(FileAccessMode::Read)?.await?;
    let decode = BitmapDecoder::CreateAsync(stream)?.await?;
    let bitmap = decode.GetSoftwareBitmapAsync()?.await?;
    let engine = OcrEngine::TryCreateFromUserProfileLanguages()?;
    let result = engine.RecognizeAsync(bitmap)?.await?;

    let dest = output.unwrap_or_else(|| {
        let mut p: PathBuf = source.clone();
        if complete {
            p.set_extension(".json");
        } else {
            p.set_extension("txt");
        }
        p
    });
    let mut file = std::fs::File::create(dest).expect("无法创建目标文件");
    let content = if complete {
        let mut complete_data = vec![];
        for line in result.Lines().unwrap() {
            let mut worlds = vec![];
            for word in line.Words().unwrap() {
                let rect = word.BoundingRect().unwrap();
                worlds.push(Word {
                    text: word.Text().unwrap().to_string(),
                    x: rect.X,
                    y: rect.Y,
                    width: rect.Width,
                    height: rect.Height,
                })
            }
            complete_data.push(worlds)
        }
        serde_json::to_string_pretty(&complete_data).unwrap()
    } else {
        let mut content = String::new();
        for line in result.Lines().unwrap() {
            content.push_str(&line.Text().unwrap().to_string_lossy().trim());
            content.push('\n');
        }
        content
    };
    file.write_all(content.as_bytes()).unwrap();
    Ok(())
}
