#[doc = "Register `BODCTRL` reader"]
pub struct R(crate::R<BODCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BODCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BODCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BODCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BODCTRL` writer"]
pub struct W(crate::W<BODCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BODCTRL_SPEC>;
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
impl From<crate::W<BODCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BODCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "BOD reset level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BODRSTLEV_A {
    #[doc = "0: Level 0: 1.5 V"]
    LEVEL0 = 0,
    #[doc = "1: Level 1: 1.85 V"]
    LEVEL1 = 1,
    #[doc = "2: Level 2: 2.0 V"]
    LEVEL2 = 2,
    #[doc = "3: Level 3: 2.3 V"]
    LEVEL3 = 3,
}
impl From<BODRSTLEV_A> for u8 {
    #[inline(always)]
    fn from(variant: BODRSTLEV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BODRSTLEV` reader - BOD reset level"]
pub struct BODRSTLEV_R(crate::FieldReader<u8, BODRSTLEV_A>);
impl BODRSTLEV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BODRSTLEV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODRSTLEV_A {
        match self.bits {
            0 => BODRSTLEV_A::LEVEL0,
            1 => BODRSTLEV_A::LEVEL1,
            2 => BODRSTLEV_A::LEVEL2,
            3 => BODRSTLEV_A::LEVEL3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL0`"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        **self == BODRSTLEV_A::LEVEL0
    }
    #[doc = "Checks if the value of the field is `LEVEL1`"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        **self == BODRSTLEV_A::LEVEL1
    }
    #[doc = "Checks if the value of the field is `LEVEL2`"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        **self == BODRSTLEV_A::LEVEL2
    }
    #[doc = "Checks if the value of the field is `LEVEL3`"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        **self == BODRSTLEV_A::LEVEL3
    }
}
impl core::ops::Deref for BODRSTLEV_R {
    type Target = crate::FieldReader<u8, BODRSTLEV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODRSTLEV` writer - BOD reset level"]
pub struct BODRSTLEV_W<'a> {
    w: &'a mut W,
}
impl<'a> BODRSTLEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODRSTLEV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Level 0: 1.5 V"]
    #[inline(always)]
    pub fn level0(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::LEVEL0)
    }
    #[doc = "Level 1: 1.85 V"]
    #[inline(always)]
    pub fn level1(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::LEVEL1)
    }
    #[doc = "Level 2: 2.0 V"]
    #[inline(always)]
    pub fn level2(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::LEVEL2)
    }
    #[doc = "Level 3: 2.3 V"]
    #[inline(always)]
    pub fn level3(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::LEVEL3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "BOD reset enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRSTENA_A {
    #[doc = "0: Disable reset function."]
    DISABLE = 0,
    #[doc = "1: Enable reset function."]
    ENABLE = 1,
}
impl From<BODRSTENA_A> for bool {
    #[inline(always)]
    fn from(variant: BODRSTENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODRSTENA` reader - BOD reset enable"]
pub struct BODRSTENA_R(crate::FieldReader<bool, BODRSTENA_A>);
impl BODRSTENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BODRSTENA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODRSTENA_A {
        match self.bits {
            false => BODRSTENA_A::DISABLE,
            true => BODRSTENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == BODRSTENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == BODRSTENA_A::ENABLE
    }
}
impl core::ops::Deref for BODRSTENA_R {
    type Target = crate::FieldReader<bool, BODRSTENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODRSTENA` writer - BOD reset enable"]
pub struct BODRSTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> BODRSTENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODRSTENA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable reset function."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODRSTENA_A::DISABLE)
    }
    #[doc = "Enable reset function."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODRSTENA_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "BOD interrupt level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BODINTLEV_A {
    #[doc = "0: Level 0: 2.05 V"]
    LEVEL0 = 0,
    #[doc = "1: Level 1: 2.45 V"]
    LEVEL1 = 1,
    #[doc = "2: Level 2: 2.75 V"]
    LEVEL2 = 2,
    #[doc = "3: Level 3: 3.05 V"]
    LEVEL3 = 3,
}
impl From<BODINTLEV_A> for u8 {
    #[inline(always)]
    fn from(variant: BODINTLEV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BODINTLEV` reader - BOD interrupt level"]
pub struct BODINTLEV_R(crate::FieldReader<u8, BODINTLEV_A>);
impl BODINTLEV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BODINTLEV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODINTLEV_A {
        match self.bits {
            0 => BODINTLEV_A::LEVEL0,
            1 => BODINTLEV_A::LEVEL1,
            2 => BODINTLEV_A::LEVEL2,
            3 => BODINTLEV_A::LEVEL3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL0`"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        **self == BODINTLEV_A::LEVEL0
    }
    #[doc = "Checks if the value of the field is `LEVEL1`"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        **self == BODINTLEV_A::LEVEL1
    }
    #[doc = "Checks if the value of the field is `LEVEL2`"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        **self == BODINTLEV_A::LEVEL2
    }
    #[doc = "Checks if the value of the field is `LEVEL3`"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        **self == BODINTLEV_A::LEVEL3
    }
}
impl core::ops::Deref for BODINTLEV_R {
    type Target = crate::FieldReader<u8, BODINTLEV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODINTLEV` writer - BOD interrupt level"]
pub struct BODINTLEV_W<'a> {
    w: &'a mut W,
}
impl<'a> BODINTLEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODINTLEV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Level 0: 2.05 V"]
    #[inline(always)]
    pub fn level0(self) -> &'a mut W {
        self.variant(BODINTLEV_A::LEVEL0)
    }
    #[doc = "Level 1: 2.45 V"]
    #[inline(always)]
    pub fn level1(self) -> &'a mut W {
        self.variant(BODINTLEV_A::LEVEL1)
    }
    #[doc = "Level 2: 2.75 V"]
    #[inline(always)]
    pub fn level2(self) -> &'a mut W {
        self.variant(BODINTLEV_A::LEVEL2)
    }
    #[doc = "Level 3: 3.05 V"]
    #[inline(always)]
    pub fn level3(self) -> &'a mut W {
        self.variant(BODINTLEV_A::LEVEL3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "BOD interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODINTENA_A {
    #[doc = "0: Disable interrupt function."]
    DISABLE = 0,
    #[doc = "1: Enable interrupt function."]
    ENABLE = 1,
}
impl From<BODINTENA_A> for bool {
    #[inline(always)]
    fn from(variant: BODINTENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODINTENA` reader - BOD interrupt enable"]
pub struct BODINTENA_R(crate::FieldReader<bool, BODINTENA_A>);
impl BODINTENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BODINTENA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODINTENA_A {
        match self.bits {
            false => BODINTENA_A::DISABLE,
            true => BODINTENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == BODINTENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == BODINTENA_A::ENABLE
    }
}
impl core::ops::Deref for BODINTENA_R {
    type Target = crate::FieldReader<bool, BODINTENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODINTENA` writer - BOD interrupt enable"]
pub struct BODINTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> BODINTENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODINTENA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable interrupt function."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODINTENA_A::DISABLE)
    }
    #[doc = "Enable interrupt function."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODINTENA_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `BODRSTSTAT` reader - BOD reset status. When 1, a BOD reset has occurred. Cleared by writing 1 to this bit."]
pub struct BODRSTSTAT_R(crate::FieldReader<bool, bool>);
impl BODRSTSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BODRSTSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BODRSTSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODRSTSTAT` writer - BOD reset status. When 1, a BOD reset has occurred. Cleared by writing 1 to this bit."]
pub struct BODRSTSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BODRSTSTAT_W<'a> {
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
#[doc = "Field `BODINTSTAT` reader - BOD interrupt status. When 1, a BOD interrupt has occurred. Cleared by writing 1 to this bit."]
pub struct BODINTSTAT_R(crate::FieldReader<bool, bool>);
impl BODINTSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BODINTSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BODINTSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODINTSTAT` writer - BOD interrupt status. When 1, a BOD interrupt has occurred. Cleared by writing 1 to this bit."]
pub struct BODINTSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BODINTSTAT_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - BOD reset level"]
    #[inline(always)]
    pub fn bodrstlev(&self) -> BODRSTLEV_R {
        BODRSTLEV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - BOD reset enable"]
    #[inline(always)]
    pub fn bodrstena(&self) -> BODRSTENA_R {
        BODRSTENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - BOD interrupt level"]
    #[inline(always)]
    pub fn bodintlev(&self) -> BODINTLEV_R {
        BODINTLEV_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - BOD interrupt enable"]
    #[inline(always)]
    pub fn bodintena(&self) -> BODINTENA_R {
        BODINTENA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - BOD reset status. When 1, a BOD reset has occurred. Cleared by writing 1 to this bit."]
    #[inline(always)]
    pub fn bodrststat(&self) -> BODRSTSTAT_R {
        BODRSTSTAT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - BOD interrupt status. When 1, a BOD interrupt has occurred. Cleared by writing 1 to this bit."]
    #[inline(always)]
    pub fn bodintstat(&self) -> BODINTSTAT_R {
        BODINTSTAT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - BOD reset level"]
    #[inline(always)]
    pub fn bodrstlev(&mut self) -> BODRSTLEV_W {
        BODRSTLEV_W { w: self }
    }
    #[doc = "Bit 2 - BOD reset enable"]
    #[inline(always)]
    pub fn bodrstena(&mut self) -> BODRSTENA_W {
        BODRSTENA_W { w: self }
    }
    #[doc = "Bits 3:4 - BOD interrupt level"]
    #[inline(always)]
    pub fn bodintlev(&mut self) -> BODINTLEV_W {
        BODINTLEV_W { w: self }
    }
    #[doc = "Bit 5 - BOD interrupt enable"]
    #[inline(always)]
    pub fn bodintena(&mut self) -> BODINTENA_W {
        BODINTENA_W { w: self }
    }
    #[doc = "Bit 6 - BOD reset status. When 1, a BOD reset has occurred. Cleared by writing 1 to this bit."]
    #[inline(always)]
    pub fn bodrststat(&mut self) -> BODRSTSTAT_W {
        BODRSTSTAT_W { w: self }
    }
    #[doc = "Bit 7 - BOD interrupt status. When 1, a BOD interrupt has occurred. Cleared by writing 1 to this bit."]
    #[inline(always)]
    pub fn bodintstat(&mut self) -> BODINTSTAT_W {
        BODINTSTAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Brown-Out Detect control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bodctrl](index.html) module"]
pub struct BODCTRL_SPEC;
impl crate::RegisterSpec for BODCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bodctrl::R](R) reader structure"]
impl crate::Readable for BODCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bodctrl::W](W) writer structure"]
impl crate::Writable for BODCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BODCTRL to value 0"]
impl crate::Resettable for BODCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
