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
#[doc = "Reader of field `IRQM4`"]
pub type IRQM4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IRQM4`"]
pub struct IRQM4_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQM4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `NMIENM4`"]
pub type NMIENM4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NMIENM4`"]
pub struct NMIENM4_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIENM4_W<'a> {
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
    #[doc = "Bits 0:5 - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the Cortex-M4, if enabled by NMIENM4."]
    #[inline(always)]
    pub fn irqm4(&self) -> IRQM4_R {
        IRQM4_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQM4."]
    #[inline(always)]
    pub fn nmienm4(&self) -> NMIENM4_R {
        NMIENM4_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the Cortex-M4, if enabled by NMIENM4."]
    #[inline(always)]
    pub fn irqm4(&mut self) -> IRQM4_W {
        IRQM4_W { w: self }
    }
    #[doc = "Bit 31 - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQM4."]
    #[inline(always)]
    pub fn nmienm4(&mut self) -> NMIENM4_W {
        NMIENM4_W { w: self }
    }
}
