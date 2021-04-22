#[doc = "Register `CHAN_THRSEL` reader"]
pub struct R(crate::R<CHAN_THRSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHAN_THRSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CHAN_THRSEL_SPEC>> for R {
    fn from(reader: crate::R<CHAN_THRSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHAN_THRSEL` writer"]
pub struct W(crate::W<CHAN_THRSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHAN_THRSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<CHAN_THRSEL_SPEC>> for W {
    fn from(writer: crate::W<CHAN_THRSEL_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `CH0_THRSEL` reader - Threshold select for channel 0."]
pub struct CH0_THRSEL_R(crate::FieldReader<bool, CH0_THRSEL_A>);
impl CH0_THRSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH0_THRSEL_R(crate::FieldReader::new(bits))
    }
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
        **self == CH0_THRSEL_A::THRESHOLD0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD1`"]
    #[inline(always)]
    pub fn is_threshold1(&self) -> bool {
        **self == CH0_THRSEL_A::THRESHOLD1
    }
}
impl core::ops::Deref for CH0_THRSEL_R {
    type Target = crate::FieldReader<bool, CH0_THRSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_THRSEL` writer - Threshold select for channel 0."]
pub struct CH0_THRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_THRSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0_THRSEL_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `CH1_THRSEL` reader - Threshold select for channel 1. See description for channel 0."]
pub struct CH1_THRSEL_R(crate::FieldReader<bool, bool>);
impl CH1_THRSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1_THRSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_THRSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_THRSEL` writer - Threshold select for channel 1. See description for channel 0."]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CH2_THRSEL` reader - Threshold select for channel 2. See description for channel 0."]
pub struct CH2_THRSEL_R(crate::FieldReader<bool, bool>);
impl CH2_THRSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH2_THRSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_THRSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2_THRSEL` writer - Threshold select for channel 2. See description for channel 0."]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CH3_THRSEL` reader - Threshold select for channel 3. See description for channel 0."]
pub struct CH3_THRSEL_R(crate::FieldReader<bool, bool>);
impl CH3_THRSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH3_THRSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_THRSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3_THRSEL` writer - Threshold select for channel 3. See description for channel 0."]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CH4_THRSEL` reader - Threshold select for channel 4. See description for channel 0."]
pub struct CH4_THRSEL_R(crate::FieldReader<bool, bool>);
impl CH4_THRSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH4_THRSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH4_THRSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH4_THRSEL` writer - Threshold select for channel 4. See description for channel 0."]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CH5_THRSEL` reader - Threshold select for channel 5. See description for channel 0."]
pub struct CH5_THRSEL_R(crate::FieldReader<bool, bool>);
impl CH5_THRSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH5_THRSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH5_THRSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH5_THRSEL` writer - Threshold select for channel 5. See description for channel 0."]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `CH6_THRSEL` reader - Threshold select for channel 6. See description for channel 0."]
pub struct CH6_THRSEL_R(crate::FieldReader<bool, bool>);
impl CH6_THRSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH6_THRSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH6_THRSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH6_THRSEL` writer - Threshold select for channel 6. See description for channel 0."]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `CH7_THRSEL` reader - Threshold select for channel 7. See description for channel 0."]
pub struct CH7_THRSEL_R(crate::FieldReader<bool, bool>);
impl CH7_THRSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH7_THRSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH7_THRSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH7_THRSEL` writer - Threshold select for channel 7. See description for channel 0."]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `CH8_THRSEL` reader - Threshold select for channel 8. See description for channel 0."]
pub struct CH8_THRSEL_R(crate::FieldReader<bool, bool>);
impl CH8_THRSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH8_THRSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH8_THRSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH8_THRSEL` writer - Threshold select for channel 8. See description for channel 0."]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `CH9_THRSEL` reader - Threshold select for channel 9. See description for channel 0."]
pub struct CH9_THRSEL_R(crate::FieldReader<bool, bool>);
impl CH9_THRSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH9_THRSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH9_THRSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH9_THRSEL` writer - Threshold select for channel 9. See description for channel 0."]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CH10_THRSEL` reader - Threshold select for channel 10. See description for channel 0."]
pub struct CH10_THRSEL_R(crate::FieldReader<bool, bool>);
impl CH10_THRSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH10_THRSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH10_THRSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH10_THRSEL` writer - Threshold select for channel 10. See description for channel 0."]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `CH11_THRSEL` reader - Threshold select for channel 11. See description for channel 0."]
pub struct CH11_THRSEL_R(crate::FieldReader<bool, bool>);
impl CH11_THRSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH11_THRSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH11_THRSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH11_THRSEL` writer - Threshold select for channel 11. See description for channel 0."]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
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
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Channel-Threshold Select register. Specifies which set of threshold compare registers are to be used for each channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chan_thrsel](index.html) module"]
pub struct CHAN_THRSEL_SPEC;
impl crate::RegisterSpec for CHAN_THRSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chan_thrsel::R](R) reader structure"]
impl crate::Readable for CHAN_THRSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chan_thrsel::W](W) writer structure"]
impl crate::Writable for CHAN_THRSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHAN_THRSEL to value 0"]
impl crate::Resettable for CHAN_THRSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
