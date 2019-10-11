#[doc = "Writer for register INTCLEAR"]
pub type W = crate::W<u32, super::INTCLEAR>;
#[doc = "Register INTCLEAR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTCLEAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RXOVERRUNINTCLR`"]
pub struct RXOVERRUNINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVERRUNINTCLR_W<'a> {
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
#[doc = "Write proxy for field `RXERRORINTCLR`"]
pub struct RXERRORINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXERRORINTCLR_W<'a> {
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
#[doc = "Write proxy for field `RXFINISHEDINTCLR`"]
pub struct RXFINISHEDINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFINISHEDINTCLR_W<'a> {
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
#[doc = "Write proxy for field `RXDONEINTCLR`"]
pub struct RXDONEINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDONEINTCLR_W<'a> {
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
#[doc = "Write proxy for field `TXUNDERRUNINTCLR`"]
pub struct TXUNDERRUNINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUNDERRUNINTCLR_W<'a> {
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
#[doc = "Write proxy for field `TXERRORINTCLR`"]
pub struct TXERRORINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXERRORINTCLR_W<'a> {
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
#[doc = "Write proxy for field `TXFINISHEDINTCLR`"]
pub struct TXFINISHEDINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFINISHEDINTCLR_W<'a> {
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
#[doc = "Write proxy for field `TXDONEINTCLR`"]
pub struct TXDONEINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDONEINTCLR_W<'a> {
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
#[doc = "Write proxy for field `SOFTINTCLR`"]
pub struct SOFTINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTINTCLR_W<'a> {
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
#[doc = "Write proxy for field `WAKEUPINTCLR`"]
pub struct WAKEUPINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPINTCLR_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn rxoverrunintclr(&mut self) -> RXOVERRUNINTCLR_W {
        RXOVERRUNINTCLR_W { w: self }
    }
    #[doc = "Bit 1 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn rxerrorintclr(&mut self) -> RXERRORINTCLR_W {
        RXERRORINTCLR_W { w: self }
    }
    #[doc = "Bit 2 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn rxfinishedintclr(&mut self) -> RXFINISHEDINTCLR_W {
        RXFINISHEDINTCLR_W { w: self }
    }
    #[doc = "Bit 3 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn rxdoneintclr(&mut self) -> RXDONEINTCLR_W {
        RXDONEINTCLR_W { w: self }
    }
    #[doc = "Bit 4 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn txunderrunintclr(&mut self) -> TXUNDERRUNINTCLR_W {
        TXUNDERRUNINTCLR_W { w: self }
    }
    #[doc = "Bit 5 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn txerrorintclr(&mut self) -> TXERRORINTCLR_W {
        TXERRORINTCLR_W { w: self }
    }
    #[doc = "Bit 6 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn txfinishedintclr(&mut self) -> TXFINISHEDINTCLR_W {
        TXFINISHEDINTCLR_W { w: self }
    }
    #[doc = "Bit 7 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn txdoneintclr(&mut self) -> TXDONEINTCLR_W {
        TXDONEINTCLR_W { w: self }
    }
    #[doc = "Bit 12 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn softintclr(&mut self) -> SOFTINTCLR_W {
        SOFTINTCLR_W { w: self }
    }
    #[doc = "Bit 13 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn wakeupintclr(&mut self) -> WAKEUPINTCLR_W {
        WAKEUPINTCLR_W { w: self }
    }
}
