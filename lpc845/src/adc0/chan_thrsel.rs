#[doc = "Reader of register CHAN_THRSEL"]
pub type R = crate::R<u32, super::CHAN_THRSEL>;
#[doc = "Writer for register CHAN_THRSEL"]
pub type W = crate::W<u32, super::CHAN_THRSEL>;
#[doc = "Register CHAN_THRSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::CHAN_THRSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Threshold select for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0_THRSEL_A {
    #[doc = "0: Threshold 0. Results for this channel will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers."]
    THRESHOLD0 = 0,
    #[doc = "1: Threshold 1. Results for this channel will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers."]
    THRESHOLD1 = 1,
}
impl From<CH0_THRSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH0_THRSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH0_THRSEL`"]
pub type CH0_THRSEL_R = crate::R<bool, CH0_THRSEL_A>;
impl CH0_THRSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0_THRSEL_A {
        match self.bits {
            false => CH0_THRSEL_A::THRESHOLD0,
            true => CH0_THRSEL_A::THRESHOLD1,
        }
    }
    #[doc = "Checks if the value of the field is `THRESHOLD0`"]
    #[inline(always)]
    pub fn is_threshold0(&self) -> bool {
        *self == CH0_THRSEL_A::THRESHOLD0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD1`"]
    #[inline(always)]
    pub fn is_threshold1(&self) -> bool {
        *self == CH0_THRSEL_A::THRESHOLD1
    }
}
#[doc = "Write proxy for field `CH0_THRSEL`"]
pub struct CH0_THRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_THRSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0_THRSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Threshold 0. Results for this channel will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers."]
    #[inline(always)]
    pub fn threshold0(self) -> &'a mut W {
        self.variant(CH0_THRSEL_A::THRESHOLD0)
    }
    #[doc = "Threshold 1. Results for this channel will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers."]
    #[inline(always)]
    pub fn threshold1(self) -> &'a mut W {
        self.variant(CH0_THRSEL_A::THRESHOLD1)
    }
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
#[doc = "Reader of field `CH1_THRSEL`"]
pub type CH1_THRSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1_THRSEL`"]
pub struct CH1_THRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_THRSEL_W<'a> {
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
#[doc = "Reader of field `CH2_THRSEL`"]
pub type CH2_THRSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2_THRSEL`"]
pub struct CH2_THRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_THRSEL_W<'a> {
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
#[doc = "Reader of field `CH3_THRSEL`"]
pub type CH3_THRSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3_THRSEL`"]
pub struct CH3_THRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_THRSEL_W<'a> {
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
#[doc = "Reader of field `CH4_THRSEL`"]
pub type CH4_THRSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH4_THRSEL`"]
pub struct CH4_THRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4_THRSEL_W<'a> {
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
#[doc = "Reader of field `CH5_THRSEL`"]
pub type CH5_THRSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH5_THRSEL`"]
pub struct CH5_THRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5_THRSEL_W<'a> {
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
#[doc = "Reader of field `CH6_THRSEL`"]
pub type CH6_THRSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH6_THRSEL`"]
pub struct CH6_THRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6_THRSEL_W<'a> {
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
#[doc = "Reader of field `CH7_THRSEL`"]
pub type CH7_THRSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH7_THRSEL`"]
pub struct CH7_THRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7_THRSEL_W<'a> {
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
#[doc = "Reader of field `CH8_THRSEL`"]
pub type CH8_THRSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH8_THRSEL`"]
pub struct CH8_THRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH8_THRSEL_W<'a> {
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
#[doc = "Reader of field `CH9_THRSEL`"]
pub type CH9_THRSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH9_THRSEL`"]
pub struct CH9_THRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH9_THRSEL_W<'a> {
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
#[doc = "Reader of field `CH10_THRSEL`"]
pub type CH10_THRSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH10_THRSEL`"]
pub struct CH10_THRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH10_THRSEL_W<'a> {
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
#[doc = "Reader of field `CH11_THRSEL`"]
pub type CH11_THRSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH11_THRSEL`"]
pub struct CH11_THRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH11_THRSEL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Threshold select for channel 0."]
    #[inline(always)]
    pub fn ch0_thrsel(&self) -> CH0_THRSEL_R {
        CH0_THRSEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Threshold select for channel 1. See description for channel 0."]
    #[inline(always)]
    pub fn ch1_thrsel(&self) -> CH1_THRSEL_R {
        CH1_THRSEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Threshold select for channel 2. See description for channel 0."]
    #[inline(always)]
    pub fn ch2_thrsel(&self) -> CH2_THRSEL_R {
        CH2_THRSEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Threshold select for channel 3. See description for channel 0."]
    #[inline(always)]
    pub fn ch3_thrsel(&self) -> CH3_THRSEL_R {
        CH3_THRSEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Threshold select for channel 4. See description for channel 0."]
    #[inline(always)]
    pub fn ch4_thrsel(&self) -> CH4_THRSEL_R {
        CH4_THRSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Threshold select for channel 5. See description for channel 0."]
    #[inline(always)]
    pub fn ch5_thrsel(&self) -> CH5_THRSEL_R {
        CH5_THRSEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Threshold select for channel 6. See description for channel 0."]
    #[inline(always)]
    pub fn ch6_thrsel(&self) -> CH6_THRSEL_R {
        CH6_THRSEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Threshold select for channel 7. See description for channel 0."]
    #[inline(always)]
    pub fn ch7_thrsel(&self) -> CH7_THRSEL_R {
        CH7_THRSEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Threshold select for channel 8. See description for channel 0."]
    #[inline(always)]
    pub fn ch8_thrsel(&self) -> CH8_THRSEL_R {
        CH8_THRSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Threshold select for channel 9. See description for channel 0."]
    #[inline(always)]
    pub fn ch9_thrsel(&self) -> CH9_THRSEL_R {
        CH9_THRSEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Threshold select for channel 10. See description for channel 0."]
    #[inline(always)]
    pub fn ch10_thrsel(&self) -> CH10_THRSEL_R {
        CH10_THRSEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Threshold select for channel 11. See description for channel 0."]
    #[inline(always)]
    pub fn ch11_thrsel(&self) -> CH11_THRSEL_R {
        CH11_THRSEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Threshold select for channel 0."]
    #[inline(always)]
    pub fn ch0_thrsel(&mut self) -> CH0_THRSEL_W {
        CH0_THRSEL_W { w: self }
    }
    #[doc = "Bit 1 - Threshold select for channel 1. See description for channel 0."]
    #[inline(always)]
    pub fn ch1_thrsel(&mut self) -> CH1_THRSEL_W {
        CH1_THRSEL_W { w: self }
    }
    #[doc = "Bit 2 - Threshold select for channel 2. See description for channel 0."]
    #[inline(always)]
    pub fn ch2_thrsel(&mut self) -> CH2_THRSEL_W {
        CH2_THRSEL_W { w: self }
    }
    #[doc = "Bit 3 - Threshold select for channel 3. See description for channel 0."]
    #[inline(always)]
    pub fn ch3_thrsel(&mut self) -> CH3_THRSEL_W {
        CH3_THRSEL_W { w: self }
    }
    #[doc = "Bit 4 - Threshold select for channel 4. See description for channel 0."]
    #[inline(always)]
    pub fn ch4_thrsel(&mut self) -> CH4_THRSEL_W {
        CH4_THRSEL_W { w: self }
    }
    #[doc = "Bit 5 - Threshold select for channel 5. See description for channel 0."]
    #[inline(always)]
    pub fn ch5_thrsel(&mut self) -> CH5_THRSEL_W {
        CH5_THRSEL_W { w: self }
    }
    #[doc = "Bit 6 - Threshold select for channel 6. See description for channel 0."]
    #[inline(always)]
    pub fn ch6_thrsel(&mut self) -> CH6_THRSEL_W {
        CH6_THRSEL_W { w: self }
    }
    #[doc = "Bit 7 - Threshold select for channel 7. See description for channel 0."]
    #[inline(always)]
    pub fn ch7_thrsel(&mut self) -> CH7_THRSEL_W {
        CH7_THRSEL_W { w: self }
    }
    #[doc = "Bit 8 - Threshold select for channel 8. See description for channel 0."]
    #[inline(always)]
    pub fn ch8_thrsel(&mut self) -> CH8_THRSEL_W {
        CH8_THRSEL_W { w: self }
    }
    #[doc = "Bit 9 - Threshold select for channel 9. See description for channel 0."]
    #[inline(always)]
    pub fn ch9_thrsel(&mut self) -> CH9_THRSEL_W {
        CH9_THRSEL_W { w: self }
    }
    #[doc = "Bit 10 - Threshold select for channel 10. See description for channel 0."]
    #[inline(always)]
    pub fn ch10_thrsel(&mut self) -> CH10_THRSEL_W {
        CH10_THRSEL_W { w: self }
    }
    #[doc = "Bit 11 - Threshold select for channel 11. See description for channel 0."]
    #[inline(always)]
    pub fn ch11_thrsel(&mut self) -> CH11_THRSEL_W {
        CH11_THRSEL_W { w: self }
    }
}
