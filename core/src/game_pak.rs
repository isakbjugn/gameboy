pub trait GamePak {
    fn read_rom(&self) -> Vec<u8>;
    fn read_header(&self) -> Vec<u8> {
        let mut header = vec![0; 0x14f - 0x100 + 1];
        header.copy_from_slice(&self.read_rom()[0x0100..=0x014f]);
        header
    }
    fn title(&self) -> String {
        const TITLE_START: usize = 0x0134 - 0x0100;
        const TITLE_END: usize = 0x0143 - 0x0100;

        String::from_utf8(self.read_header()[TITLE_START..=TITLE_END].to_owned()).unwrap()
    }
}
