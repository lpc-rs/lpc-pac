#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2S Digital Audio Output Register. Contains control bits for the I2S transmit channel."]
    pub dao: DAO,
    #[doc = "0x04 - I2S Digital Audio Input Register. Contains control bits for the I2S receive channel."]
    pub dai: DAI,
    #[doc = "0x08 - I2S Transmit FIFO. Access register for the 8 x 32-bit transmitter FIFO."]
    pub txfifo: TXFIFO,
    #[doc = "0x0c - I2S Receive FIFO. Access register for the 8 x 32-bit receiver FIFO."]
    pub rxfifo: RXFIFO,
    #[doc = "0x10 - I2S Status Feedback Register. Contains status information about the I2S interface."]
    pub state: STATE,
    #[doc = "0x14 - I2S DMA Configuration Register 1. Contains control information for DMA request 1."]
    pub dma1: DMA1,
    #[doc = "0x18 - I2S DMA Configuration Register 2. Contains control information for DMA request 2."]
    pub dma2: DMA2,
    #[doc = "0x1c - I2S Interrupt Request Control Register. Contains bits that control how the I2S interrupt request is generated."]
    pub irq: IRQ,
    #[doc = "0x20 - I2S Transmit MCLK divider. This register determines the I2S TX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK."]
    pub txrate: TXRATE,
    #[doc = "0x24 - I2S Receive MCLK divider. This register determines the I2S RX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK."]
    pub rxrate: RXRATE,
    #[doc = "0x28 - I2S Transmit bit rate divider. This register determines the I2S transmit bit rate by specifying the value to divide TX_MCLK by in order to produce the transmit bit clock."]
    pub txbitrate: TXBITRATE,
    #[doc = "0x2c - I2S Receive bit rate divider. This register determines the I2S receive bit rate by specifying the value to divide RX_MCLK by in order to produce the receive bit clock."]
    pub rxbitrate: RXBITRATE,
    #[doc = "0x30 - I2S Transmit mode control."]
    pub txmode: TXMODE,
    #[doc = "0x34 - I2S Receive mode control."]
    pub rxmode: RXMODE,
}
#[doc = "I2S Digital Audio Output Register. Contains control bits for the I2S transmit channel."]
pub struct DAO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S Digital Audio Output Register. Contains control bits for the I2S transmit channel."]
pub mod dao;
#[doc = "I2S Digital Audio Input Register. Contains control bits for the I2S receive channel."]
pub struct DAI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S Digital Audio Input Register. Contains control bits for the I2S receive channel."]
pub mod dai;
#[doc = "I2S Transmit FIFO. Access register for the 8 x 32-bit transmitter FIFO."]
pub struct TXFIFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S Transmit FIFO. Access register for the 8 x 32-bit transmitter FIFO."]
pub mod txfifo;
#[doc = "I2S Receive FIFO. Access register for the 8 x 32-bit receiver FIFO."]
pub struct RXFIFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S Receive FIFO. Access register for the 8 x 32-bit receiver FIFO."]
pub mod rxfifo;
#[doc = "I2S Status Feedback Register. Contains status information about the I2S interface."]
pub struct STATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S Status Feedback Register. Contains status information about the I2S interface."]
pub mod state;
#[doc = "I2S DMA Configuration Register 1. Contains control information for DMA request 1."]
pub struct DMA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S DMA Configuration Register 1. Contains control information for DMA request 1."]
pub mod dma1;
#[doc = "I2S DMA Configuration Register 2. Contains control information for DMA request 2."]
pub struct DMA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S DMA Configuration Register 2. Contains control information for DMA request 2."]
pub mod dma2;
#[doc = "I2S Interrupt Request Control Register. Contains bits that control how the I2S interrupt request is generated."]
pub struct IRQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S Interrupt Request Control Register. Contains bits that control how the I2S interrupt request is generated."]
pub mod irq;
#[doc = "I2S Transmit MCLK divider. This register determines the I2S TX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK."]
pub struct TXRATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S Transmit MCLK divider. This register determines the I2S TX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK."]
pub mod txrate;
#[doc = "I2S Receive MCLK divider. This register determines the I2S RX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK."]
pub struct RXRATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S Receive MCLK divider. This register determines the I2S RX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK."]
pub mod rxrate;
#[doc = "I2S Transmit bit rate divider. This register determines the I2S transmit bit rate by specifying the value to divide TX_MCLK by in order to produce the transmit bit clock."]
pub struct TXBITRATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S Transmit bit rate divider. This register determines the I2S transmit bit rate by specifying the value to divide TX_MCLK by in order to produce the transmit bit clock."]
pub mod txbitrate;
#[doc = "I2S Receive bit rate divider. This register determines the I2S receive bit rate by specifying the value to divide RX_MCLK by in order to produce the receive bit clock."]
pub struct RXBITRATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S Receive bit rate divider. This register determines the I2S receive bit rate by specifying the value to divide RX_MCLK by in order to produce the receive bit clock."]
pub mod rxbitrate;
#[doc = "I2S Transmit mode control."]
pub struct TXMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S Transmit mode control."]
pub mod txmode;
#[doc = "I2S Receive mode control."]
pub struct RXMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S Receive mode control."]
pub mod rxmode;
