#[doc = "Register `PIO0_5` reader"]
pub struct R(crate::R<PIO0_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIO0_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIO0_5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIO0_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIO0_5` writer"]
pub struct W(crate::W<PIO0_5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIO0_5_SPEC>;
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
impl From<crate::W<PIO0_5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIO0_5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects pin function. Values 0x2 to 0x7 are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: PIO0_5 (open-drain pin)."]
    PIO0_5 = 0,
    #[doc = "1: I2C SDA (open-drain pin)."]
    I2C_SDA = 1,
}
impl From<FUNC_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FUNC` reader - Selects pin function. Values 0x2 to 0x7 are reserved."]
pub struct FUNC_R(crate::FieldReader<u8, FUNC_A>);
impl FUNC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FUNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FUNC_A> {
        match self.bits {
            0 => Some(FUNC_A::PIO0_5),
            1 => Some(FUNC_A::I2C_SDA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PIO0_5`"]
    #[inline(always)]
    pub fn is_pio0_5(&self) -> bool {
        **self == FUNC_A::PIO0_5
    }
    #[doc = "Checks if the value of the field is `I2C_SDA`"]
    #[inline(always)]
    pub fn is_i2c_sda(&self) -> bool {
        **self == FUNC_A::I2C_SDA
    }
}
impl core::ops::Deref for FUNC_R {
    type Target = crate::FieldReader<u8, FUNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUNC` writer - Selects pin function. Values 0x2 to 0x7 are reserved."]
pub struct FUNC_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FUNC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PIO0_5 (open-drain pin)."]
    #[inline(always)]
    pub fn pio0_5(self) -> &'a mut W {
        self.variant(FUNC_A::PIO0_5)
    }
    #[doc = "I2C SDA (open-drain pin)."]
    #[inline(always)]
    pub fn i2c_sda(self) -> &'a mut W {
        self.variant(FUNC_A::I2C_SDA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Selects I2C mode (see Section 7.3.8). Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2CMODE_A {
    #[doc = "0: Standard mode/ Fast-mode I2C."]
    STANDARD_MODE = 0,
    #[doc = "1: Standard I/O functionality"]
    STANDARD_IO = 1,
    #[doc = "2: Fast-mode Plus I2C"]
    FAST_MODE_PLUS = 2,
}
impl From<I2CMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: I2CMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `I2CMODE` reader - Selects I2C mode (see Section 7.3.8). Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000)."]
pub struct I2CMODE_R(crate::FieldReader<u8, I2CMODE_A>);
impl I2CMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        I2CMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2CMODE_A {
        match self.bits {
            0 => I2CMODE_A::STANDARD_MODE,
            1 => I2CMODE_A::STANDARD_IO,
            2 => I2CMODE_A::FAST_MODE_PLUS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD_MODE`"]
    #[inline(always)]
    pub fn is_standard_mode(&self) -> bool {
        **self == I2CMODE_A::STANDARD_MODE
    }
    #[doc = "Checks if the value of the field is `STANDARD_IO`"]
    #[inline(always)]
    pub fn is_standard_io(&self) -> bool {
        **self == I2CMODE_A::STANDARD_IO
    }
    #[doc = "Checks if the value of the field is `FAST_MODE_PLUS`"]
    #[inline(always)]
    pub fn is_fast_mode_plus(&self) -> bool {
        **self == I2CMODE_A::FAST_MODE_PLUS
    }
}
impl core::ops::Deref for I2CMODE_R {
    type Target = crate::FieldReader<u8, I2CMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2CMODE` writer - Selects I2C mode (see Section 7.3.8). Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000)."]
pub struct I2CMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2CMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Standard mode/ Fast-mode I2C."]
    #[inline(always)]
    pub fn standard_mode(self) -> &'a mut W {
        self.variant(I2CMODE_A::STANDARD_MODE)
    }
    #[doc = "Standard I/O functionality"]
    #[inline(always)]
    pub fn standard_io(self) -> &'a mut W {
        self.variant(I2CMODE_A::STANDARD_IO)
    }
    #[doc = "Fast-mode Plus I2C"]
    #[inline(always)]
    pub fn fast_mode_plus(self) -> &'a mut W {
        self.variant(I2CMODE_A::FAST_MODE_PLUS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function. Values 0x2 to 0x7 are reserved."]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - Selects I2C mode (see Section 7.3.8). Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000)."]
    #[inline(always)]
    pub fn i2cmode(&self) -> I2CMODE_R {
        I2CMODE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function. Values 0x2 to 0x7 are reserved."]
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W {
        FUNC_W { w: self }
    }
    #[doc = "Bits 8:9 - Selects I2C mode (see Section 7.3.8). Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000)."]
    #[inline(always)]
    pub fn i2cmode(&mut self) -> I2CMODE_W {
        I2CMODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O configuration for pin PIO0_5/SDA\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_5](index.html) module"]
pub struct PIO0_5_SPEC;
impl crate::RegisterSpec for PIO0_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pio0_5::R](R) reader structure"]
impl crate::Readable for PIO0_5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pio0_5::W](W) writer structure"]
impl crate::Writable for PIO0_5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIO0_5 to value 0x80"]
impl crate::Resettable for PIO0_5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
