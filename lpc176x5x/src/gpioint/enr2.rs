#[doc = "Reader of register ENR2"]
pub type R = crate::R<u32, super::ENR2>;
#[doc = "Writer for register ENR2"]
pub type W = crate::W<u32, super::ENR2>;
#[doc = "Register ENR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ENR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P2_0ER`"]
pub type P2_0ER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_0ER`"]
pub struct P2_0ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_0ER_W<'a> {
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
#[doc = "Reader of field `P2_1ER`"]
pub type P2_1ER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_1ER`"]
pub struct P2_1ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_1ER_W<'a> {
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
#[doc = "Reader of field `P2_2ER`"]
pub type P2_2ER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_2ER`"]
pub struct P2_2ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_2ER_W<'a> {
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
#[doc = "Reader of field `P2_3ER`"]
pub type P2_3ER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_3ER`"]
pub struct P2_3ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_3ER_W<'a> {
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
#[doc = "Reader of field `P2_4ER`"]
pub type P2_4ER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_4ER`"]
pub struct P2_4ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_4ER_W<'a> {
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
#[doc = "Reader of field `P2_5ER`"]
pub type P2_5ER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_5ER`"]
pub struct P2_5ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_5ER_W<'a> {
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
#[doc = "Reader of field `P2_6ER`"]
pub type P2_6ER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_6ER`"]
pub struct P2_6ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_6ER_W<'a> {
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
#[doc = "Reader of field `P2_7ER`"]
pub type P2_7ER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_7ER`"]
pub struct P2_7ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_7ER_W<'a> {
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
#[doc = "Reader of field `P2_8ER`"]
pub type P2_8ER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_8ER`"]
pub struct P2_8ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_8ER_W<'a> {
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
#[doc = "Reader of field `P2_9ER`"]
pub type P2_9ER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_9ER`"]
pub struct P2_9ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_9ER_W<'a> {
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
#[doc = "Reader of field `P2_10ER`"]
pub type P2_10ER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_10ER`"]
pub struct P2_10ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_10ER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `P2_11ER`"]
pub type P2_11ER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_11ER`"]
pub struct P2_11ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_11ER_W<'a> {
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
#[doc = "Reader of field `P2_12ER`"]
pub type P2_12ER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_12ER`"]
pub struct P2_12ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_12ER_W<'a> {
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
#[doc = "Reader of field `P2_13ER`"]
pub type P2_13ER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_13ER`"]
pub struct P2_13ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_13ER_W<'a> {
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
    #[doc = "Bit 0 - Enable rising edge interrupt for P2\\[0\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_0er(&self) -> P2_0ER_R {
        P2_0ER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable rising edge interrupt for P2\\[1\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_1er(&self) -> P2_1ER_R {
        P2_1ER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable rising edge interrupt for P2\\[2\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_2er(&self) -> P2_2ER_R {
        P2_2ER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable rising edge interrupt for P2\\[3\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_3er(&self) -> P2_3ER_R {
        P2_3ER_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable rising edge interrupt for P2\\[4\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_4er(&self) -> P2_4ER_R {
        P2_4ER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable rising edge interrupt for P2\\[5\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_5er(&self) -> P2_5ER_R {
        P2_5ER_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable rising edge interrupt for P2\\[6\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_6er(&self) -> P2_6ER_R {
        P2_6ER_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable rising edge interrupt for P2\\[7\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_7er(&self) -> P2_7ER_R {
        P2_7ER_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable rising edge interrupt for P2\\[8\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_8er(&self) -> P2_8ER_R {
        P2_8ER_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable rising edge interrupt for P2\\[9\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_9er(&self) -> P2_9ER_R {
        P2_9ER_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable rising edge interrupt for P2\\[10\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_10er(&self) -> P2_10ER_R {
        P2_10ER_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable rising edge interrupt for P2\\[11\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_11er(&self) -> P2_11ER_R {
        P2_11ER_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable rising edge interrupt for P2\\[12\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_12er(&self) -> P2_12ER_R {
        P2_12ER_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable rising edge interrupt for P2\\[13\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_13er(&self) -> P2_13ER_R {
        P2_13ER_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable rising edge interrupt for P2\\[0\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_0er(&mut self) -> P2_0ER_W {
        P2_0ER_W { w: self }
    }
    #[doc = "Bit 1 - Enable rising edge interrupt for P2\\[1\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_1er(&mut self) -> P2_1ER_W {
        P2_1ER_W { w: self }
    }
    #[doc = "Bit 2 - Enable rising edge interrupt for P2\\[2\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_2er(&mut self) -> P2_2ER_W {
        P2_2ER_W { w: self }
    }
    #[doc = "Bit 3 - Enable rising edge interrupt for P2\\[3\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_3er(&mut self) -> P2_3ER_W {
        P2_3ER_W { w: self }
    }
    #[doc = "Bit 4 - Enable rising edge interrupt for P2\\[4\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_4er(&mut self) -> P2_4ER_W {
        P2_4ER_W { w: self }
    }
    #[doc = "Bit 5 - Enable rising edge interrupt for P2\\[5\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_5er(&mut self) -> P2_5ER_W {
        P2_5ER_W { w: self }
    }
    #[doc = "Bit 6 - Enable rising edge interrupt for P2\\[6\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_6er(&mut self) -> P2_6ER_W {
        P2_6ER_W { w: self }
    }
    #[doc = "Bit 7 - Enable rising edge interrupt for P2\\[7\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_7er(&mut self) -> P2_7ER_W {
        P2_7ER_W { w: self }
    }
    #[doc = "Bit 8 - Enable rising edge interrupt for P2\\[8\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_8er(&mut self) -> P2_8ER_W {
        P2_8ER_W { w: self }
    }
    #[doc = "Bit 9 - Enable rising edge interrupt for P2\\[9\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_9er(&mut self) -> P2_9ER_W {
        P2_9ER_W { w: self }
    }
    #[doc = "Bit 10 - Enable rising edge interrupt for P2\\[10\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_10er(&mut self) -> P2_10ER_W {
        P2_10ER_W { w: self }
    }
    #[doc = "Bit 11 - Enable rising edge interrupt for P2\\[11\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_11er(&mut self) -> P2_11ER_W {
        P2_11ER_W { w: self }
    }
    #[doc = "Bit 12 - Enable rising edge interrupt for P2\\[12\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_12er(&mut self) -> P2_12ER_W {
        P2_12ER_W { w: self }
    }
    #[doc = "Bit 13 - Enable rising edge interrupt for P2\\[13\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_13er(&mut self) -> P2_13ER_W {
        P2_13ER_W { w: self }
    }
}
