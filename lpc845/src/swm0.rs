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
    #[doc = "0x24 - Pin assign register 9. Assign movable functions SCT_OUT5, SCT_OUT6, I2C1_SDA, I2C1_SCL."]
    pub pinassign9: PINASSIGN9,
    #[doc = "0x28 - Pin assign register 10. Assign movable functions I2C2_SDA, I2C2_SCL, I2C3_SDA, I2C3_SCL."]
    pub pinassign10: PINASSIGN10,
    #[doc = "0x2c - Pin assign register 11. Assign movable functions COMP0_OUT, CLKOUT, GPIOINT_BMATCH, UART3_TXD"]
    pub pinassign11: PINASSIGN11,
    #[doc = "0x30 - Pin assign register 12. Assign movable functions UART3_RXD, UART3_SCLK, UART4_TXD, UART4_RXD."]
    pub pinassign12: PINASSIGN12,
    #[doc = "0x34 - Pin assign register 13. Assign movable functions UART4_SCLK, T0_MAT0, T0_MAT1, T0_MAT2."]
    pub pinassign13: PINASSIGN13,
    #[doc = "0x38 - Pin assign register 14. Assign movable functions T0_MAT3, T0_CAP0, T0_CAP1, T0_CAP2."]
    pub pinassign14: PINASSIGN14,
    _reserved0: [u8; 388usize],
    #[doc = "0x1c0 - Pin enable register 0. Enables fixed-pin functions ACMP_I0, ACMP_I1, SWCLK, SWDIO, XTALIN, XTALOUT, RESET, CLKIN, VDDCMP and so on."]
    pub pinenable0: PINENABLE0,
    #[doc = "0x1c4 - Pin enable register 1. Enables fixed-pin functions CAPT_X4, CAPT_X5, CAPT_X6, CAPT_X7, CAPT_X8, CAPT_X4, CAPT_YL and CAPT_YH."]
    pub pinenable1: PINENABLE1,
}
#[doc = "Pin assign register 0. Assign movable functions U0_TXD, U0_RXD, U0_RTS, U0_CTS."]
pub struct PINASSIGN0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 0. Assign movable functions U0_TXD, U0_RXD, U0_RTS, U0_CTS."]
pub mod pinassign0;
#[doc = "Pin assign register"]
pub struct PINASSIGN_DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register"]
pub mod pinassign_data0;
#[doc = "Pin assign register 1. Assign movable functions U0_SCLK, U1_TXD, U1_RXD, U1_RTS."]
pub struct PINASSIGN1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 1. Assign movable functions U0_SCLK, U1_TXD, U1_RXD, U1_RTS."]
pub mod pinassign1;
#[doc = "Pin assign register"]
pub struct PINASSIGN_DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register"]
pub mod pinassign_data1;
#[doc = "Pin assign register 2. Assign movable functions U1_CTS, U1_SCLK, U2_TXD, U2_RXD."]
pub struct PINASSIGN2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 2. Assign movable functions U1_CTS, U1_SCLK, U2_TXD, U2_RXD."]
pub mod pinassign2;
#[doc = "Pin assign register"]
pub struct PINASSIGN_DATA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register"]
pub mod pinassign_data2;
#[doc = "Pin assign register 3. Assign movable function U2_RTS, U2_CTS, U2_SCLK, SPI0_SCK."]
pub struct PINASSIGN3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 3. Assign movable function U2_RTS, U2_CTS, U2_SCLK, SPI0_SCK."]
pub mod pinassign3;
#[doc = "Pin assign register"]
pub struct PINASSIGN_DATA3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register"]
pub mod pinassign_data3;
#[doc = "Pin assign register 4. Assign movable functions SPI0_MOSI, SPI0_MISO, SPI0_SSEL0, SPI0_SSEL1."]
pub struct PINASSIGN4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 4. Assign movable functions SPI0_MOSI, SPI0_MISO, SPI0_SSEL0, SPI0_SSEL1."]
pub mod pinassign4;
#[doc = "Pin assign register"]
pub struct PINASSIGN_DATA4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register"]
pub mod pinassign_data4;
#[doc = "Pin assign register 5. Assign movable functions SPI0_SSEL2, SPI0_SSEL3, SPI1_SCK, SPI1_MOSI"]
pub struct PINASSIGN5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 5. Assign movable functions SPI0_SSEL2, SPI0_SSEL3, SPI1_SCK, SPI1_MOSI"]
pub mod pinassign5;
#[doc = "Pin assign register"]
pub struct PINASSIGN_DATA5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register"]
pub mod pinassign_data5;
#[doc = "Pin assign register 6. Assign movable functions SPI1_MISO, SPI1_SSEL0, SPI1_SSEL1, SCT0_IN0."]
pub struct PINASSIGN6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 6. Assign movable functions SPI1_MISO, SPI1_SSEL0, SPI1_SSEL1, SCT0_IN0."]
pub mod pinassign6;
#[doc = "Pin assign register"]
pub struct PINASSIGN_DATA6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register"]
pub mod pinassign_data6;
#[doc = "Pin assign register 7. Assign movable functions SCT_IN1, SCT_IN2, SCT_IN3, SCT_OUT0."]
pub struct PINASSIGN7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 7. Assign movable functions SCT_IN1, SCT_IN2, SCT_IN3, SCT_OUT0."]
pub mod pinassign7;
#[doc = "Pin assign register"]
pub struct PINASSIGN_DATA7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register"]
pub mod pinassign_data7;
#[doc = "Pin assign register 8. Assign movable functions SCT_OUT1, SCT_OUT2, SCT_OUT3, SCT_OUT4."]
pub struct PINASSIGN8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 8. Assign movable functions SCT_OUT1, SCT_OUT2, SCT_OUT3, SCT_OUT4."]
pub mod pinassign8;
#[doc = "Pin assign register"]
pub struct PINASSIGN_DATA8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register"]
pub mod pinassign_data8;
#[doc = "Pin assign register 9. Assign movable functions SCT_OUT5, SCT_OUT6, I2C1_SDA, I2C1_SCL."]
pub struct PINASSIGN9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 9. Assign movable functions SCT_OUT5, SCT_OUT6, I2C1_SDA, I2C1_SCL."]
pub mod pinassign9;
#[doc = "Pin assign register"]
pub struct PINASSIGN_DATA9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register"]
pub mod pinassign_data9;
#[doc = "Pin assign register 10. Assign movable functions I2C2_SDA, I2C2_SCL, I2C3_SDA, I2C3_SCL."]
pub struct PINASSIGN10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 10. Assign movable functions I2C2_SDA, I2C2_SCL, I2C3_SDA, I2C3_SCL."]
pub mod pinassign10;
#[doc = "Pin assign register"]
pub struct PINASSIGN_DATA10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register"]
pub mod pinassign_data10;
#[doc = "Pin assign register 11. Assign movable functions COMP0_OUT, CLKOUT, GPIOINT_BMATCH, UART3_TXD"]
pub struct PINASSIGN11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 11. Assign movable functions COMP0_OUT, CLKOUT, GPIOINT_BMATCH, UART3_TXD"]
pub mod pinassign11;
#[doc = "Pin assign register"]
pub struct PINASSIGN_DATA11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register"]
pub mod pinassign_data11;
#[doc = "Pin assign register 12. Assign movable functions UART3_RXD, UART3_SCLK, UART4_TXD, UART4_RXD."]
pub struct PINASSIGN12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 12. Assign movable functions UART3_RXD, UART3_SCLK, UART4_TXD, UART4_RXD."]
pub mod pinassign12;
#[doc = "Pin assign register"]
pub struct PINASSIGN_DATA12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register"]
pub mod pinassign_data12;
#[doc = "Pin assign register 13. Assign movable functions UART4_SCLK, T0_MAT0, T0_MAT1, T0_MAT2."]
pub struct PINASSIGN13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 13. Assign movable functions UART4_SCLK, T0_MAT0, T0_MAT1, T0_MAT2."]
pub mod pinassign13;
#[doc = "Pin assign register"]
pub struct PINASSIGN_DATA13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register"]
pub mod pinassign_data13;
#[doc = "Pin assign register 14. Assign movable functions T0_MAT3, T0_CAP0, T0_CAP1, T0_CAP2."]
pub struct PINASSIGN14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register 14. Assign movable functions T0_MAT3, T0_CAP0, T0_CAP1, T0_CAP2."]
pub mod pinassign14;
#[doc = "Pin assign register"]
pub struct PINASSIGN_DATA14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin assign register"]
pub mod pinassign_data14;
#[doc = "Pin enable register 0. Enables fixed-pin functions ACMP_I0, ACMP_I1, SWCLK, SWDIO, XTALIN, XTALOUT, RESET, CLKIN, VDDCMP and so on."]
pub struct PINENABLE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin enable register 0. Enables fixed-pin functions ACMP_I0, ACMP_I1, SWCLK, SWDIO, XTALIN, XTALOUT, RESET, CLKIN, VDDCMP and so on."]
pub mod pinenable0;
#[doc = "Pin enable register 1. Enables fixed-pin functions CAPT_X4, CAPT_X5, CAPT_X6, CAPT_X7, CAPT_X8, CAPT_X4, CAPT_YL and CAPT_YH."]
pub struct PINENABLE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin enable register 1. Enables fixed-pin functions CAPT_X4, CAPT_X5, CAPT_X6, CAPT_X7, CAPT_X8, CAPT_X4, CAPT_YL and CAPT_YH."]
pub mod pinenable1;
