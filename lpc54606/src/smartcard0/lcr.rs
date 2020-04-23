#[doc = "Reader of register LCR"]
pub type R = crate::R<u32, super::LCR>;
#[doc = "Writer for register LCR"]
pub type W = crate::W<u32, super::LCR>;
#[doc = "Register LCR `reset()`'s with value 0"]
impl crate::ResetValue for super::LCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WLS`"]
pub type WLS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WLS`"]
pub struct WLS_W<'a> {
    w: &'a mut W,
}
impl<'a> WLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `SBS`"]
pub type SBS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SBS`"]
pub struct SBS_W<'a> {
    w: &'a mut W,
}
impl<'a> SBS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `PE`"]
pub type PE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PE`"]
pub struct PE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE_W<'a> {
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
#[doc = "Reader of field `PS`"]
pub type PS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PS`"]
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `DLAB`"]
pub type DLAB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DLAB`"]
pub struct DLAB_W<'a> {
    w: &'a mut W,
}
impl<'a> DLAB_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - Word Length Select."]
    #[inline(always)]
    pub fn wls(&self) -> WLS_R {
        WLS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Stop Bit Select."]
    #[inline(always)]
    pub fn sbs(&self) -> SBS_R {
        SBS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Parity Enable."]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Parity Select."]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit."]
    #[inline(always)]
    pub fn dlab(&self) -> DLAB_R {
        DLAB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Word Length Select."]
    #[inline(always)]
    pub fn wls(&mut self) -> WLS_W {
        WLS_W { w: self }
    }
    #[doc = "Bit 2 - Stop Bit Select."]
    #[inline(always)]
    pub fn sbs(&mut self) -> SBS_W {
        SBS_W { w: self }
    }
    #[doc = "Bit 3 - Parity Enable."]
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W {
        PE_W { w: self }
    }
    #[doc = "Bits 4:5 - Parity Select."]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit."]
    #[inline(always)]
    pub fn dlab(&mut self) -> DLAB_W {
        DLAB_W { w: self }
    }
}
