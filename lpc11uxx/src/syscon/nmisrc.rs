#[doc = "Reader of register NMISRC"]
pub type R = crate::R<u32, super::NMISRC>;
#[doc = "Writer for register NMISRC"]
pub type W = crate::W<u32, super::NMISRC>;
#[doc = "Register NMISRC `reset()`'s with value 0"]
impl crate::ResetValue for super::NMISRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IRQNO`"]
pub type IRQNO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IRQNO`"]
pub struct IRQNO_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQNO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `NMIEN`"]
pub type NMIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NMIEN`"]
pub struct NMIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) if bit 31 is 1. See Table 58 for the list of interrupt sources and their IRQ numbers."]
    #[inline(always)]
    pub fn irqno(&self) -> IRQNO_R {
        IRQNO_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by bits 4:0."]
    #[inline(always)]
    pub fn nmien(&self) -> NMIEN_R {
        NMIEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) if bit 31 is 1. See Table 58 for the list of interrupt sources and their IRQ numbers."]
    #[inline(always)]
    pub fn irqno(&mut self) -> IRQNO_W {
        IRQNO_W { w: self }
    }
    #[doc = "Bit 31 - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by bits 4:0."]
    #[inline(always)]
    pub fn nmien(&mut self) -> NMIEN_W {
        NMIEN_W { w: self }
    }
}
