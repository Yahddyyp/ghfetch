use crate::config::Layout;
use anyhow::Result;
use base64::Engine;
use std::io::{self, Write};

// Size of base64 data to send to kitty's image protocol
const KITTY_CHUNK_SIZE: usize = 4096;

/// Take the url, download the image, cache the image in mermory and covert it to png,
/// base64 encodes it then send it to the terminal.
pub async fn get_image(url: &str, image_id: u32, layout: &Layout) -> Result<()> {
    let response = reqwest::get(url).await?.error_for_status()?;

    let bytes = response.bytes().await?;

    // Decode whatever format it is, then re-encode as PNG
    let img = image::load_from_memory(&bytes)?;
    let mut png_bytes: Vec<u8> = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut png_bytes),
        image::ImageFormat::Png,
    )?;

    // Encode to base64
    let encoded = base64::engine::general_purpose::STANDARD.encode(&png_bytes);

    // Contains the base64 data that is generated but has not been sent to the
    // terminal
    let mut encoded_buffer: Vec<u8> = encoded.into_bytes();

    // Kitty's protocol needs a=t on every chunk, but f=100 only on
    // the first one, this tracks whether we're still on that chunk
    let mut first = true;

    // Check if there is enough for kitty's image protocol
    while encoded_buffer.len() >= KITTY_CHUNK_SIZE {
        // Remove the first KITTY_CHUNK_SIZE bytes from the buffer and put them into the chunk
        let kitty_chunk: Vec<u8> = encoded_buffer.drain(..KITTY_CHUNK_SIZE).collect();

        // Send it to kitty
        send_escape_sequence(&kitty_chunk, first, true, image_id)?;

        // Now first transmission is done
        first = false;
    }

    // Send whatever's left
    if !encoded_buffer.is_empty() {
        send_escape_sequence(&encoded_buffer, first, false, image_id)?;
    } else if first {
        // The whole image fit in fewer bytes than ever triggered a
        // chunk send, make sure at least one transmission happens
        send_escape_sequence(&[], first, false, image_id)?;
    }

    // Put the image correctly
    println!();
    print!("{}", " ".repeat(layout.left_gap));
    print!(
        "\x1b_Ga=p,i={},c={},r={},q=1;\x1b\\",
        image_id, layout.image_columns, layout.image_rows
    );

    // Send everything buffered in stdout
    io::stdout().flush()?;

    Ok(())
}

/// Take data in base64 bytes and check it is the first and whether more bytes are
/// coming. &[u8] instead of Vec<u8> as it just needs to read the data not own it.
pub fn send_escape_sequence(data: &[u8], first: bool, more: bool, image_id: u32) -> Result<()> {
    // If there is more chunks coming 1 else 0
    let more = if more { 1 } else { 0 };

    // If this is the first chunk f=100 else no
    if first {
        print!(
            "\x1b_Ga=t,q=1,f=100,i={},m={};{}\x1b\\",
            image_id,
            more,
            std::str::from_utf8(data)?
        );
    } else {
        print!(
            "\x1b_Ga=t,q=1,i={},m={};{}\x1b\\",
            image_id,
            more,
            std::str::from_utf8(data)?
        );
    }

    Ok(())
}
