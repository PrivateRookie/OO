fn main() {
    windows::build! {
        Windows::Foundation::IAsyncOperation,
        Windows::Foundation::IReference,
        Windows::Foundation::Rect,
        Windows::Foundation::Collections::IVectorView,
        Windows::Globalization::Language,
        Windows::Graphics::Imaging::{BitmapDecoder, SoftwareBitmap},
        Windows::Media::Ocr::*,
        Windows::Storage::Streams::IRandomAccessStream,
        Windows::Storage::{FileAccessMode, StorageFile},
    };
}
