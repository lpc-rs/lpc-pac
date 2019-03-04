#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Digital I/O control for pins PIO0_17"]
    pub pio0_17: PIO0_17,
    #[doc = "0x04 - Digital I/O control for pins PIO0_13"]
    pub pio0_13: PIO0_13,
    #[doc = "0x08 - Digital I/O control for pins PIO0_12"]
    pub pio0_12: PIO0_12,
    #[doc = "0x0c - Digital I/O control for pins PIO0_5"]
    pub pio0_5: PIO0_5,
    #[doc = "0x10 - Digital I/O control for pins PIO0_4"]
    pub pio0_4: PIO0_4,
    #[doc = "0x14 - Digital I/O control for pins PIO0_3"]
    pub pio0_3: PIO0_3,
    #[doc = "0x18 - Digital I/O control for pins PIO0_2"]
    pub pio0_2: PIO0_2,
    #[doc = "0x1c - Digital I/O control for pins PIO0_11"]
    pub pio0_11: PIO0_11,
    #[doc = "0x20 - Digital I/O control for pins PIO0_10"]
    pub pio0_10: PIO0_10,
    #[doc = "0x24 - Digital I/O control for pins PIO0_16"]
    pub pio0_16: PIO0_16,
    #[doc = "0x28 - Digital I/O control for pins PIO0_15"]
    pub pio0_15: PIO0_15,
    #[doc = "0x2c - Digital I/O control for pins PIO0_1"]
    pub pio0_1: PIO0_1,
    _reserved0: [u8; 4usize],
    #[doc = "0x34 - Digital I/O control for pins PIO0_9"]
    pub pio0_9: PIO0_9,
    #[doc = "0x38 - Digital I/O control for pins PIO0_8"]
    pub pio0_8: PIO0_8,
    #[doc = "0x3c - Digital I/O control for pins PIO0_7"]
    pub pio0_7: PIO0_7,
    #[doc = "0x40 - Digital I/O control for pins PIO0_6"]
    pub pio0_6: PIO0_6,
    #[doc = "0x44 - Digital I/O control for pins PIO0_0"]
    pub pio0_0: PIO0_0,
    #[doc = "0x48 - Digital I/O control for pins PIO0_14"]
    pub pio0_14: PIO0_14,
    _reserved1: [u8; 4usize],
    #[doc = "0x50 - Digital I/O control for pins PIO0_28"]
    pub pio0_28: PIO0_28,
    #[doc = "0x54 - Digital I/O control for pins PIO0_27"]
    pub pio0_27: PIO0_27,
    #[doc = "0x58 - Digital I/O control for pins PIO0_26"]
    pub pio0_26: PIO0_26,
    #[doc = "0x5c - Digital I/O control for pins PIO0_25"]
    pub pio0_25: PIO0_25,
    #[doc = "0x60 - Digital I/O control for pins PIO0_24"]
    pub pio0_24: PIO0_24,
    #[doc = "0x64 - Digital I/O control for pins PIO0_23"]
    pub pio0_23: PIO0_23,
    #[doc = "0x68 - Digital I/O control for pins PIO0_22"]
    pub pio0_22: PIO0_22,
    #[doc = "0x6c - Digital I/O control for pins PIO0_21"]
    pub pio0_21: PIO0_21,
    #[doc = "0x70 - Digital I/O control for pins PIO0_20"]
    pub pio0_20: PIO0_20,
    #[doc = "0x74 - Digital I/O control for pins PIO0_19"]
    pub pio0_19: PIO0_19,
    #[doc = "0x78 - Digital I/O control for pins PIO0_18"]
    pub pio0_18: PIO0_18,
    #[doc = "0x7c - Digital I/O control for pins PIO1_8"]
    pub pio1_8: PIO1_8,
    #[doc = "0x80 - Digital I/O control for pins PIO1_9"]
    pub pio1_9: PIO1_9,
    #[doc = "0x84 - Digital I/O control for pins PIO1_12"]
    pub pio1_12: PIO1_12,
    #[doc = "0x88 - Digital I/O control for pins PIO1_13"]
    pub pio1_13: PIO1_13,
    #[doc = "0x8c - Digital I/O control for pins PIO0_31"]
    pub pio0_31: PIO0_31,
    #[doc = "0x90 - Digital I/O control for pins PIO1_0"]
    pub pio1_0: PIO1_0,
    #[doc = "0x94 - Digital I/O control for pins PIO1_1"]
    pub pio1_1: PIO1_1,
    #[doc = "0x98 - Digital I/O control for pins PIO1_2"]
    pub pio1_2: PIO1_2,
    #[doc = "0x9c - Digital I/O control for pins PIO1_14"]
    pub pio1_14: PIO1_14,
    #[doc = "0xa0 - Digital I/O control for pins PIO1_15"]
    pub pio1_15: PIO1_15,
    #[doc = "0xa4 - Digital I/O control for pins PIO1_3"]
    pub pio1_3: PIO1_3,
    #[doc = "0xa8 - Digital I/O control for pins PIO1_4"]
    pub pio1_4: PIO1_4,
    #[doc = "0xac - Digital I/O control for pins PIO1_5"]
    pub pio1_5: PIO1_5,
    #[doc = "0xb0 - Digital I/O control for pins PIO1_16"]
    pub pio1_16: PIO1_16,
    #[doc = "0xb4 - Digital I/O control for pins PIO1_17"]
    pub pio1_17: PIO1_17,
    #[doc = "0xb8 - Digital I/O control for pins PIO1_6"]
    pub pio1_6: PIO1_6,
    #[doc = "0xbc - Digital I/O control for pins PIO1_18"]
    pub pio1_18: PIO1_18,
    #[doc = "0xc0 - Digital I/O control for pins PIO1_19"]
    pub pio1_19: PIO1_19,
    #[doc = "0xc4 - Digital I/O control for pins PIO1_7"]
    pub pio1_7: PIO1_7,
    #[doc = "0xc8 - Digital I/O control for pins PIO0_29"]
    pub pio0_29: PIO0_29,
    #[doc = "0xcc - Digital I/O control for pins PIO0_30"]
    pub pio0_30: PIO0_30,
    #[doc = "0xd0 - Digital I/O control for pins PIO1_20"]
    pub pio1_20: PIO1_20,
    #[doc = "0xd4 - Digital I/O control for pins PIO1_21"]
    pub pio1_21: PIO1_21,
    #[doc = "0xd8 - Digital I/O control for pins PIO1_11"]
    pub pio1_11: PIO1_11,
    #[doc = "0xdc - Digital I/O control for pins PIO1_10"]
    pub pio1_10: PIO1_10,
}
#[doc = "Digital I/O control for pins PIO0_17"]
pub struct PIO0_17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_17"]
pub mod pio0_17;
#[doc = "Digital I/O control for pins PIO0_13"]
pub struct PIO0_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_13"]
pub mod pio0_13;
#[doc = "Digital I/O control for pins PIO0_12"]
pub struct PIO0_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_12"]
pub mod pio0_12;
#[doc = "Digital I/O control for pins PIO0_5"]
pub struct PIO0_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_5"]
pub mod pio0_5;
#[doc = "Digital I/O control for pins PIO0_4"]
pub struct PIO0_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_4"]
pub mod pio0_4;
#[doc = "Digital I/O control for pins PIO0_3"]
pub struct PIO0_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_3"]
pub mod pio0_3;
#[doc = "Digital I/O control for pins PIO0_2"]
pub struct PIO0_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_2"]
pub mod pio0_2;
#[doc = "Digital I/O control for pins PIO0_11"]
pub struct PIO0_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_11"]
pub mod pio0_11;
#[doc = "Digital I/O control for pins PIO0_10"]
pub struct PIO0_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_10"]
pub mod pio0_10;
#[doc = "Digital I/O control for pins PIO0_16"]
pub struct PIO0_16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_16"]
pub mod pio0_16;
#[doc = "Digital I/O control for pins PIO0_15"]
pub struct PIO0_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_15"]
pub mod pio0_15;
#[doc = "Digital I/O control for pins PIO0_1"]
pub struct PIO0_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_1"]
pub mod pio0_1;
#[doc = "Digital I/O control for pins PIO0_9"]
pub struct PIO0_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_9"]
pub mod pio0_9;
#[doc = "Digital I/O control for pins PIO0_8"]
pub struct PIO0_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_8"]
pub mod pio0_8;
#[doc = "Digital I/O control for pins PIO0_7"]
pub struct PIO0_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_7"]
pub mod pio0_7;
#[doc = "Digital I/O control for pins PIO0_6"]
pub struct PIO0_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_6"]
pub mod pio0_6;
#[doc = "Digital I/O control for pins PIO0_0"]
pub struct PIO0_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_0"]
pub mod pio0_0;
#[doc = "Digital I/O control for pins PIO0_14"]
pub struct PIO0_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_14"]
pub mod pio0_14;
#[doc = "Digital I/O control for pins PIO0_28"]
pub struct PIO0_28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_28"]
pub mod pio0_28;
#[doc = "Digital I/O control for pins PIO0_27"]
pub struct PIO0_27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_27"]
pub mod pio0_27;
#[doc = "Digital I/O control for pins PIO0_26"]
pub struct PIO0_26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_26"]
pub mod pio0_26;
#[doc = "Digital I/O control for pins PIO0_25"]
pub struct PIO0_25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_25"]
pub mod pio0_25;
#[doc = "Digital I/O control for pins PIO0_24"]
pub struct PIO0_24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_24"]
pub mod pio0_24;
#[doc = "Digital I/O control for pins PIO0_23"]
pub struct PIO0_23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_23"]
pub mod pio0_23;
#[doc = "Digital I/O control for pins PIO0_22"]
pub struct PIO0_22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_22"]
pub mod pio0_22;
#[doc = "Digital I/O control for pins PIO0_21"]
pub struct PIO0_21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_21"]
pub mod pio0_21;
#[doc = "Digital I/O control for pins PIO0_20"]
pub struct PIO0_20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_20"]
pub mod pio0_20;
#[doc = "Digital I/O control for pins PIO0_19"]
pub struct PIO0_19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_19"]
pub mod pio0_19;
#[doc = "Digital I/O control for pins PIO0_18"]
pub struct PIO0_18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_18"]
pub mod pio0_18;
#[doc = "Digital I/O control for pins PIO1_8"]
pub struct PIO1_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO1_8"]
pub mod pio1_8;
#[doc = "Digital I/O control for pins PIO1_9"]
pub struct PIO1_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO1_9"]
pub mod pio1_9;
#[doc = "Digital I/O control for pins PIO1_12"]
pub struct PIO1_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO1_12"]
pub mod pio1_12;
#[doc = "Digital I/O control for pins PIO1_13"]
pub struct PIO1_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO1_13"]
pub mod pio1_13;
#[doc = "Digital I/O control for pins PIO0_31"]
pub struct PIO0_31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_31"]
pub mod pio0_31;
#[doc = "Digital I/O control for pins PIO1_0"]
pub struct PIO1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO1_0"]
pub mod pio1_0;
#[doc = "Digital I/O control for pins PIO1_1"]
pub struct PIO1_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO1_1"]
pub mod pio1_1;
#[doc = "Digital I/O control for pins PIO1_2"]
pub struct PIO1_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO1_2"]
pub mod pio1_2;
#[doc = "Digital I/O control for pins PIO1_14"]
pub struct PIO1_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO1_14"]
pub mod pio1_14;
#[doc = "Digital I/O control for pins PIO1_15"]
pub struct PIO1_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO1_15"]
pub mod pio1_15;
#[doc = "Digital I/O control for pins PIO1_3"]
pub struct PIO1_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO1_3"]
pub mod pio1_3;
#[doc = "Digital I/O control for pins PIO1_4"]
pub struct PIO1_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO1_4"]
pub mod pio1_4;
#[doc = "Digital I/O control for pins PIO1_5"]
pub struct PIO1_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO1_5"]
pub mod pio1_5;
#[doc = "Digital I/O control for pins PIO1_16"]
pub struct PIO1_16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO1_16"]
pub mod pio1_16;
#[doc = "Digital I/O control for pins PIO1_17"]
pub struct PIO1_17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO1_17"]
pub mod pio1_17;
#[doc = "Digital I/O control for pins PIO1_6"]
pub struct PIO1_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO1_6"]
pub mod pio1_6;
#[doc = "Digital I/O control for pins PIO1_18"]
pub struct PIO1_18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO1_18"]
pub mod pio1_18;
#[doc = "Digital I/O control for pins PIO1_19"]
pub struct PIO1_19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO1_19"]
pub mod pio1_19;
#[doc = "Digital I/O control for pins PIO1_7"]
pub struct PIO1_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO1_7"]
pub mod pio1_7;
#[doc = "Digital I/O control for pins PIO0_29"]
pub struct PIO0_29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_29"]
pub mod pio0_29;
#[doc = "Digital I/O control for pins PIO0_30"]
pub struct PIO0_30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO0_30"]
pub mod pio0_30;
#[doc = "Digital I/O control for pins PIO1_20"]
pub struct PIO1_20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO1_20"]
pub mod pio1_20;
#[doc = "Digital I/O control for pins PIO1_21"]
pub struct PIO1_21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO1_21"]
pub mod pio1_21;
#[doc = "Digital I/O control for pins PIO1_11"]
pub struct PIO1_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO1_11"]
pub mod pio1_11;
#[doc = "Digital I/O control for pins PIO1_10"]
pub struct PIO1_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for pins PIO1_10"]
pub mod pio1_10;
