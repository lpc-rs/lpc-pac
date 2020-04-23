#[doc = "Writer for register INTENCLR"]
pub type W = crate::W<u32, super::INTENCLR>;
#[doc = "Register INTENCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TXIDLECLR`"]
pub struct TXIDLECLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIDLECLR_W<'a> {
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
#[doc = "Write proxy for field `DELTACTSCLR`"]
pub struct DELTACTSCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DELTACTSCLR_W<'a> {
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
#[doc = "Write proxy for field `TXDISCLR`"]
pub struct TXDISCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDISCLR_W<'a> {
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
#[doc = "Write proxy for field `DELTARXBRKCLR`"]
pub struct DELTARXBRKCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DELTARXBRKCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Write proxy for field `STARTCLR`"]
pub struct STARTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Write proxy for field `FRAMERRCLR`"]
pub struct FRAMERRCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMERRCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Write proxy for field `PARITYERRCLR`"]
pub struct PARITYERRCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITYERRCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Write proxy for field `RXNOISECLR`"]
pub struct RXNOISECLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXNOISECLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Write proxy for field `ABERRCLR`"]
pub struct ABERRCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ABERRCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl W {
    #[doc = "Bit 3 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn txidleclr(&mut self) -> TXIDLECLR_W {
        TXIDLECLR_W { w: self }
    }
    #[doc = "Bit 5 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn deltactsclr(&mut self) -> DELTACTSCLR_W {
        DELTACTSCLR_W { w: self }
    }
    #[doc = "Bit 6 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn txdisclr(&mut self) -> TXDISCLR_W {
        TXDISCLR_W { w: self }
    }
    #[doc = "Bit 11 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn deltarxbrkclr(&mut self) -> DELTARXBRKCLR_W {
        DELTARXBRKCLR_W { w: self }
    }
    #[doc = "Bit 12 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn startclr(&mut self) -> STARTCLR_W {
        STARTCLR_W { w: self }
    }
    #[doc = "Bit 13 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn framerrclr(&mut self) -> FRAMERRCLR_W {
        FRAMERRCLR_W { w: self }
    }
    #[doc = "Bit 14 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn parityerrclr(&mut self) -> PARITYERRCLR_W {
        PARITYERRCLR_W { w: self }
    }
    #[doc = "Bit 15 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn rxnoiseclr(&mut self) -> RXNOISECLR_W {
        RXNOISECLR_W { w: self }
    }
    #[doc = "Bit 16 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn aberrclr(&mut self) -> ABERRCLR_W {
        ABERRCLR_W { w: self }
    }
}
