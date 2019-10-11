#[doc = "Writer for register INTSET"]
pub type W = crate::W<u32, super::INTSET>;
#[doc = "Register INTSET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RXOVERRUNINTSET`"]
pub struct RXOVERRUNINTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVERRUNINTSET_W<'a> {
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
#[doc = "Write proxy for field `RXERRORINTSET`"]
pub struct RXERRORINTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> RXERRORINTSET_W<'a> {
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
#[doc = "Write proxy for field `RXFINISHEDINTSET`"]
pub struct RXFINISHEDINTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFINISHEDINTSET_W<'a> {
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
#[doc = "Write proxy for field `RXDONEINTSET`"]
pub struct RXDONEINTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDONEINTSET_W<'a> {
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
#[doc = "Write proxy for field `TXUNDERRUNINTSET`"]
pub struct TXUNDERRUNINTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUNDERRUNINTSET_W<'a> {
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
#[doc = "Write proxy for field `TXERRORINTSET`"]
pub struct TXERRORINTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> TXERRORINTSET_W<'a> {
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
#[doc = "Write proxy for field `TXFINISHEDINTSET`"]
pub struct TXFINISHEDINTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFINISHEDINTSET_W<'a> {
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
#[doc = "Write proxy for field `TXDONEINTSET`"]
pub struct TXDONEINTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDONEINTSET_W<'a> {
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
#[doc = "Write proxy for field `SOFTINTSET`"]
pub struct SOFTINTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTINTSET_W<'a> {
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
#[doc = "Write proxy for field `WAKEUPINTSET`"]
pub struct WAKEUPINTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPINTSET_W<'a> {
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
    #[doc = "Bit 0 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn rxoverrunintset(&mut self) -> RXOVERRUNINTSET_W {
        RXOVERRUNINTSET_W { w: self }
    }
    #[doc = "Bit 1 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn rxerrorintset(&mut self) -> RXERRORINTSET_W {
        RXERRORINTSET_W { w: self }
    }
    #[doc = "Bit 2 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn rxfinishedintset(&mut self) -> RXFINISHEDINTSET_W {
        RXFINISHEDINTSET_W { w: self }
    }
    #[doc = "Bit 3 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn rxdoneintset(&mut self) -> RXDONEINTSET_W {
        RXDONEINTSET_W { w: self }
    }
    #[doc = "Bit 4 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn txunderrunintset(&mut self) -> TXUNDERRUNINTSET_W {
        TXUNDERRUNINTSET_W { w: self }
    }
    #[doc = "Bit 5 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn txerrorintset(&mut self) -> TXERRORINTSET_W {
        TXERRORINTSET_W { w: self }
    }
    #[doc = "Bit 6 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn txfinishedintset(&mut self) -> TXFINISHEDINTSET_W {
        TXFINISHEDINTSET_W { w: self }
    }
    #[doc = "Bit 7 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn txdoneintset(&mut self) -> TXDONEINTSET_W {
        TXDONEINTSET_W { w: self }
    }
    #[doc = "Bit 12 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn softintset(&mut self) -> SOFTINTSET_W {
        SOFTINTSET_W { w: self }
    }
    #[doc = "Bit 13 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn wakeupintset(&mut self) -> WAKEUPINTSET_W {
        WAKEUPINTSET_W { w: self }
    }
}
