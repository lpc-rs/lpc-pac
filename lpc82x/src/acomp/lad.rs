#[doc = "Register `LAD` reader"]
pub struct R(crate::R<LAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LAD_SPEC>> for R {
    fn from(reader: crate::R<LAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAD` writer"]
pub struct W(crate::W<LAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAD_SPEC>;
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
impl core::convert::From<crate::W<LAD_SPEC>> for W {
    fn from(writer: crate::W<LAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LADEN` reader - Voltage ladder enable"]
pub struct LADEN_R(crate::FieldReader<bool, bool>);
impl LADEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LADEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LADEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LADEN` writer - Voltage ladder enable"]
pub struct LADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LADEN_W<'a> {
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
#[doc = "Field `LADSEL` reader - Voltage ladder value. The reference voltage Vref depends on the LADREF bit below. 00000 = VSS 00001 = 1 x Vref/31 00010 = 2 x Vref/31 ... 11111 = Vref"]
pub struct LADSEL_R(crate::FieldReader<u8, u8>);
impl LADSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LADSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LADSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LADSEL` writer - Voltage ladder value. The reference voltage Vref depends on the LADREF bit below. 00000 = VSS 00001 = 1 x Vref/31 00010 = 2 x Vref/31 ... 11111 = Vref"]
pub struct LADSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LADSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | ((value as u32 & 0x1f) << 1);
        self.w
    }
}
#[doc = "Selects the reference voltage Vref for the voltage ladder.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LADREF_A {
    #[doc = "0: Supply pin VDD"]
    LADREF_0 = 0,
    #[doc = "1: VDDCMP pin"]
    LADREF_1 = 1,
}
impl From<LADREF_A> for bool {
    #[inline(always)]
    fn from(variant: LADREF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LADREF` reader - Selects the reference voltage Vref for the voltage ladder."]
pub struct LADREF_R(crate::FieldReader<bool, LADREF_A>);
impl LADREF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LADREF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LADREF_A {
        match self.bits {
            false => LADREF_A::LADREF_0,
            true => LADREF_A::LADREF_1,
        }
    }
    #[doc = "Checks if the value of the field is `LADREF_0`"]
    #[inline(always)]
    pub fn is_ladref_0(&self) -> bool {
        **self == LADREF_A::LADREF_0
    }
    #[doc = "Checks if the value of the field is `LADREF_1`"]
    #[inline(always)]
    pub fn is_ladref_1(&self) -> bool {
        **self == LADREF_A::LADREF_1
    }
}
impl core::ops::Deref for LADREF_R {
    type Target = crate::FieldReader<bool, LADREF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LADREF` writer - Selects the reference voltage Vref for the voltage ladder."]
pub struct LADREF_W<'a> {
    w: &'a mut W,
}
impl<'a> LADREF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LADREF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Supply pin VDD"]
    #[inline(always)]
    pub fn ladref_0(self) -> &'a mut W {
        self.variant(LADREF_A::LADREF_0)
    }
    #[doc = "VDDCMP pin"]
    #[inline(always)]
    pub fn ladref_1(self) -> &'a mut W {
        self.variant(LADREF_A::LADREF_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Voltage ladder enable"]
    #[inline(always)]
    pub fn laden(&self) -> LADEN_R {
        LADEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:5 - Voltage ladder value. The reference voltage Vref depends on the LADREF bit below. 00000 = VSS 00001 = 1 x Vref/31 00010 = 2 x Vref/31 ... 11111 = Vref"]
    #[inline(always)]
    pub fn ladsel(&self) -> LADSEL_R {
        LADSEL_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Selects the reference voltage Vref for the voltage ladder."]
    #[inline(always)]
    pub fn ladref(&self) -> LADREF_R {
        LADREF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage ladder enable"]
    #[inline(always)]
    pub fn laden(&mut self) -> LADEN_W {
        LADEN_W { w: self }
    }
    #[doc = "Bits 1:5 - Voltage ladder value. The reference voltage Vref depends on the LADREF bit below. 00000 = VSS 00001 = 1 x Vref/31 00010 = 2 x Vref/31 ... 11111 = Vref"]
    #[inline(always)]
    pub fn ladsel(&mut self) -> LADSEL_W {
        LADSEL_W { w: self }
    }
    #[doc = "Bit 6 - Selects the reference voltage Vref for the voltage ladder."]
    #[inline(always)]
    pub fn ladref(&mut self) -> LADREF_W {
        LADREF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Voltage ladder register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lad](index.html) module"]
pub struct LAD_SPEC;
impl crate::RegisterSpec for LAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lad::R](R) reader structure"]
impl crate::Readable for LAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lad::W](W) writer structure"]
impl crate::Writable for LAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LAD to value 0"]
impl crate::Resettable for LAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
