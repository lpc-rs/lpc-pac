#[doc = "Reader of register STATICCONFIG"]
pub type R = crate::R<u32, super::STATICCONFIG>;
#[doc = "Writer for register STATICCONFIG"]
pub type W = crate::W<u32, super::STATICCONFIG>;
#[doc = "Register STATICCONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::STATICCONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MW`"]
pub type MW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MW`"]
pub struct MW_W<'a> {
    w: &'a mut W,
}
impl<'a> MW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `PM`"]
pub type PM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PM`"]
pub struct PM_W<'a> {
    w: &'a mut W,
}
impl<'a> PM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `PC`"]
pub type PC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC`"]
pub struct PC_W<'a> {
    w: &'a mut W,
}
impl<'a> PC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `PB`"]
pub type PB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PB`"]
pub struct PB_W<'a> {
    w: &'a mut W,
}
impl<'a> PB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `EW`"]
pub type EW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EW`"]
pub struct EW_W<'a> {
    w: &'a mut W,
}
impl<'a> EW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `B`"]
pub type B_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B`"]
pub struct B_W<'a> {
    w: &'a mut W,
}
impl<'a> B_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `P`"]
pub type P_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P`"]
pub struct P_W<'a> {
    w: &'a mut W,
}
impl<'a> P_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Memory width."]
    #[inline(always)]
    pub fn mw(&self) -> MW_R {
        MW_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 3 - Page mode."]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Chip select polarity."]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Byte lane state."]
    #[inline(always)]
    pub fn pb(&self) -> PB_R {
        PB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Extended wait (EW) uses the EMCStaticExtendedWait register to time both the read and write transfers rather than the EMCStaticWaitRd and EMCStaticWaitWr registers."]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Buffer enable \\[2\\]."]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Write protect."]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory width."]
    #[inline(always)]
    pub fn mw(&mut self) -> MW_W {
        MW_W { w: self }
    }
    #[doc = "Bit 3 - Page mode."]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W {
        PM_W { w: self }
    }
    #[doc = "Bit 6 - Chip select polarity."]
    #[inline(always)]
    pub fn pc(&mut self) -> PC_W {
        PC_W { w: self }
    }
    #[doc = "Bit 7 - Byte lane state."]
    #[inline(always)]
    pub fn pb(&mut self) -> PB_W {
        PB_W { w: self }
    }
    #[doc = "Bit 8 - Extended wait (EW) uses the EMCStaticExtendedWait register to time both the read and write transfers rather than the EMCStaticWaitRd and EMCStaticWaitWr registers."]
    #[inline(always)]
    pub fn ew(&mut self) -> EW_W {
        EW_W { w: self }
    }
    #[doc = "Bit 19 - Buffer enable \\[2\\]."]
    #[inline(always)]
    pub fn b(&mut self) -> B_W {
        B_W { w: self }
    }
    #[doc = "Bit 20 - Write protect."]
    #[inline(always)]
    pub fn p(&mut self) -> P_W {
        P_W { w: self }
    }
}
