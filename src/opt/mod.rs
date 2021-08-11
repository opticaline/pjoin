pub mod join {
	use image::DynamicImage;
	use image::GenericImage;
	use image::GenericImageView;
	use std::vec;

	pub fn try_join(img1: DynamicImage, img2: DynamicImage) -> Option<image::DynamicImage> {
		if let Some((_, y2)) = find_merge(&img1, &img2) {
			return Some(join(&img1, &img2, y2));
		}
		return None;
	}

	fn join(img1: &DynamicImage, img2: &DynamicImage, skip: u32) -> DynamicImage {
		let mut imgbuf =
			image::DynamicImage::new_rgba8(img1.width(), img1.height() + img2.height() - skip);

		for h in 0..img1.height() {
			for w in 0..img1.width() {
				imgbuf.put_pixel(w, h, img1.get_pixel(w, h));
			}
		}
		for h in skip..img2.height() {
			for w in 0..img2.width() {
				imgbuf.put_pixel(w, img1.height() + h - skip, img2.get_pixel(w, h));
			}
		}
		return imgbuf;
	}

	fn find_merge(img1: &DynamicImage, img2: &DynamicImage) -> Option<(u32, u32)> {
		let mut start = 0;
		loop {
			for y in (0..img1.height()).rev() {
				let line1 = get_line(&img1, y);
				if let Some(y2) = search_same_line(&img2, start, img2.height(), &line1) {
					if same_block(&img2, y2, &img1, y) {
						return Some((y, y2));
					} else {
						start = y2 + 1;
						break;
					}
				} else {
					return None;
				}
			}
		}
	}

	fn same_block<'a>(img: &'a DynamicImage, end: u32, img2: &'a DynamicImage, y: u32) -> bool {
		for i in (0..end).rev() {
			if !is_equals(&get_line(&img2, y - end + i), &get_line(&img, i)) {
				return false;
			}
		}
		true
	}

	fn search_same_line<'a>(
		img: &'a DynamicImage,
		start: u32,
		end: u32,
		line: &'a Vec<image::Rgba<u8>>,
	) -> Option<u32> {
		for y in start..end {
			let l = get_line(&img, y);
			if is_equals(&l, &line) {
				return Some(y);
			}
		}
		None
	}

	fn get_line<'a>(img1: &'a DynamicImage, y: u32) -> Vec<image::Rgba<u8>> {
		let mut line1 = vec![];
		for x in 0..img1.width() {
			line1.push(img1.get_pixel(x, y));
		}
		return line1;
	}

	fn is_equals(line1: &Vec<image::Rgba<u8>>, line2: &Vec<image::Rgba<u8>>) -> bool {
		for i in 0..line1.len() {
			if !point_is_equals(line1.get(i).unwrap(), line2.get(i).unwrap()) {
				return false;
			}
		}
		return true;
	}
	fn point_is_equals(p1: &image::Rgba<u8>, p2: &image::Rgba<u8>) -> bool {
		let critical = 13;
		// println!("{:?}", (p1[0] as i32 - p2[0] as i32).abs());
		return (p1[0] as i32 - p2[0] as i32).abs() < critical
			&& (p1[1] as i32 - p2[1] as i32).abs() < critical
			&& (p1[2] as i32 - p2[2] as i32).abs() < critical;
	}
}
