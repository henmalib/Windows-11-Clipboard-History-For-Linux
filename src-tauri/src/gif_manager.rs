//! GIF Manager
//! Handles downloading GIFs and preparing them for clipboard paste

use arboard::{Clipboard, ImageData};
use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder, GenericImageView};
use std::io::Cursor;

/// Download a GIF from URL and extract the first frame as RGBA pixels
pub fn download_gif_as_image(url: &str) -> Result<(Vec<u8>, usize, usize), String> {
    // Download the GIF
    let response = reqwest::blocking::get(url)
        .map_err(|e| format!("Failed to download GIF: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()));
    }
    
    let bytes = response
        .bytes()
        .map_err(|e| format!("Failed to read response: {}", e))?;
    
    // Try to decode as GIF first
    let cursor = Cursor::new(&bytes);
    
    // Attempt GIF decoding to get the first frame
    if let Ok(decoder) = GifDecoder::new(cursor) {
        if let Some(frame_result) = decoder.into_frames().next() {
            if let Ok(frame) = frame_result {
                let buffer = frame.into_buffer();
                let (width, height) = buffer.dimensions();
                let rgba = buffer.into_raw();
                return Ok((rgba, width as usize, height as usize));
            }
        }
    }
    
    // Fallback: try to decode as a generic image (PNG, JPEG, etc.)
    let cursor = Cursor::new(&bytes);
    let img = image::load(cursor, image::ImageFormat::Gif)
        .or_else(|_| {
            // Try auto-detect format
            let cursor = Cursor::new(&bytes);
            image::load(cursor, image::ImageFormat::Png)
        })
        .or_else(|_| {
            let cursor = Cursor::new(&bytes);
            image::load(cursor, image::ImageFormat::Jpeg)
        })
        .map_err(|e| format!("Failed to decode image: {}", e))?;
    
    let (width, height) = img.dimensions();
    let rgba = img.to_rgba8().into_raw();
    
    Ok((rgba, width as usize, height as usize))
}

/// Copy an image (RGBA pixels) to the system clipboard
pub fn copy_image_to_clipboard(rgba: Vec<u8>, width: usize, height: usize) -> Result<(), String> {
    let mut clipboard = Clipboard::new()
        .map_err(|e| format!("Failed to open clipboard: {}", e))?;
    
    let image_data = ImageData {
        width,
        height,
        bytes: std::borrow::Cow::Owned(rgba),
    };
    
    clipboard
        .set_image(image_data)
        .map_err(|e| format!("Failed to set clipboard image: {}", e))?;
    
    Ok(())
}

/// Download a GIF and copy it to clipboard as an image (first frame)
pub fn paste_gif_to_clipboard(url: &str) -> Result<(), String> {
    eprintln!("[GifManager] Downloading GIF from: {}", url);
    
    let (rgba, width, height) = download_gif_as_image(url)?;
    
    eprintln!("[GifManager] Downloaded image: {}x{}, {} bytes", width, height, rgba.len());
    
    copy_image_to_clipboard(rgba, width, height)?;
    
    eprintln!("[GifManager] Copied to clipboard successfully");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_download_gif() {
        // This test requires network access
        // Skip in CI or if network is unavailable
        let test_url = "https://media.tenor.com/images/test.gif";
        // Just verify the function exists and can be called
        let _ = download_gif_as_image(test_url);
    }
}
