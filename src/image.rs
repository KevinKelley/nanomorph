fn unpremultiply_alpha(image: &mut [u8], w: u32, h: u32, stride: u32)
{
	let w: uint = w as uint; let h: uint = h as uint; let stride: uint = stride as uint;

	// Unpremultiply
	for y in range(0, h) {
		//unsigned char *row = &image[y*stride];
		let row = image.mut_slice(y*stride, y*stride + w*4);
		for x in range(0, w) {
			let pix = row.mut_slice(x*4, x*4 + 4);
			let r = pix[0] as f32;
			let g = pix[1] as f32;
			let b = pix[2] as f32;
			let a = pix[3] as f32;
			if a != 0.0 {
				pix[0] = min(r*255.0/a, 255.0) as u8;
				pix[1] = min(g*255.0/a, 255.0) as u8;
				pix[2] = min(b*255.0/a, 255.0) as u8;
			}
		}
	}

	// Defringe
	for y in range(0, h) {
		for x in range(0, w) {
			let ix = y*stride + x*4;
			let mut r = 0;
			let mut g = 0;
			let mut b = 0;
			let a = image[ix+3];
			let mut n = 0;
			if a == 0 {
				if x-1 > 0 && image[ix-1] != 0 {
					r += image[ix-4];
					g += image[ix-3];
					b += image[ix-2];
					n += 1;
				}
				if x+1 < w && image[ix+7] != 0 {
					r += image[ix+4];
					g += image[ix+5];
					b += image[ix+6];
					n += 1;
				}
				if y-1 > 0 && image[ix-stride+3] != 0 {
					r += image[ix-stride];
					g += image[ix-stride+1];
					b += image[ix-stride+2];
					n += 1;
				}
				if y+1 < h && image[ix+stride+3] != 0 {
					r += image[ix+stride];
					g += image[ix+stride+1];
					b += image[ix+stride+2];
					n += 1;
				}
				if n > 0 {
					image[ix+0] = r/n;
					image[ix+1] = g/n;
					image[ix+2] = b/n;
				}
			}
		}
	}
}

fn set_alpha(image: &mut [u8], w: u32, h: u32, stride: u32, a: u8)
{
	let w: uint = w as uint; let h: uint = h as uint; let stride: uint = stride as uint;
	for y in range(0, h) {
		let row = image.mut_slice(y*stride, y*stride + w*4); //&image[y*stride];
		for x in range(0, w) {
			row[x*4+3] = a;
		}
	}
}

fn flip_image(image: &mut [u8], w: u32, h: u32, stride: u32)
{
	let w: uint = w as uint; let h: uint = h as uint; let stride: uint = stride as uint;
	let mut i: uint = 0;
	let mut j: uint = h-1;
	while i < j {
		//let row_i = image.mut_slice(i*stride, i*stride + w*4); //&image[i * stride]; //unsigned char*
		//let row_j = image.mut_slice(j*stride, j*stride + w*4); //&image[j * stride]; //unsigned char*
		// error; can't borrow twice from the same source
		let ix: uint = i*stride;
		let jx: uint = j*stride;
		for k in range(0, w*4) {
			let t       = image[ix+k];  // let t = row_i[k];
			image[ix+k] = image[jx+k];  // row_i[k] = row_j[k];
			image[jx+k] = t;			// row_j[k] = t;
		}
		i += 1;
		j -= 1;
	}
}

pub fn save_screenshot(w: u32, h: u32, premult: bool, name: &str)
{
	let sz: uint = (w*h*4) as uint;
	//let mut image: [u8, ..sz] = [0, ..sz];
	let mut image: Vec<u8> = Vec::with_capacity(sz);
	unsafe {image.set_len(sz);}
	assert!(image.len() == sz);
	let addr: *mut u8 = &mut image.as_mut_slice()[0];
	let vptr: *mut c_void = addr as *mut c_void;
	unsafe {gl::ReadPixels(0, 0, w as i32, h as i32, gl::RGBA, gl::UNSIGNED_BYTE, addr as *mut c_void)};
	if premult {
		unpremultiply_alpha(image.as_mut_slice(), w, h, w*4);
	}
	else {
		set_alpha(image.as_mut_slice(), w, h, w*4, 255);
	}
	flip_image(image.as_mut_slice(), w, h, w*4);
 	write_png(name, w, h, 4, &image.as_slice()[0], w*4);
}
