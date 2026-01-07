use image::GenericImageView;
use std::fs;
use std::path::Path;

pub fn migrate_to_webp<P: AsRef<Path>>(
    dir_path: P,
    max_size: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let dir = dir_path.as_ref();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() || path.extension().and_then(|s| s.to_str()) == Some("webp") {
            continue;
        }

        println!("Processing: {:?}", path.file_name().unwrap());

        // 1. Load and resize
        let img = image::open(&path)?;
        let resized = img.thumbnail(max_size, max_size);

        let (width, height) = &resized.dimensions();
        let rgb_resized = resized.to_rgba8();
        let raw_resized = rgb_resized.as_raw();
        let encoder = webp::Encoder::from_rgba(raw_resized, *width, *height);

        // 2. Encode to WebP
        let webp_data = encoder.encode(85.0); // 85 is high quality but small size

        // 3. Create new filename (e.g., photo.jpg -> photo.webp)
        let mut new_path = path.clone();
        new_path.set_extension("webp");

        // 4. Save the new file
        fs::write(&new_path, &*webp_data)?;

        // 5. DELETE the original file
        fs::remove_file(&path)?;

        println!(
            "Successfully replaced with {:?}",
            new_path.file_name().unwrap()
        );
    }

    Ok(())
}
