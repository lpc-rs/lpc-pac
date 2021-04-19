#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin assign register 0. Assign movable functions U0_TXD, U0_RXD, U0_RTS, U0_CTS."]
    pub pinassign0: crate::Reg<pinassign0::PINASSIGN0_SPEC>,
    #[doc = "0x04 - Pin assign register 1. Assign movable functions U0_SCLK, U1_TXD, U1_RXD, U1_RTS."]
    pub pinassign1: crate::Reg<pinassign1::PINASSIGN1_SPEC>,
    #[doc = "0x08 - Pin assign register 2. Assign movable functions U1_CTS, U1_SCLK, U2_TXD, U2_RXD."]
    pub pinassign2: crate::Reg<pinassign2::PINASSIGN2_SPEC>,
    #[doc = "0x0c - Pin assign register 3. Assign movable function U2_RTS, U2_CTS, U2_SCLK, SPI0_SCK."]
    pub pinassign3: crate::Reg<pinassign3::PINASSIGN3_SPEC>,
    #[doc = "0x10 - Pin assign register 4. Assign movable functions SPI0_MOSI, SPI0_MISO, SPI0_SSEL0, SPI0_SSEL1."]
    pub pinassign4: crate::Reg<pinassign4::PINASSIGN4_SPEC>,
    #[doc = "0x14 - Pin assign register 5. Assign movable functions SPI0_SSEL2, SPI0_SSEL3, SPI1_SCK, SPI1_MOSI"]
    pub pinassign5: crate::Reg<pinassign5::PINASSIGN5_SPEC>,
    #[doc = "0x18 - Pin assign register 6. Assign movable functions SPI1_MISO, SPI1_SSEL0, SPI1_SSEL1, SCT0_IN0."]
    pub pinassign6: crate::Reg<pinassign6::PINASSIGN6_SPEC>,
    #[doc = "0x1c - Pin assign register 7. Assign movable functions SCT_IN1, SCT_IN2, SCT_IN3, SCT_OUT0."]
    pub pinassign7: crate::Reg<pinassign7::PINASSIGN7_SPEC>,
    #[doc = "0x20 - Pin assign register 8. Assign movable functions SCT_OUT1, SCT_OUT2, SCT_OUT3, SCT_OUT4."]
    pub pinassign8: crate::Reg<pinassign8::PINASSIGN8_SPEC>,
    #[doc = "0x24 - Pin assign register 9. Assign movable functions SCT_OUT5, I2C1_SDA, I2C1_SCL, I2C2_SDA."]
    pub pinassign9: crate::Reg<pinassign9::PINASSIGN9_SPEC>,
    #[doc = "0x28 - Pin assign register 10. Assign movable functions, I2C2_SCL, I2C3_SDA, I2C3_SCL, ADC_PINTRIG0."]
    pub pinassign10: crate::Reg<pinassign10::PINASSIGN10_SPEC>,
    #[doc = "0x2c - Pin assign register 11. Assign movable functions ADC_PINTRIG1, ACMP_O, CLKOUT, GPIO_INT_BMAT"]
    pub pinassign11: crate::Reg<pinassign11::PINASSIGN11_SPEC>,
    _reserved12: [u8; 400usize],
    #[doc = "0x1c0 - Pin enable register 0. Enables fixed-pin functions ACMP_I0, ACMP_I1, SWCLK, SWDIO, XTALIN, XTALOUT, RESET, CLKIN, VDDCMP and so on."]
    pub pinenable0: crate::Reg<pinenable0::PINENABLE0_SPEC>,
}
#[doc = "PINASSIGN0 register accessor: an alias for `Reg<PINASSIGN0_SPEC>`"]
pub type PINASSIGN0 = crate::Reg<pinassign0::PINASSIGN0_SPEC>;
#[doc = "Pin assign register 0. Assign movable functions U0_TXD, U0_RXD, U0_RTS, U0_CTS."]
pub mod pinassign0;
#[doc = "PINASSIGN1 register accessor: an alias for `Reg<PINASSIGN1_SPEC>`"]
pub type PINASSIGN1 = crate::Reg<pinassign1::PINASSIGN1_SPEC>;
#[doc = "Pin assign register 1. Assign movable functions U0_SCLK, U1_TXD, U1_RXD, U1_RTS."]
pub mod pinassign1;
#[doc = "PINASSIGN2 register accessor: an alias for `Reg<PINASSIGN2_SPEC>`"]
pub type PINASSIGN2 = crate::Reg<pinassign2::PINASSIGN2_SPEC>;
#[doc = "Pin assign register 2. Assign movable functions U1_CTS, U1_SCLK, U2_TXD, U2_RXD."]
pub mod pinassign2;
#[doc = "PINASSIGN3 register accessor: an alias for `Reg<PINASSIGN3_SPEC>`"]
pub type PINASSIGN3 = crate::Reg<pinassign3::PINASSIGN3_SPEC>;
#[doc = "Pin assign register 3. Assign movable function U2_RTS, U2_CTS, U2_SCLK, SPI0_SCK."]
pub mod pinassign3;
#[doc = "PINASSIGN4 register accessor: an alias for `Reg<PINASSIGN4_SPEC>`"]
pub type PINASSIGN4 = crate::Reg<pinassign4::PINASSIGN4_SPEC>;
#[doc = "Pin assign register 4. Assign movable functions SPI0_MOSI, SPI0_MISO, SPI0_SSEL0, SPI0_SSEL1."]
pub mod pinassign4;
#[doc = "PINASSIGN5 register accessor: an alias for `Reg<PINASSIGN5_SPEC>`"]
pub type PINASSIGN5 = crate::Reg<pinassign5::PINASSIGN5_SPEC>;
#[doc = "Pin assign register 5. Assign movable functions SPI0_SSEL2, SPI0_SSEL3, SPI1_SCK, SPI1_MOSI"]
pub mod pinassign5;
#[doc = "PINASSIGN6 register accessor: an alias for `Reg<PINASSIGN6_SPEC>`"]
pub type PINASSIGN6 = crate::Reg<pinassign6::PINASSIGN6_SPEC>;
#[doc = "Pin assign register 6. Assign movable functions SPI1_MISO, SPI1_SSEL0, SPI1_SSEL1, SCT0_IN0."]
pub mod pinassign6;
#[doc = "PINASSIGN7 register accessor: an alias for `Reg<PINASSIGN7_SPEC>`"]
pub type PINASSIGN7 = crate::Reg<pinassign7::PINASSIGN7_SPEC>;
#[doc = "Pin assign register 7. Assign movable functions SCT_IN1, SCT_IN2, SCT_IN3, SCT_OUT0."]
pub mod pinassign7;
#[doc = "PINASSIGN8 register accessor: an alias for `Reg<PINASSIGN8_SPEC>`"]
pub type PINASSIGN8 = crate::Reg<pinassign8::PINASSIGN8_SPEC>;
#[doc = "Pin assign register 8. Assign movable functions SCT_OUT1, SCT_OUT2, SCT_OUT3, SCT_OUT4."]
pub mod pinassign8;
#[doc = "PINASSIGN9 register accessor: an alias for `Reg<PINASSIGN9_SPEC>`"]
pub type PINASSIGN9 = crate::Reg<pinassign9::PINASSIGN9_SPEC>;
#[doc = "Pin assign register 9. Assign movable functions SCT_OUT5, I2C1_SDA, I2C1_SCL, I2C2_SDA."]
pub mod pinassign9;
#[doc = "PINASSIGN10 register accessor: an alias for `Reg<PINASSIGN10_SPEC>`"]
pub type PINASSIGN10 = crate::Reg<pinassign10::PINASSIGN10_SPEC>;
#[doc = "Pin assign register 10. Assign movable functions, I2C2_SCL, I2C3_SDA, I2C3_SCL, ADC_PINTRIG0."]
pub mod pinassign10;
#[doc = "PINASSIGN11 register accessor: an alias for `Reg<PINASSIGN11_SPEC>`"]
pub type PINASSIGN11 = crate::Reg<pinassign11::PINASSIGN11_SPEC>;
#[doc = "Pin assign register 11. Assign movable functions ADC_PINTRIG1, ACMP_O, CLKOUT, GPIO_INT_BMAT"]
pub mod pinassign11;
#[doc = "PINENABLE0 register accessor: an alias for `Reg<PINENABLE0_SPEC>`"]
pub type PINENABLE0 = crate::Reg<pinenable0::PINENABLE0_SPEC>;
#[doc = "Pin enable register 0. Enables fixed-pin functions ACMP_I0, ACMP_I1, SWCLK, SWDIO, XTALIN, XTALOUT, RESET, CLKIN, VDDCMP and so on."]
pub mod pinenable0;
