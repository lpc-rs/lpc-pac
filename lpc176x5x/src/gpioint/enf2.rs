#[doc = "Reader of register ENF2"]
pub type R = crate::R<u32, super::ENF2>;
#[doc = "Writer for register ENF2"]
pub type W = crate::W<u32, super::ENF2>;
#[doc = "Register ENF2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ENF2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P2_0EF`"]
pub type P2_0EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_0EF`"]
pub struct P2_0EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_0EF_W<'a> {
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
#[doc = "Reader of field `P2_1EF`"]
pub type P2_1EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_1EF`"]
pub struct P2_1EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_1EF_W<'a> {
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
#[doc = "Reader of field `P2_2EF`"]
pub type P2_2EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_2EF`"]
pub struct P2_2EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_2EF_W<'a> {
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
#[doc = "Reader of field `P2_3EF`"]
pub type P2_3EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_3EF`"]
pub struct P2_3EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_3EF_W<'a> {
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
#[doc = "Reader of field `P2_4EF`"]
pub type P2_4EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_4EF`"]
pub struct P2_4EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_4EF_W<'a> {
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
#[doc = "Reader of field `P2_5EF`"]
pub type P2_5EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_5EF`"]
pub struct P2_5EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_5EF_W<'a> {
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
#[doc = "Reader of field `P2_6EF`"]
pub type P2_6EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_6EF`"]
pub struct P2_6EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_6EF_W<'a> {
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
#[doc = "Reader of field `P2_7EF`"]
pub type P2_7EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_7EF`"]
pub struct P2_7EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_7EF_W<'a> {
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
#[doc = "Reader of field `P2_8EF`"]
pub type P2_8EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_8EF`"]
pub struct P2_8EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_8EF_W<'a> {
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
#[doc = "Reader of field `P2_9EF`"]
pub type P2_9EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_9EF`"]
pub struct P2_9EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_9EF_W<'a> {
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
#[doc = "Reader of field `P2_10EF`"]
pub type P2_10EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_10EF`"]
pub struct P2_10EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_10EF_W<'a> {
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
#[doc = "Reader of field `P2_11EF`"]
pub type P2_11EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_11EF`"]
pub struct P2_11EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_11EF_W<'a> {
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
#[doc = "Reader of field `P2_12EF`"]
pub type P2_12EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_12EF`"]
pub struct P2_12EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_12EF_W<'a> {
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
#[doc = "Reader of field `P2_13EF`"]
pub type P2_13EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_13EF`"]
pub struct P2_13EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_13EF_W<'a> {
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
    #[doc = "Bit 0 - Enable falling edge interrupt for P2\\[0\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_0ef(&self) -> P2_0EF_R {
        P2_0EF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable falling edge interrupt for P2\\[1\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_1ef(&self) -> P2_1EF_R {
        P2_1EF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable falling edge interrupt for P2\\[2\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_2ef(&self) -> P2_2EF_R {
        P2_2EF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable falling edge interrupt for P2\\[3\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_3ef(&self) -> P2_3EF_R {
        P2_3EF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable falling edge interrupt for P2\\[4\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_4ef(&self) -> P2_4EF_R {
        P2_4EF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable falling edge interrupt for P2\\[5\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_5ef(&self) -> P2_5EF_R {
        P2_5EF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable falling edge interrupt for P2\\[6\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_6ef(&self) -> P2_6EF_R {
        P2_6EF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable falling edge interrupt for P2\\[7\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_7ef(&self) -> P2_7EF_R {
        P2_7EF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable falling edge interrupt for P2\\[8\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_8ef(&self) -> P2_8EF_R {
        P2_8EF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable falling edge interrupt for P2\\[9\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_9ef(&self) -> P2_9EF_R {
        P2_9EF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable falling edge interrupt for P2\\[10\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_10ef(&self) -> P2_10EF_R {
        P2_10EF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable falling edge interrupt for P2\\[11\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_11ef(&self) -> P2_11EF_R {
        P2_11EF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable falling edge interrupt for P2\\[12\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_12ef(&self) -> P2_12EF_R {
        P2_12EF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable falling edge interrupt for P2\\[13\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_13ef(&self) -> P2_13EF_R {
        P2_13EF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable falling edge interrupt for P2\\[0\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_0ef(&mut self) -> P2_0EF_W {
        P2_0EF_W { w: self }
    }
    #[doc = "Bit 1 - Enable falling edge interrupt for P2\\[1\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_1ef(&mut self) -> P2_1EF_W {
        P2_1EF_W { w: self }
    }
    #[doc = "Bit 2 - Enable falling edge interrupt for P2\\[2\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_2ef(&mut self) -> P2_2EF_W {
        P2_2EF_W { w: self }
    }
    #[doc = "Bit 3 - Enable falling edge interrupt for P2\\[3\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_3ef(&mut self) -> P2_3EF_W {
        P2_3EF_W { w: self }
    }
    #[doc = "Bit 4 - Enable falling edge interrupt for P2\\[4\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_4ef(&mut self) -> P2_4EF_W {
        P2_4EF_W { w: self }
    }
    #[doc = "Bit 5 - Enable falling edge interrupt for P2\\[5\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_5ef(&mut self) -> P2_5EF_W {
        P2_5EF_W { w: self }
    }
    #[doc = "Bit 6 - Enable falling edge interrupt for P2\\[6\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_6ef(&mut self) -> P2_6EF_W {
        P2_6EF_W { w: self }
    }
    #[doc = "Bit 7 - Enable falling edge interrupt for P2\\[7\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_7ef(&mut self) -> P2_7EF_W {
        P2_7EF_W { w: self }
    }
    #[doc = "Bit 8 - Enable falling edge interrupt for P2\\[8\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_8ef(&mut self) -> P2_8EF_W {
        P2_8EF_W { w: self }
    }
    #[doc = "Bit 9 - Enable falling edge interrupt for P2\\[9\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_9ef(&mut self) -> P2_9EF_W {
        P2_9EF_W { w: self }
    }
    #[doc = "Bit 10 - Enable falling edge interrupt for P2\\[10\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_10ef(&mut self) -> P2_10EF_W {
        P2_10EF_W { w: self }
    }
    #[doc = "Bit 11 - Enable falling edge interrupt for P2\\[11\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_11ef(&mut self) -> P2_11EF_W {
        P2_11EF_W { w: self }
    }
    #[doc = "Bit 12 - Enable falling edge interrupt for P2\\[12\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_12ef(&mut self) -> P2_12EF_W {
        P2_12EF_W { w: self }
    }
    #[doc = "Bit 13 - Enable falling edge interrupt for P2\\[13\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_13ef(&mut self) -> P2_13EF_W {
        P2_13EF_W { w: self }
    }
}
