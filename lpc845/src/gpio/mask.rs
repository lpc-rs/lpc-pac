#[doc = "Reader of register MASK[%s]"]
pub type R = crate::R<u32, super::MASK>;
#[doc = "Writer for register MASK[%s]"]
pub type W = crate::W<u32, super::MASK>;
#[doc = "Register MASK[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MASKP`"]
pub type MASKP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MASKP`"]
pub struct MASKP_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Controls which bits corresponding to PIOm_n are active in the MPORT register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp(&self) -> MASKP_R {
        MASKP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Controls which bits corresponding to PIOm_n are active in the MPORT register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp(&mut self) -> MASKP_W {
        MASKP_W { w: self }
    }
}
