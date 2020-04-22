#[doc = "Reader of register FIFORD"]
pub type R = crate::R<u32, super::FIFORD>;
#[doc = "Reader of field `RXDATA`"]
pub type RXDATA_R = crate::R<u16, u16>;
#[doc = "Reader of field `RXSSEL0_N`"]
pub type RXSSEL0_N_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXSSEL1_N`"]
pub type RXSSEL1_N_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXSSEL2_N`"]
pub type RXSSEL2_N_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXSSEL3_N`"]
pub type RXSSEL3_N_R = crate::R<bool, bool>;
#[doc = "Reader of field `SOT`"]
pub type SOT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:15 - Received data from the FIFO."]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Slave Select for receive. This field allows the state of the SSEL0 pin to be saved along with received data. The value will reflect the SSEL0 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub fn rxssel0_n(&self) -> RXSSEL0_N_R {
        RXSSEL0_N_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Slave Select for receive. This field allows the state of the SSEL1 pin to be saved along with received data. The value will reflect the SSEL1 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub fn rxssel1_n(&self) -> RXSSEL1_N_R {
        RXSSEL1_N_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Slave Select for receive. This field allows the state of the SSEL2 pin to be saved along with received data. The value will reflect the SSEL2 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub fn rxssel2_n(&self) -> RXSSEL2_N_R {
        RXSSEL2_N_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Slave Select for receive. This field allows the state of the SSEL3 pin to be saved along with received data. The value will reflect the SSEL3 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub fn rxssel3_n(&self) -> RXSSEL3_N_R {
        RXSSEL3_N_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Start of Transfer flag. This flag will be 1 if this is the first data after the SSELs went from deasserted to asserted (i.e., any previous transfer has ended). This information can be used to identify the first piece of data in cases where the transfer length is greater than 16 bits."]
    #[inline(always)]
    pub fn sot(&self) -> SOT_R {
        SOT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
