#[doc = r"Register block"]
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
#[doc = "I2S Digital Audio Output Register. Contains control bits for the I2S transmit channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dao](dao) module"]
pub type DAO = crate::Reg<u32, _DAO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAO;
#[doc = "`read()` method returns [dao::R](dao::R) reader structure"]
impl crate::Readable for DAO {}
#[doc = "`write(|w| ..)` method takes [dao::W](dao::W) writer structure"]
impl crate::Writable for DAO {}
#[doc = "I2S Digital Audio Output Register. Contains control bits for the I2S transmit channel."]
pub mod dao;
#[doc = "I2S Digital Audio Input Register. Contains control bits for the I2S receive channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dai](dai) module"]
pub type DAI = crate::Reg<u32, _DAI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAI;
#[doc = "`read()` method returns [dai::R](dai::R) reader structure"]
impl crate::Readable for DAI {}
#[doc = "`write(|w| ..)` method takes [dai::W](dai::W) writer structure"]
impl crate::Writable for DAI {}
#[doc = "I2S Digital Audio Input Register. Contains control bits for the I2S receive channel."]
pub mod dai;
#[doc = "I2S Transmit FIFO. Access register for the 8 x 32-bit transmitter FIFO.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfifo](txfifo) module"]
pub type TXFIFO = crate::Reg<u32, _TXFIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFIFO;
#[doc = "`write(|w| ..)` method takes [txfifo::W](txfifo::W) writer structure"]
impl crate::Writable for TXFIFO {}
#[doc = "I2S Transmit FIFO. Access register for the 8 x 32-bit transmitter FIFO."]
pub mod txfifo;
#[doc = "I2S Receive FIFO. Access register for the 8 x 32-bit receiver FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfifo](rxfifo) module"]
pub type RXFIFO = crate::Reg<u32, _RXFIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFIFO;
#[doc = "`read()` method returns [rxfifo::R](rxfifo::R) reader structure"]
impl crate::Readable for RXFIFO {}
#[doc = "I2S Receive FIFO. Access register for the 8 x 32-bit receiver FIFO."]
pub mod rxfifo;
#[doc = "I2S Status Feedback Register. Contains status information about the I2S interface.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state](state) module"]
pub type STATE = crate::Reg<u32, _STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATE;
#[doc = "`read()` method returns [state::R](state::R) reader structure"]
impl crate::Readable for STATE {}
#[doc = "I2S Status Feedback Register. Contains status information about the I2S interface."]
pub mod state;
#[doc = "I2S DMA Configuration Register 1. Contains control information for DMA request 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma1](dma1) module"]
pub type DMA1 = crate::Reg<u32, _DMA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA1;
#[doc = "`read()` method returns [dma1::R](dma1::R) reader structure"]
impl crate::Readable for DMA1 {}
#[doc = "`write(|w| ..)` method takes [dma1::W](dma1::W) writer structure"]
impl crate::Writable for DMA1 {}
#[doc = "I2S DMA Configuration Register 1. Contains control information for DMA request 1."]
pub mod dma1;
#[doc = "I2S DMA Configuration Register 2. Contains control information for DMA request 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2](dma2) module"]
pub type DMA2 = crate::Reg<u32, _DMA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2;
#[doc = "`read()` method returns [dma2::R](dma2::R) reader structure"]
impl crate::Readable for DMA2 {}
#[doc = "`write(|w| ..)` method takes [dma2::W](dma2::W) writer structure"]
impl crate::Writable for DMA2 {}
#[doc = "I2S DMA Configuration Register 2. Contains control information for DMA request 2."]
pub mod dma2;
#[doc = "I2S Interrupt Request Control Register. Contains bits that control how the I2S interrupt request is generated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq](irq) module"]
pub type IRQ = crate::Reg<u32, _IRQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ;
#[doc = "`read()` method returns [irq::R](irq::R) reader structure"]
impl crate::Readable for IRQ {}
#[doc = "`write(|w| ..)` method takes [irq::W](irq::W) writer structure"]
impl crate::Writable for IRQ {}
#[doc = "I2S Interrupt Request Control Register. Contains bits that control how the I2S interrupt request is generated."]
pub mod irq;
#[doc = "I2S Transmit MCLK divider. This register determines the I2S TX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txrate](txrate) module"]
pub type TXRATE = crate::Reg<u32, _TXRATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXRATE;
#[doc = "`read()` method returns [txrate::R](txrate::R) reader structure"]
impl crate::Readable for TXRATE {}
#[doc = "`write(|w| ..)` method takes [txrate::W](txrate::W) writer structure"]
impl crate::Writable for TXRATE {}
#[doc = "I2S Transmit MCLK divider. This register determines the I2S TX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK."]
pub mod txrate;
#[doc = "I2S Receive MCLK divider. This register determines the I2S RX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxrate](rxrate) module"]
pub type RXRATE = crate::Reg<u32, _RXRATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXRATE;
#[doc = "`read()` method returns [rxrate::R](rxrate::R) reader structure"]
impl crate::Readable for RXRATE {}
#[doc = "`write(|w| ..)` method takes [rxrate::W](rxrate::W) writer structure"]
impl crate::Writable for RXRATE {}
#[doc = "I2S Receive MCLK divider. This register determines the I2S RX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK."]
pub mod rxrate;
#[doc = "I2S Transmit bit rate divider. This register determines the I2S transmit bit rate by specifying the value to divide TX_MCLK by in order to produce the transmit bit clock.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbitrate](txbitrate) module"]
pub type TXBITRATE = crate::Reg<u32, _TXBITRATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBITRATE;
#[doc = "`read()` method returns [txbitrate::R](txbitrate::R) reader structure"]
impl crate::Readable for TXBITRATE {}
#[doc = "`write(|w| ..)` method takes [txbitrate::W](txbitrate::W) writer structure"]
impl crate::Writable for TXBITRATE {}
#[doc = "I2S Transmit bit rate divider. This register determines the I2S transmit bit rate by specifying the value to divide TX_MCLK by in order to produce the transmit bit clock."]
pub mod txbitrate;
#[doc = "I2S Receive bit rate divider. This register determines the I2S receive bit rate by specifying the value to divide RX_MCLK by in order to produce the receive bit clock.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbitrate](rxbitrate) module"]
pub type RXBITRATE = crate::Reg<u32, _RXBITRATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXBITRATE;
#[doc = "`read()` method returns [rxbitrate::R](rxbitrate::R) reader structure"]
impl crate::Readable for RXBITRATE {}
#[doc = "`write(|w| ..)` method takes [rxbitrate::W](rxbitrate::W) writer structure"]
impl crate::Writable for RXBITRATE {}
#[doc = "I2S Receive bit rate divider. This register determines the I2S receive bit rate by specifying the value to divide RX_MCLK by in order to produce the receive bit clock."]
pub mod rxbitrate;
#[doc = "I2S Transmit mode control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txmode](txmode) module"]
pub type TXMODE = crate::Reg<u32, _TXMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXMODE;
#[doc = "`read()` method returns [txmode::R](txmode::R) reader structure"]
impl crate::Readable for TXMODE {}
#[doc = "`write(|w| ..)` method takes [txmode::W](txmode::W) writer structure"]
impl crate::Writable for TXMODE {}
#[doc = "I2S Transmit mode control."]
pub mod txmode;
#[doc = "I2S Receive mode control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmode](rxmode) module"]
pub type RXMODE = crate::Reg<u32, _RXMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXMODE;
#[doc = "`read()` method returns [rxmode::R](rxmode::R) reader structure"]
impl crate::Readable for RXMODE {}
#[doc = "`write(|w| ..)` method takes [rxmode::W](rxmode::W) writer structure"]
impl crate::Writable for RXMODE {}
#[doc = "I2S Receive mode control."]
pub mod rxmode;
