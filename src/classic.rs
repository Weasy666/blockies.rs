use std::io;
use hsl::HSL;
use pixelate::{Color, Image, Error};

use crate::util::{create_image_data, hsl_to_rgb};

/// Context struct for creating an classic 2-color Blockies.
///
/// The best way to create it is by using the default trait:
///
/// ```rust
/// use blockies::Classic;
///
/// let mut gen = Classic::default();
/// let mut png = Vec::new();
///
/// gen.scale = 8;
/// gen.create_icon(&mut png, b"0x0000000000000000000000000000000000000000");
///
/// // `png` contains the PNG image of the blockies.
/// assert!(png.len() > 0);
/// ```
pub struct Classic {
	/// Size of blockies (number of blocks per row in the image), default: 8
	pub size: usize,
	/// Pixel size (width and height) of a single block in the image, default: 16
	pub scale: usize,
	/// Foreground color of the image, default: None (derived from seed)
	pub color: Option<Color>,
	/// Background color of the image, default: None (white)
	pub background_color: Option<Color>,
}

impl Classic {
	/// Write the PNG image of the blockies for a given `seed` into a writer.
	pub fn create_icon<W: io::Write>(&self, writer: W, seed: &[u8]) -> Result<(), Error> {
		let mut seed = Seed::new(seed);

		let color = self.color.unwrap_or_else(|| seed.create_color());
		let background_color = self.background_color.unwrap_or_else(|| pixelate::WHITE);

		let palette = vec![background_color, color];
		let pixels = create_image_data(self.size as usize, || seed.rand() >= 0.5);

		Image {
			palette: &palette,
			pixels: &pixels,
			width: self.size + self.size % 2,
			scale: self.scale,
		}.render(writer)
	}
}

impl Default for Classic {
	fn default() -> Self {
		Classic {
			size: 8,
			scale: 16,
			color: None,
			background_color: None,
		}
	}
}

pub struct Seed {
	randseed: f64,
}

impl Seed {
	fn new(seed: &[u8]) -> Self {
		let mut randseed = 0u64;

		for i in 0..seed.len() / 2 {
			let h = ((seed[i * 2] as u64) << 8) | seed[i * 2 + 1] as u64;
			randseed = randseed ^ h;
		}

		if seed.len() % 2 == 1 {
			randseed = randseed ^ ((seed[seed.len() - 1] as u64) << 8);
		}

		Seed {
			randseed: randseed as f64,
		}
	}

	fn rand(&mut self) -> f64 {
		let n = (self.randseed.sin() + 1.0) / 2.0;
		self.randseed += 1.0;
		let r = n * 10000.0;
		r - r.floor()
	}

	fn create_color(&mut self) -> Color {
		let hsl = HSL {
			h: (self.rand() * 360.0).floor(),
			s: (self.rand() * 50.0 + 50.0) / 100.0,
			l: (self.rand() * 60.0 + 20.0) / 100.0,
		};
		hsl_to_rgb(hsl)
	}
}
