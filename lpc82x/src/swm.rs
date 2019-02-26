#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin assign register 0. Assign movable functions U0_TXD, U0_RXD, U0_RTS, U0_CTS."]
    pub pinassign0: PINASSIGN0,
    #[doc = "0x04 - Pin assign register 1. Assign movable functions U0_SCLK, U1_TXD, U1_RXD, U1_RTS."]
    pub pinassign1: PINASSIGN1,
    #[doc = "0x08 - Pin assign register 2. Assign movable functions U1_CTS, U1_SCLK, U2_TXD, U2_RXD."]
    pub pinassign2: PINASSIGN2,
    #[doc = "0x0c - Pin assign register 3. Assign movable function U2_RTS, U2_CTS, U2_SCLK, SPI0_SCK."]
    pub pinassign3: PINASSIGN3,
    #[doc = "0x10 - Pin assign register 4. Assign movable functions SPI0_MOSI, SPI0_MISO, SPI0_SSEL0, SPI0_SSEL1."]
    pub pinassign4: PINASSIGN4,
    #[doc = "0x14 - Pin assign register 5. Assign movable functions SPI0_SSEL2, SPI0_SSEL3, SPI1_SCK, SPI1_MOSI"]
    pub pinassign5: PINASSIGN5,
    #[doc = "0x18 - Pin assign register 6. Assign movable functions SPI1_MISO, SPI1_SSEL0, SPI1_SSEL1, SCT0_IN0."]
    pub pinassign6: PINASSIGN6,
    #[doc = "0x1c - Pin assign register 7. Assign movable functions SCT_IN1, SCT_IN2, SCT_IN3, SCT_OUT0."]
    pub pinassign7: PINASSIGN7,
    #[doc = "0x20 - Pin assign register 8. Assign movable functions SCT_OUT1, SCT_OUT2, SCT_OUT3, SCT_OUT4."]
    pub pinassign8: PINASSIGN8,
    #[doc = "0x24 - Pin assign register 9. Assign movable functions SCT_OUT5, I2C1_SDA, I2C1_SCL, I2C2_SDA."]
    pub pinassign9: PINASSIGN9,
    #[doc = "0x28 - Pin assign register 10. Assign movable functions I2C2_SCL, I2C3_SDA, I2C3_SCL, ADC_PINTRIG0."]
    pub pinassign10: PINASSIGN10,
    #[doc = "0x2c - Pin assign register 11. Assign movable functions ADC_PINTRIG1, ACMP_O, CLKOUT, GPIO_INT_BMAT"]
    pub pinassign11: PINASSIGN11,
    _reserved0: [u8; 400usize],
    #[doc = "0x1c0 - Pin enable register 0. Enables fixed-pin functions ACMP_I0, ACMP_I1, SWCLK, SWDIO, XTALIN, XTALOUT, RESET, CLKIN, VDDCMP."]
    pub pinenable0: PINENABLE0,
}
#[doc = "Pin assign register 0. Assign movable functions U0_TXD, U0_RXD, U0_RTS, U0_CTS."]
pub struct PINASSIGN0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 0. Assign movable functions U0_TXD, U0_RXD, U0_RTS, U0_CTS."]
pub mod pinassign0;
#[doc = "Pin assign register 1. Assign movable functions U0_SCLK, U1_TXD, U1_RXD, U1_RTS."]
pub struct PINASSIGN1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 1. Assign movable functions U0_SCLK, U1_TXD, U1_RXD, U1_RTS."]
pub mod pinassign1;
#[doc = "Pin assign register 2. Assign movable functions U1_CTS, U1_SCLK, U2_TXD, U2_RXD."]
pub struct PINASSIGN2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 2. Assign movable functions U1_CTS, U1_SCLK, U2_TXD, U2_RXD."]
pub mod pinassign2;
#[doc = "Pin assign register 3. Assign movable function U2_RTS, U2_CTS, U2_SCLK, SPI0_SCK."]
pub struct PINASSIGN3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 3. Assign movable function U2_RTS, U2_CTS, U2_SCLK, SPI0_SCK."]
pub mod pinassign3;
#[doc = "Pin assign register 4. Assign movable functions SPI0_MOSI, SPI0_MISO, SPI0_SSEL0, SPI0_SSEL1."]
pub struct PINASSIGN4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 4. Assign movable functions SPI0_MOSI, SPI0_MISO, SPI0_SSEL0, SPI0_SSEL1."]
pub mod pinassign4;
#[doc = "Pin assign register 5. Assign movable functions SPI0_SSEL2, SPI0_SSEL3, SPI1_SCK, SPI1_MOSI"]
pub struct PINASSIGN5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 5. Assign movable functions SPI0_SSEL2, SPI0_SSEL3, SPI1_SCK, SPI1_MOSI"]
pub mod pinassign5;
#[doc = "Pin assign register 6. Assign movable functions SPI1_MISO, SPI1_SSEL0, SPI1_SSEL1, SCT0_IN0."]
pub struct PINASSIGN6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 6. Assign movable functions SPI1_MISO, SPI1_SSEL0, SPI1_SSEL1, SCT0_IN0."]
pub mod pinassign6;
#[doc = "Pin assign register 7. Assign movable functions SCT_IN1, SCT_IN2, SCT_IN3, SCT_OUT0."]
pub struct PINASSIGN7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 7. Assign movable functions SCT_IN1, SCT_IN2, SCT_IN3, SCT_OUT0."]
pub mod pinassign7;
#[doc = "Pin assign register 8. Assign movable functions SCT_OUT1, SCT_OUT2, SCT_OUT3, SCT_OUT4."]
pub struct PINASSIGN8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 8. Assign movable functions SCT_OUT1, SCT_OUT2, SCT_OUT3, SCT_OUT4."]
pub mod pinassign8;
#[doc = "Pin assign register 9. Assign movable functions SCT_OUT5, I2C1_SDA, I2C1_SCL, I2C2_SDA."]
pub struct PINASSIGN9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 9. Assign movable functions SCT_OUT5, I2C1_SDA, I2C1_SCL, I2C2_SDA."]
pub mod pinassign9;
#[doc = "Pin assign register 10. Assign movable functions I2C2_SCL, I2C3_SDA, I2C3_SCL, ADC_PINTRIG0."]
pub struct PINASSIGN10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 10. Assign movable functions I2C2_SCL, I2C3_SDA, I2C3_SCL, ADC_PINTRIG0."]
pub mod pinassign10;
#[doc = "Pin assign register 11. Assign movable functions ADC_PINTRIG1, ACMP_O, CLKOUT, GPIO_INT_BMAT"]
pub struct PINASSIGN11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 11. Assign movable functions ADC_PINTRIG1, ACMP_O, CLKOUT, GPIO_INT_BMAT"]
pub mod pinassign11;
#[doc = "Pin enable register 0. Enables fixed-pin functions ACMP_I0, ACMP_I1, SWCLK, SWDIO, XTALIN, XTALOUT, RESET, CLKIN, VDDCMP."]
pub struct PINENABLE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin enable register 0. Enables fixed-pin functions ACMP_I0, ACMP_I1, SWCLK, SWDIO, XTALIN, XTALOUT, RESET, CLKIN, VDDCMP."]
pub mod pinenable0;
