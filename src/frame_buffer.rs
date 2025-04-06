pub trait FrameBuffer {
    fn write_to_rbga_buffer(&self, rgba_buffer: &mut [u8]);
}

impl FrameBuffer for Vec<u8> {
    fn write_to_rbga_buffer(&self, rgba_buffer: &mut [u8]) {
        for (i, byte) in self.iter().enumerate() {
            let pixel_index = i * 4;
            let color = if cfg!(feature = "green") {
                match *byte {
                    0 => [0x9b, 0xbc, 0x0f, 0xff], // Lysest grønn
                    1 => [0x8b, 0xac, 0x0f, 0xff], // Lys grønn
                    2 => [0x30, 0x62, 0x30, 0xff], // Mørk grønn
                    _ => [0x0f, 0x38, 0x0f, 0xff], // Mørkest grønn
                }
            } else {
                match *byte {
                    0 => [0xff, 0xff, 0xff, 0xff], // Hvit
                    1 => [0xaa, 0xaa, 0xaa, 0xff], // Lys grå
                    2 => [0x55, 0x55, 0x55, 0xff], // Mørk grå
                    _ => [0x00, 0x00, 0x00, 0xff], // Svart
                }
            } ;
            rgba_buffer[pixel_index..pixel_index + 4].copy_from_slice(&color);
        }
    }
}