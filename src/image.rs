use std::fs::File;
use std::path::Path;
use std::io::BufWriter;
use std::ops::{AddAssign, DivAssign};

#[derive( Clone, Copy )]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

pub struct ColorSink {
    width: u32,
    height: u32,
    data: Vec<Color>
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl DivAssign for Color {
    fn div_assign(&mut self, other: Self) {
        self.r /= other.r;
        self.g /= other.g;
        self.b /= other.b;
    }
}

impl ColorSink {
    pub fn get_width(&self) -> u32 {
        return self.width;
    }

    pub fn get_height(&self) -> u32 {
        return self.height;
    }
}

impl ColorSink {
    pub fn new(width: u32, height: u32) -> Self {
        if width == 0 || height == 0 {
            panic!("Width and height must be greater than 0.");
        }

        let data = vec![Color::new(0, 0, 0, 255); (width * height) as usize];
        Self { width, height, data }
    }

    pub fn get_pixel(&mut self, x: u32, y: u32 ) -> Color {
        if x >= self.width || y >= self.height {
            panic!("Pixel out of bounds.");
        }

        self.data[(y * self.width + x) as usize]
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x >= self.width || y >= self.height {
            panic!("Pixel out of bounds.");
        }

        self.data[(y * self.width + x) as usize] = color;
    }

    pub fn set_block(&mut self, i: u32, data: Box<[Color]>) {
        if i >= self.width * self.height {
            panic!("Block out of bounds.");
        }

        let start = i;
        let end = i + data.len() as u32;
        self.data[start as usize..end as usize].clone_from_slice(&data);
    }

    pub fn get_data(&mut self) -> &mut Vec<Color> {
        self.data.as_mut()
    }
}

pub fn write_png_image( in_data: ColorSink, path: &str ) {
    let path = Path::new(path);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, in_data.width, in_data.height ); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut im_data = vec![ 0; (in_data.width * in_data.height * 3) as usize ].into_boxed_slice();
    for i in 0..(in_data.width * in_data.height) as usize {
        im_data[i * 3 + 0] = in_data.data[i].r;
        im_data[i * 3 + 1] = in_data.data[i].g;
        im_data[i * 3 + 2] = in_data.data[i].b;
    }

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data( &im_data ).unwrap(); // Save
}

pub fn read_png_image( path: &str ) -> ColorSink {
    let path = Path::new(path);
    let file = File::open(path).unwrap();
    let decoder = png::Decoder::new(file);
    let mut reader = decoder.read_info().unwrap();

    if( reader.info().color_type != png::ColorType::Rgb ) {
        panic!("Image must be RGB");
    }

    let width = reader.info().width;
    let height = reader.info().height;

    let mut data = vec![ 0 as u8; (width * height * 3) as usize ].into_boxed_slice();
    reader.next_frame( &mut data ).unwrap();

    let mut color_data = vec![ Color::new(0, 0, 0, 255); (width * height) as usize ];
    for i  in 0..(width * height) as usize {
        color_data[i].r = data[i * 3 + 0] as u8;
        color_data[i].g = data[i * 3 + 1] as u8;
        color_data[i].b = data[i * 3 + 2] as u8;
        color_data[i].a = data[i * 3 + 3] as u8;
    }

    ColorSink {
        width,
        height,
        data: color_data
    }
}