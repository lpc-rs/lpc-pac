#[doc = "Writer for register RXFILTERWOLCLEAR"]
pub type W = crate::W<u32, super::RXFILTERWOLCLEAR>;
#[doc = "Register RXFILTERWOLCLEAR `reset()`'s with value 0"]
impl crate::ResetValue for super::RXFILTERWOLCLEAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `AUWCLR`"]
pub struct AUWCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> AUWCLR_W<'a> {
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
#[doc = "Write proxy for field `ABWCLR`"]
pub struct ABWCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ABWCLR_W<'a> {
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
#[doc = "Write proxy for field `AMWCLR`"]
pub struct AMWCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> AMWCLR_W<'a> {
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
#[doc = "Write proxy for field `AUHWCLR`"]
pub struct AUHWCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> AUHWCLR_W<'a> {
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
#[doc = "Write proxy for field `AMHWCLR`"]
pub struct AMHWCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> AMHWCLR_W<'a> {
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
#[doc = "Write proxy for field `APWCLR`"]
pub struct APWCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> APWCLR_W<'a> {
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
#[doc = "Write proxy for field `RFWCLR`"]
pub struct RFWCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RFWCLR_W<'a> {
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
#[doc = "Write proxy for field `MPWCLR`"]
pub struct MPWCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> MPWCLR_W<'a> {
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
impl W {
    #[doc = "Bit 0 - AcceptUnicastWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
    #[inline(always)]
    pub fn auwclr(&mut self) -> AUWCLR_W {
        AUWCLR_W { w: self }
    }
    #[doc = "Bit 1 - AcceptBroadcastWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
    #[inline(always)]
    pub fn abwclr(&mut self) -> ABWCLR_W {
        ABWCLR_W { w: self }
    }
    #[doc = "Bit 2 - AcceptMulticastWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
    #[inline(always)]
    pub fn amwclr(&mut self) -> AMWCLR_W {
        AMWCLR_W { w: self }
    }
    #[doc = "Bit 3 - AcceptUnicastHashWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
    #[inline(always)]
    pub fn auhwclr(&mut self) -> AUHWCLR_W {
        AUHWCLR_W { w: self }
    }
    #[doc = "Bit 4 - AcceptMulticastHashWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
    #[inline(always)]
    pub fn amhwclr(&mut self) -> AMHWCLR_W {
        AMHWCLR_W { w: self }
    }
    #[doc = "Bit 5 - AcceptPerfectWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
    #[inline(always)]
    pub fn apwclr(&mut self) -> APWCLR_W {
        APWCLR_W { w: self }
    }
    #[doc = "Bit 7 - RxFilterWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
    #[inline(always)]
    pub fn rfwclr(&mut self) -> RFWCLR_W {
        RFWCLR_W { w: self }
    }
    #[doc = "Bit 8 - MagicPacketWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
    #[inline(always)]
    pub fn mpwclr(&mut self) -> MPWCLR_W {
        MPWCLR_W { w: self }
    }
}
