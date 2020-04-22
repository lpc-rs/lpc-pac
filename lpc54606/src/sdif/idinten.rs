#[doc = "Reader of register IDINTEN"]
pub type R = crate::R<u32, super::IDINTEN>;
#[doc = "Writer for register IDINTEN"]
pub type W = crate::W<u32, super::IDINTEN>;
#[doc = "Register IDINTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::IDINTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TI`"]
pub type TI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TI`"]
pub struct TI_W<'a> {
    w: &'a mut W,
}
impl<'a> TI_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `RI`"]
pub type RI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RI`"]
pub struct RI_W<'a> {
    w: &'a mut W,
}
impl<'a> RI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `FBE`"]
pub type FBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBE`"]
pub struct FBE_W<'a> {
    w: &'a mut W,
}
impl<'a> FBE_W<'a> {
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
#[doc = "Reader of field `DU`"]
pub type DU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DU`"]
pub struct DU_W<'a> {
    w: &'a mut W,
}
impl<'a> DU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `CES`"]
pub type CES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CES`"]
pub struct CES_W<'a> {
    w: &'a mut W,
}
impl<'a> CES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `NIS`"]
pub type NIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NIS`"]
pub struct NIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NIS_W<'a> {
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
#[doc = "Reader of field `AIS`"]
pub type AIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AIS`"]
pub struct AIS_W<'a> {
    w: &'a mut W,
}
impl<'a> AIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Interrupt Enable."]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Interrupt Enable."]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fatal Bus Error Enable."]
    #[inline(always)]
    pub fn fbe(&self) -> FBE_R {
        FBE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt."]
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Card Error summary Interrupt Enable."]
    #[inline(always)]
    pub fn ces(&self) -> CES_R {
        CES_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Normal Interrupt Summary Enable."]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary Enable."]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt Enable."]
    #[inline(always)]
    pub fn ti(&mut self) -> TI_W {
        TI_W { w: self }
    }
    #[doc = "Bit 1 - Receive Interrupt Enable."]
    #[inline(always)]
    pub fn ri(&mut self) -> RI_W {
        RI_W { w: self }
    }
    #[doc = "Bit 2 - Fatal Bus Error Enable."]
    #[inline(always)]
    pub fn fbe(&mut self) -> FBE_W {
        FBE_W { w: self }
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt."]
    #[inline(always)]
    pub fn du(&mut self) -> DU_W {
        DU_W { w: self }
    }
    #[doc = "Bit 5 - Card Error summary Interrupt Enable."]
    #[inline(always)]
    pub fn ces(&mut self) -> CES_W {
        CES_W { w: self }
    }
    #[doc = "Bit 8 - Normal Interrupt Summary Enable."]
    #[inline(always)]
    pub fn nis(&mut self) -> NIS_W {
        NIS_W { w: self }
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary Enable."]
    #[inline(always)]
    pub fn ais(&mut self) -> AIS_W {
        AIS_W { w: self }
    }
}
