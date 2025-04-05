use glam::{UVec2};
use image::{ImageBuffer};

fn main() {
	// Create Image
	let dimensions: UVec2 = UVec2::new(800, 600);
	let mut buffer: ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::new(dimensions.x, dimensions.y);

	// Write a gradient
	for y in (0..dimensions.y).rev() {
		for x in 0..dimensions.x {
			let r: f32 = x as f32 / dimensions.x as f32;
			let g: f32 = y as f32 / dimensions.y as f32;
			let b: f32 = 0.0;

			let ir: u8 = (255.99 * r) as u8;
			let ig: u8 = (255.99 * g) as u8;
			let ib: u8 = (255.99 * b) as u8;

			let pixel = buffer.get_pixel_mut(x, y);
			*pixel = image::Rgb([ir, ig, ib]);
		}
	}

	buffer.save("output.png").unwrap();
}