#[doc = "Register `PIO0_10` reader"]
pub struct R(crate::R<PIO0_10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIO0_10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIO0_10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIO0_10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIO0_10` writer"]
pub struct W(crate::W<PIO0_10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIO0_10_SPEC>;
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
impl From<crate::W<PIO0_10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIO0_10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Invert input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INV_A {
    #[doc = "0: Input not inverted (HIGH on pin reads as 1; LOW on pin reads as 0)."]
    NOT_INVERTED = 0,
    #[doc = "1: Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    INVERTED = 1,
}
impl From<INV_A> for bool {
    #[inline(always)]
    fn from(variant: INV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INV` reader - Invert input"]
pub struct INV_R(crate::FieldReader<bool, INV_A>);
impl INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INV_A {
        match self.bits {
            false => INV_A::NOT_INVERTED,
            true => INV_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        **self == INV_A::NOT_INVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == INV_A::INVERTED
    }
}
impl core::ops::Deref for INV_R {
    type Target = crate::FieldReader<bool, INV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INV` writer - Invert input"]
pub struct INV_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input not inverted (HIGH on pin reads as 1; LOW on pin reads as 0)."]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(INV_A::NOT_INVERTED)
    }
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(INV_A::INVERTED)
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
#[doc = "Selects I2C mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2CMODE_A {
    #[doc = "0: Standard mode/ Fast-mode I2C."]
    STANDARAD_I2C = 0,
    #[doc = "1: Standard GPIO functionality. Requires external pull-up for GPIO output function."]
    STANDARD_GPIO = 1,
    #[doc = "2: Fast-mode Plus I2C"]
    FAST_PLUS_I2C = 2,
}
impl From<I2CMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: I2CMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `I2CMODE` reader - Selects I2C mode."]
pub struct I2CMODE_R(crate::FieldReader<u8, I2CMODE_A>);
impl I2CMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        I2CMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<I2CMODE_A> {
        match self.bits {
            0 => Some(I2CMODE_A::STANDARAD_I2C),
            1 => Some(I2CMODE_A::STANDARD_GPIO),
            2 => Some(I2CMODE_A::FAST_PLUS_I2C),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARAD_I2C`"]
    #[inline(always)]
    pub fn is_standarad_i2c(&self) -> bool {
        **self == I2CMODE_A::STANDARAD_I2C
    }
    #[doc = "Checks if the value of the field is `STANDARD_GPIO`"]
    #[inline(always)]
    pub fn is_standard_gpio(&self) -> bool {
        **self == I2CMODE_A::STANDARD_GPIO
    }
    #[doc = "Checks if the value of the field is `FAST_PLUS_I2C`"]
    #[inline(always)]
    pub fn is_fast_plus_i2c(&self) -> bool {
        **self == I2CMODE_A::FAST_PLUS_I2C
    }
}
impl core::ops::Deref for I2CMODE_R {
    type Target = crate::FieldReader<u8, I2CMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2CMODE` writer - Selects I2C mode."]
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
    pub fn standarad_i2c(self) -> &'a mut W {
        self.variant(I2CMODE_A::STANDARAD_I2C)
    }
    #[doc = "Standard GPIO functionality. Requires external pull-up for GPIO output function."]
    #[inline(always)]
    pub fn standard_gpio(self) -> &'a mut W {
        self.variant(I2CMODE_A::STANDARD_GPIO)
    }
    #[doc = "Fast-mode Plus I2C"]
    #[inline(always)]
    pub fn fast_plus_i2c(self) -> &'a mut W {
        self.variant(I2CMODE_A::FAST_PLUS_I2C)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Digital filter sample mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum S_MODE_A {
    #[doc = "0: Bypass input filter."]
    S_MODE_0 = 0,
    #[doc = "1: 1 clock cycle. Input pulses shorter than one filter clock are rejected."]
    S_MODE_1 = 1,
    #[doc = "2: 2 clock cycles. Input pulses shorter than two filter clocks are rejected."]
    S_MODE_2 = 2,
    #[doc = "3: 3 clock cycles. Input pulses shorter than three filter clocks are rejected."]
    S_MODE_3 = 3,
}
impl From<S_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: S_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `S_MODE` reader - Digital filter sample mode."]
pub struct S_MODE_R(crate::FieldReader<u8, S_MODE_A>);
impl S_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        S_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S_MODE_A {
        match self.bits {
            0 => S_MODE_A::S_MODE_0,
            1 => S_MODE_A::S_MODE_1,
            2 => S_MODE_A::S_MODE_2,
            3 => S_MODE_A::S_MODE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `S_MODE_0`"]
    #[inline(always)]
    pub fn is_s_mode_0(&self) -> bool {
        **self == S_MODE_A::S_MODE_0
    }
    #[doc = "Checks if the value of the field is `S_MODE_1`"]
    #[inline(always)]
    pub fn is_s_mode_1(&self) -> bool {
        **self == S_MODE_A::S_MODE_1
    }
    #[doc = "Checks if the value of the field is `S_MODE_2`"]
    #[inline(always)]
    pub fn is_s_mode_2(&self) -> bool {
        **self == S_MODE_A::S_MODE_2
    }
    #[doc = "Checks if the value of the field is `S_MODE_3`"]
    #[inline(always)]
    pub fn is_s_mode_3(&self) -> bool {
        **self == S_MODE_A::S_MODE_3
    }
}
impl core::ops::Deref for S_MODE_R {
    type Target = crate::FieldReader<u8, S_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S_MODE` writer - Digital filter sample mode."]
pub struct S_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> S_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Bypass input filter."]
    #[inline(always)]
    pub fn s_mode_0(self) -> &'a mut W {
        self.variant(S_MODE_A::S_MODE_0)
    }
    #[doc = "1 clock cycle. Input pulses shorter than one filter clock are rejected."]
    #[inline(always)]
    pub fn s_mode_1(self) -> &'a mut W {
        self.variant(S_MODE_A::S_MODE_1)
    }
    #[doc = "2 clock cycles. Input pulses shorter than two filter clocks are rejected."]
    #[inline(always)]
    pub fn s_mode_2(self) -> &'a mut W {
        self.variant(S_MODE_A::S_MODE_2)
    }
    #[doc = "3 clock cycles. Input pulses shorter than three filter clocks are rejected."]
    #[inline(always)]
    pub fn s_mode_3(self) -> &'a mut W {
        self.variant(S_MODE_A::S_MODE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "Select peripheral clock divider for input filter sampling clock. Value 0x7 is reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_DIV_A {
    #[doc = "0: IOCONCLKDIV0"]
    CLK_DIV_0 = 0,
    #[doc = "1: IOCONCLKDIV1"]
    CLK_DIV_1 = 1,
    #[doc = "2: IOCONCLKDIV2"]
    CLK_DIV_2 = 2,
    #[doc = "3: IOCONCLKDIV3"]
    CLK_DIV_3 = 3,
    #[doc = "4: IOCONCLKDIV4"]
    CLK_DIV_4 = 4,
    #[doc = "5: IOCONCLKDIV5"]
    CLK_DIV_5 = 5,
    #[doc = "6: IOCONCLKDIV6"]
    CLK_DIV_6 = 6,
}
impl From<CLK_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLK_DIV` reader - Select peripheral clock divider for input filter sampling clock. Value 0x7 is reserved."]
pub struct CLK_DIV_R(crate::FieldReader<u8, CLK_DIV_A>);
impl CLK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLK_DIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLK_DIV_A> {
        match self.bits {
            0 => Some(CLK_DIV_A::CLK_DIV_0),
            1 => Some(CLK_DIV_A::CLK_DIV_1),
            2 => Some(CLK_DIV_A::CLK_DIV_2),
            3 => Some(CLK_DIV_A::CLK_DIV_3),
            4 => Some(CLK_DIV_A::CLK_DIV_4),
            5 => Some(CLK_DIV_A::CLK_DIV_5),
            6 => Some(CLK_DIV_A::CLK_DIV_6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_0`"]
    #[inline(always)]
    pub fn is_clk_div_0(&self) -> bool {
        **self == CLK_DIV_A::CLK_DIV_0
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_1`"]
    #[inline(always)]
    pub fn is_clk_div_1(&self) -> bool {
        **self == CLK_DIV_A::CLK_DIV_1
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_2`"]
    #[inline(always)]
    pub fn is_clk_div_2(&self) -> bool {
        **self == CLK_DIV_A::CLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_3`"]
    #[inline(always)]
    pub fn is_clk_div_3(&self) -> bool {
        **self == CLK_DIV_A::CLK_DIV_3
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_4`"]
    #[inline(always)]
    pub fn is_clk_div_4(&self) -> bool {
        **self == CLK_DIV_A::CLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_5`"]
    #[inline(always)]
    pub fn is_clk_div_5(&self) -> bool {
        **self == CLK_DIV_A::CLK_DIV_5
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_6`"]
    #[inline(always)]
    pub fn is_clk_div_6(&self) -> bool {
        **self == CLK_DIV_A::CLK_DIV_6
    }
}
impl core::ops::Deref for CLK_DIV_R {
    type Target = crate::FieldReader<u8, CLK_DIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_DIV` writer - Select peripheral clock divider for input filter sampling clock. Value 0x7 is reserved."]
pub struct CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_DIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "IOCONCLKDIV0"]
    #[inline(always)]
    pub fn clk_div_0(self) -> &'a mut W {
        self.variant(CLK_DIV_A::CLK_DIV_0)
    }
    #[doc = "IOCONCLKDIV1"]
    #[inline(always)]
    pub fn clk_div_1(self) -> &'a mut W {
        self.variant(CLK_DIV_A::CLK_DIV_1)
    }
    #[doc = "IOCONCLKDIV2"]
    #[inline(always)]
    pub fn clk_div_2(self) -> &'a mut W {
        self.variant(CLK_DIV_A::CLK_DIV_2)
    }
    #[doc = "IOCONCLKDIV3"]
    #[inline(always)]
    pub fn clk_div_3(self) -> &'a mut W {
        self.variant(CLK_DIV_A::CLK_DIV_3)
    }
    #[doc = "IOCONCLKDIV4"]
    #[inline(always)]
    pub fn clk_div_4(self) -> &'a mut W {
        self.variant(CLK_DIV_A::CLK_DIV_4)
    }
    #[doc = "IOCONCLKDIV5"]
    #[inline(always)]
    pub fn clk_div_5(self) -> &'a mut W {
        self.variant(CLK_DIV_A::CLK_DIV_5)
    }
    #[doc = "IOCONCLKDIV6"]
    #[inline(always)]
    pub fn clk_div_6(self) -> &'a mut W {
        self.variant(CLK_DIV_A::CLK_DIV_6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | ((value as u32 & 0x07) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 6 - Invert input"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Selects I2C mode."]
    #[inline(always)]
    pub fn i2cmode(&self) -> I2CMODE_R {
        I2CMODE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 11:12 - Digital filter sample mode."]
    #[inline(always)]
    pub fn s_mode(&self) -> S_MODE_R {
        S_MODE_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 13:15 - Select peripheral clock divider for input filter sampling clock. Value 0x7 is reserved."]
    #[inline(always)]
    pub fn clk_div(&self) -> CLK_DIV_R {
        CLK_DIV_R::new(((self.bits >> 13) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 6 - Invert input"]
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W {
        INV_W { w: self }
    }
    #[doc = "Bits 8:9 - Selects I2C mode."]
    #[inline(always)]
    pub fn i2cmode(&mut self) -> I2CMODE_W {
        I2CMODE_W { w: self }
    }
    #[doc = "Bits 11:12 - Digital filter sample mode."]
    #[inline(always)]
    pub fn s_mode(&mut self) -> S_MODE_W {
        S_MODE_W { w: self }
    }
    #[doc = "Bits 13:15 - Select peripheral clock divider for input filter sampling clock. Value 0x7 is reserved."]
    #[inline(always)]
    pub fn clk_div(&mut self) -> CLK_DIV_W {
        CLK_DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital I/O control for pins PIO0_10\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_10](index.html) module"]
pub struct PIO0_10_SPEC;
impl crate::RegisterSpec for PIO0_10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pio0_10::R](R) reader structure"]
impl crate::Readable for PIO0_10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pio0_10::W](W) writer structure"]
impl crate::Writable for PIO0_10_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIO0_10 to value 0x80"]
impl crate::Resettable for PIO0_10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
