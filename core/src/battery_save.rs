pub trait BatterySave {
    fn load(&self, ram: &mut [u8]);
    fn save(&self, data: &[u8]);
}
