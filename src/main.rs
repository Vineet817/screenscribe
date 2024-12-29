
use image::{ImageBuffer, Rgba, DynamicImage, ImageFormat};
use screenshots::Screen;
use leptess::LepTess;
use std::time::{Duration, Instant};
use std::thread;

use std::io::Cursor;


fn main() {

    let interval = Duration::from_millis(33); // ~30 FPS
    let binding = Screen::all().unwrap();
    let screen = binding.get(0).expect("No screen found");

    // Initialize Tesseract OCR
    let mut ocr = LepTess::new(None, "eng").expect("Failed to initialize Tesseract");

    loop {
        let start = Instant::now();

        // Capture Screenshot
        let image = screen.capture().expect("Failed to capture screenshot");

        // Access pixel data directly
        let pixel_data = image.as_raw(); // Correctly access raw pixel data

        // Create ImageBuffer from raw pixel data
        let img_buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(
            image.width(),
            image.height(),
            pixel_data.clone(),
        ).expect("Failed to create ImageBuffer from raw data");

        // Convert ImageBuffer to DynamicImage
        let dynamic_img = DynamicImage::ImageRgba8(img_buffer);

        // Convert DynamicImage to PNG format in memory
        let mut png_data = Vec::new();
        {
            dynamic_img.write_to(&mut Cursor::new(&mut png_data), ImageFormat::Png)
                .expect("Failed to encode image to memory");
        }

        // Pass in-memory PNG image to Tesseract
        match ocr.set_image_from_mem(&png_data) {
            Ok(_) => match ocr.get_utf8_text() {
                Ok(text) => println!("Extracted Text: {}", text),
                Err(err) => eprintln!("OCR Error: {}", err),
            },
            Err(err) => eprintln!("Failed to set image for OCR: {}", err),
        }

        // Maintain consistent frame interval
        let elapsed = start.elapsed();
        if elapsed < interval {
            thread::sleep(interval - elapsed);
        }
    }
}
