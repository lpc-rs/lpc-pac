#[doc = "Reader of register INTENABLE"]
pub type R = crate::R<u32, super::INTENABLE>;
#[doc = "Writer for register INTENABLE"]
pub type W = crate::W<u32, super::INTENABLE>;
#[doc = "Register INTENABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXOVERRUNINTEN`"]
pub type RXOVERRUNINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXOVERRUNINTEN`"]
pub struct RXOVERRUNINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVERRUNINTEN_W<'a> {
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
#[doc = "Reader of field `RXERRORINTEN`"]
pub type RXERRORINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXERRORINTEN`"]
pub struct RXERRORINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXERRORINTEN_W<'a> {
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
#[doc = "Reader of field `RXFINISHEDINTEN`"]
pub type RXFINISHEDINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFINISHEDINTEN`"]
pub struct RXFINISHEDINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFINISHEDINTEN_W<'a> {
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
#[doc = "Reader of field `RXDONEINTEN`"]
pub type RXDONEINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXDONEINTEN`"]
pub struct RXDONEINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDONEINTEN_W<'a> {
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
#[doc = "Reader of field `TXUNDERRUNINTEN`"]
pub type TXUNDERRUNINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXUNDERRUNINTEN`"]
pub struct TXUNDERRUNINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUNDERRUNINTEN_W<'a> {
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
#[doc = "Reader of field `TXERRORINTEN`"]
pub type TXERRORINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXERRORINTEN`"]
pub struct TXERRORINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXERRORINTEN_W<'a> {
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
#[doc = "Reader of field `TXFINISHEDINTEN`"]
pub type TXFINISHEDINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFINISHEDINTEN`"]
pub struct TXFINISHEDINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFINISHEDINTEN_W<'a> {
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
#[doc = "Reader of field `TXDONEINTEN`"]
pub type TXDONEINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXDONEINTEN`"]
pub struct TXDONEINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDONEINTEN_W<'a> {
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
#[doc = "Reader of field `SOFTINTEN`"]
pub type SOFTINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTINTEN`"]
pub struct SOFTINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTINTEN_W<'a> {
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
#[doc = "Reader of field `WAKEUPINTEN`"]
pub type WAKEUPINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEUPINTEN`"]
pub struct WAKEUPINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPINTEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Enable for interrupt trigger on receive buffer overrun or descriptor underrun situations."]
    #[inline(always)]
    pub fn rxoverruninten(&self) -> RXOVERRUNINTEN_R {
        RXOVERRUNINTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable for interrupt trigger on receive errors."]
    #[inline(always)]
    pub fn rxerrorinten(&self) -> RXERRORINTEN_R {
        RXERRORINTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable for interrupt triggered when all receive descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    pub fn rxfinishedinten(&self) -> RXFINISHEDINTEN_R {
        RXFINISHEDINTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable for interrupt triggered when a receive descriptor has been processed while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    pub fn rxdoneinten(&self) -> RXDONEINTEN_R {
        RXDONEINTEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable for interrupt trigger on transmit buffer or descriptor underrun situations."]
    #[inline(always)]
    pub fn txunderruninten(&self) -> TXUNDERRUNINTEN_R {
        TXUNDERRUNINTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable for interrupt trigger on transmit errors."]
    #[inline(always)]
    pub fn txerrorinten(&self) -> TXERRORINTEN_R {
        TXERRORINTEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable for interrupt triggered when all transmit descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    pub fn txfinishedinten(&self) -> TXFINISHEDINTEN_R {
        TXFINISHEDINTEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable for interrupt triggered when a descriptor has been transmitted while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    pub fn txdoneinten(&self) -> TXDONEINTEN_R {
        TXDONEINTEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable for interrupt triggered by the SoftInt bit in the IntStatus register, caused by software writing a 1 to the SoftIntSet bit in the IntSet register."]
    #[inline(always)]
    pub fn softinten(&self) -> SOFTINTEN_R {
        SOFTINTEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable for interrupt triggered by a Wake-up event detected by the receive filter."]
    #[inline(always)]
    pub fn wakeupinten(&self) -> WAKEUPINTEN_R {
        WAKEUPINTEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable for interrupt trigger on receive buffer overrun or descriptor underrun situations."]
    #[inline(always)]
    pub fn rxoverruninten(&mut self) -> RXOVERRUNINTEN_W {
        RXOVERRUNINTEN_W { w: self }
    }
    #[doc = "Bit 1 - Enable for interrupt trigger on receive errors."]
    #[inline(always)]
    pub fn rxerrorinten(&mut self) -> RXERRORINTEN_W {
        RXERRORINTEN_W { w: self }
    }
    #[doc = "Bit 2 - Enable for interrupt triggered when all receive descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    pub fn rxfinishedinten(&mut self) -> RXFINISHEDINTEN_W {
        RXFINISHEDINTEN_W { w: self }
    }
    #[doc = "Bit 3 - Enable for interrupt triggered when a receive descriptor has been processed while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    pub fn rxdoneinten(&mut self) -> RXDONEINTEN_W {
        RXDONEINTEN_W { w: self }
    }
    #[doc = "Bit 4 - Enable for interrupt trigger on transmit buffer or descriptor underrun situations."]
    #[inline(always)]
    pub fn txunderruninten(&mut self) -> TXUNDERRUNINTEN_W {
        TXUNDERRUNINTEN_W { w: self }
    }
    #[doc = "Bit 5 - Enable for interrupt trigger on transmit errors."]
    #[inline(always)]
    pub fn txerrorinten(&mut self) -> TXERRORINTEN_W {
        TXERRORINTEN_W { w: self }
    }
    #[doc = "Bit 6 - Enable for interrupt triggered when all transmit descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    pub fn txfinishedinten(&mut self) -> TXFINISHEDINTEN_W {
        TXFINISHEDINTEN_W { w: self }
    }
    #[doc = "Bit 7 - Enable for interrupt triggered when a descriptor has been transmitted while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    pub fn txdoneinten(&mut self) -> TXDONEINTEN_W {
        TXDONEINTEN_W { w: self }
    }
    #[doc = "Bit 12 - Enable for interrupt triggered by the SoftInt bit in the IntStatus register, caused by software writing a 1 to the SoftIntSet bit in the IntSet register."]
    #[inline(always)]
    pub fn softinten(&mut self) -> SOFTINTEN_W {
        SOFTINTEN_W { w: self }
    }
    #[doc = "Bit 13 - Enable for interrupt triggered by a Wake-up event detected by the receive filter."]
    #[inline(always)]
    pub fn wakeupinten(&mut self) -> WAKEUPINTEN_W {
        WAKEUPINTEN_W { w: self }
    }
}
