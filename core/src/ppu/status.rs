use bitflags::bitflags;

bitflags!(
    pub struct Status: u8 {
        const lyc_select = 1 << 6;
        const mode_2_int_select = 1 << 5;
        const mode_1_int_select = 1 << 4;
        const mode_0_int_select = 1 << 3;
    }
);