#[doc = "Reader of register RXF1S"]
pub type R = crate::R<u32, super::RXF1S>;
#[doc = "Reader of field `F1FL`"]
pub type F1FL_R = crate::R<u8, u8>;
#[doc = "Reader of field `F1GI`"]
pub type F1GI_R = crate::R<u8, u8>;
#[doc = "Reader of field `F1PI`"]
pub type F1PI_R = crate::R<u8, u8>;
#[doc = "Reader of field `F1F`"]
pub type F1F_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF1L`"]
pub type RF1L_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:6 - Rx FIFO 1 fill level."]
    #[inline(always)]
    pub fn f1fl(&self) -> F1FL_R {
        F1FL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - Rx FIFO 1 get index."]
    #[inline(always)]
    pub fn f1gi(&self) -> F1GI_R {
        F1GI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Rx FIFO 1 put index."]
    #[inline(always)]
    pub fn f1pi(&self) -> F1PI_R {
        F1PI_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 1 full."]
    #[inline(always)]
    pub fn f1f(&self) -> F1F_R {
        F1F_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 1 message lost."]
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
