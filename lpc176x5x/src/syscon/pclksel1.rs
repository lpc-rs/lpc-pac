#[doc = "Reader of register PCLKSEL1"]
pub type R = crate::R<u32, super::PCLKSEL1>;
#[doc = "Writer for register PCLKSEL1"]
pub type W = crate::W<u32, super::PCLKSEL1>;
#[doc = "Register PCLKSEL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PCLKSEL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Peripheral clock selection for the Quadrature Encoder Interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_QEI_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_QEI_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_QEI_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_QEI`"]
pub type PCLK_QEI_R = crate::R<u8, PCLK_QEI_A>;
impl PCLK_QEI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_QEI_A {
        match self.bits {
            0 => PCLK_QEI_A::CCLK_DIV_4,
            1 => PCLK_QEI_A::CCLK,
            2 => PCLK_QEI_A::CCLK_DIV_2,
            3 => PCLK_QEI_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_QEI_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_QEI_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_QEI_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_QEI_A::CCLK_DIV_8
    }
}
#[doc = "Write proxy for field `PCLK_QEI`"]
pub struct PCLK_QEI_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_QEI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_QEI_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_QEI_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_QEI_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_QEI_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_QEI_A::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Peripheral clock selection for GPIO interrupts.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_GPIOINT_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_GPIOINT_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_GPIOINT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_GPIOINT`"]
pub type PCLK_GPIOINT_R = crate::R<u8, PCLK_GPIOINT_A>;
impl PCLK_GPIOINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_GPIOINT_A {
        match self.bits {
            0 => PCLK_GPIOINT_A::CCLK_DIV_4,
            1 => PCLK_GPIOINT_A::CCLK,
            2 => PCLK_GPIOINT_A::CCLK_DIV_2,
            3 => PCLK_GPIOINT_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_GPIOINT_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_GPIOINT_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_GPIOINT_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_GPIOINT_A::CCLK_DIV_8
    }
}
#[doc = "Write proxy for field `PCLK_GPIOINT`"]
pub struct PCLK_GPIOINT_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_GPIOINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_GPIOINT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_GPIOINT_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_GPIOINT_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_GPIOINT_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_GPIOINT_A::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Peripheral clock selection for the Pin Connect block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_PCB_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_PCB_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_PCB_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_PCB`"]
pub type PCLK_PCB_R = crate::R<u8, PCLK_PCB_A>;
impl PCLK_PCB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_PCB_A {
        match self.bits {
            0 => PCLK_PCB_A::CCLK_DIV_4,
            1 => PCLK_PCB_A::CCLK,
            2 => PCLK_PCB_A::CCLK_DIV_2,
            3 => PCLK_PCB_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_PCB_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_PCB_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_PCB_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_PCB_A::CCLK_DIV_8
    }
}
#[doc = "Write proxy for field `PCLK_PCB`"]
pub struct PCLK_PCB_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_PCB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_PCB_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_PCB_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_PCB_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_PCB_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_PCB_A::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Peripheral clock selection for I2C1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_I2C1_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_I2C1_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_I2C1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_I2C1`"]
pub type PCLK_I2C1_R = crate::R<u8, PCLK_I2C1_A>;
impl PCLK_I2C1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_I2C1_A {
        match self.bits {
            0 => PCLK_I2C1_A::CCLK_DIV_4,
            1 => PCLK_I2C1_A::CCLK,
            2 => PCLK_I2C1_A::CCLK_DIV_2,
            3 => PCLK_I2C1_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_I2C1_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_I2C1_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_I2C1_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_I2C1_A::CCLK_DIV_8
    }
}
#[doc = "Write proxy for field `PCLK_I2C1`"]
pub struct PCLK_I2C1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_I2C1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_I2C1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_I2C1_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_I2C1_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_I2C1_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_I2C1_A::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Peripheral clock selection for SSP0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_SSP0_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_SSP0_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_SSP0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_SSP0`"]
pub type PCLK_SSP0_R = crate::R<u8, PCLK_SSP0_A>;
impl PCLK_SSP0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_SSP0_A {
        match self.bits {
            0 => PCLK_SSP0_A::CCLK_DIV_4,
            1 => PCLK_SSP0_A::CCLK,
            2 => PCLK_SSP0_A::CCLK_DIV_2,
            3 => PCLK_SSP0_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_SSP0_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_SSP0_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_SSP0_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_SSP0_A::CCLK_DIV_8
    }
}
#[doc = "Write proxy for field `PCLK_SSP0`"]
pub struct PCLK_SSP0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_SSP0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_SSP0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_SSP0_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_SSP0_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_SSP0_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_SSP0_A::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Peripheral clock selection for TIMER2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_TIMER2_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_TIMER2_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_TIMER2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_TIMER2`"]
pub type PCLK_TIMER2_R = crate::R<u8, PCLK_TIMER2_A>;
impl PCLK_TIMER2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_TIMER2_A {
        match self.bits {
            0 => PCLK_TIMER2_A::CCLK_DIV_4,
            1 => PCLK_TIMER2_A::CCLK,
            2 => PCLK_TIMER2_A::CCLK_DIV_2,
            3 => PCLK_TIMER2_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_TIMER2_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_TIMER2_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_TIMER2_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_TIMER2_A::CCLK_DIV_8
    }
}
#[doc = "Write proxy for field `PCLK_TIMER2`"]
pub struct PCLK_TIMER2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_TIMER2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_TIMER2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_TIMER2_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_TIMER2_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_TIMER2_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_TIMER2_A::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Peripheral clock selection for TIMER3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_TIMER3_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_TIMER3_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_TIMER3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_TIMER3`"]
pub type PCLK_TIMER3_R = crate::R<u8, PCLK_TIMER3_A>;
impl PCLK_TIMER3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_TIMER3_A {
        match self.bits {
            0 => PCLK_TIMER3_A::CCLK_DIV_4,
            1 => PCLK_TIMER3_A::CCLK,
            2 => PCLK_TIMER3_A::CCLK_DIV_2,
            3 => PCLK_TIMER3_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_TIMER3_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_TIMER3_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_TIMER3_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_TIMER3_A::CCLK_DIV_8
    }
}
#[doc = "Write proxy for field `PCLK_TIMER3`"]
pub struct PCLK_TIMER3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_TIMER3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_TIMER3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_TIMER3_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_TIMER3_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_TIMER3_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_TIMER3_A::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Peripheral clock selection for UART2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_UART2_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_UART2_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_UART2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_UART2`"]
pub type PCLK_UART2_R = crate::R<u8, PCLK_UART2_A>;
impl PCLK_UART2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_UART2_A {
        match self.bits {
            0 => PCLK_UART2_A::CCLK_DIV_4,
            1 => PCLK_UART2_A::CCLK,
            2 => PCLK_UART2_A::CCLK_DIV_2,
            3 => PCLK_UART2_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_UART2_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_UART2_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_UART2_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_UART2_A::CCLK_DIV_8
    }
}
#[doc = "Write proxy for field `PCLK_UART2`"]
pub struct PCLK_UART2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_UART2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_UART2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_UART2_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_UART2_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_UART2_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_UART2_A::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Peripheral clock selection for UART3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_UART3_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_UART3_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_UART3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_UART3`"]
pub type PCLK_UART3_R = crate::R<u8, PCLK_UART3_A>;
impl PCLK_UART3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_UART3_A {
        match self.bits {
            0 => PCLK_UART3_A::CCLK_DIV_4,
            1 => PCLK_UART3_A::CCLK,
            2 => PCLK_UART3_A::CCLK_DIV_2,
            3 => PCLK_UART3_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_UART3_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_UART3_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_UART3_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_UART3_A::CCLK_DIV_8
    }
}
#[doc = "Write proxy for field `PCLK_UART3`"]
pub struct PCLK_UART3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_UART3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_UART3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_UART3_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_UART3_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_UART3_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_UART3_A::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Peripheral clock selection for I2C2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_I2C2_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_I2C2_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_I2C2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_I2C2`"]
pub type PCLK_I2C2_R = crate::R<u8, PCLK_I2C2_A>;
impl PCLK_I2C2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_I2C2_A {
        match self.bits {
            0 => PCLK_I2C2_A::CCLK_DIV_4,
            1 => PCLK_I2C2_A::CCLK,
            2 => PCLK_I2C2_A::CCLK_DIV_2,
            3 => PCLK_I2C2_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_I2C2_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_I2C2_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_I2C2_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_I2C2_A::CCLK_DIV_8
    }
}
#[doc = "Write proxy for field `PCLK_I2C2`"]
pub struct PCLK_I2C2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_I2C2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_I2C2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_I2C2_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_I2C2_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_I2C2_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_I2C2_A::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Peripheral clock selection for I2S.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_I2S_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_I2S_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_I2S_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_I2S`"]
pub type PCLK_I2S_R = crate::R<u8, PCLK_I2S_A>;
impl PCLK_I2S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_I2S_A {
        match self.bits {
            0 => PCLK_I2S_A::CCLK_DIV_4,
            1 => PCLK_I2S_A::CCLK,
            2 => PCLK_I2S_A::CCLK_DIV_2,
            3 => PCLK_I2S_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_I2S_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_I2S_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_I2S_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_I2S_A::CCLK_DIV_8
    }
}
#[doc = "Write proxy for field `PCLK_I2S`"]
pub struct PCLK_I2S_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_I2S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_I2S_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_I2S_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_I2S_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_I2S_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_I2S_A::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Peripheral clock selection for Repetitive Interrupt Timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_RIT_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_RIT_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_RIT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_RIT`"]
pub type PCLK_RIT_R = crate::R<u8, PCLK_RIT_A>;
impl PCLK_RIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_RIT_A {
        match self.bits {
            0 => PCLK_RIT_A::CCLK_DIV_4,
            1 => PCLK_RIT_A::CCLK,
            2 => PCLK_RIT_A::CCLK_DIV_2,
            3 => PCLK_RIT_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_RIT_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_RIT_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_RIT_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_RIT_A::CCLK_DIV_8
    }
}
#[doc = "Write proxy for field `PCLK_RIT`"]
pub struct PCLK_RIT_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_RIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_RIT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_RIT_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_RIT_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_RIT_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_RIT_A::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Peripheral clock selection for the System Control block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_SYSCON_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_SYSCON_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_SYSCON_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_SYSCON`"]
pub type PCLK_SYSCON_R = crate::R<u8, PCLK_SYSCON_A>;
impl PCLK_SYSCON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_SYSCON_A {
        match self.bits {
            0 => PCLK_SYSCON_A::CCLK_DIV_4,
            1 => PCLK_SYSCON_A::CCLK,
            2 => PCLK_SYSCON_A::CCLK_DIV_2,
            3 => PCLK_SYSCON_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_SYSCON_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_SYSCON_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_SYSCON_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_SYSCON_A::CCLK_DIV_8
    }
}
#[doc = "Write proxy for field `PCLK_SYSCON`"]
pub struct PCLK_SYSCON_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_SYSCON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_SYSCON_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_SYSCON_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_SYSCON_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_SYSCON_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_SYSCON_A::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Peripheral clock selection for the Motor Control PWM.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_MC_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_MC_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_MC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_MC`"]
pub type PCLK_MC_R = crate::R<u8, PCLK_MC_A>;
impl PCLK_MC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_MC_A {
        match self.bits {
            0 => PCLK_MC_A::CCLK_DIV_4,
            1 => PCLK_MC_A::CCLK,
            2 => PCLK_MC_A::CCLK_DIV_2,
            3 => PCLK_MC_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_MC_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_MC_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_MC_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_MC_A::CCLK_DIV_8
    }
}
#[doc = "Write proxy for field `PCLK_MC`"]
pub struct PCLK_MC_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_MC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_MC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_MC_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_MC_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_MC_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_MC_A::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Peripheral clock selection for the Quadrature Encoder Interface."]
    #[inline(always)]
    pub fn pclk_qei(&self) -> PCLK_QEI_R {
        PCLK_QEI_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Peripheral clock selection for GPIO interrupts."]
    #[inline(always)]
    pub fn pclk_gpioint(&self) -> PCLK_GPIOINT_R {
        PCLK_GPIOINT_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Peripheral clock selection for the Pin Connect block."]
    #[inline(always)]
    pub fn pclk_pcb(&self) -> PCLK_PCB_R {
        PCLK_PCB_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Peripheral clock selection for I2C1."]
    #[inline(always)]
    pub fn pclk_i2c1(&self) -> PCLK_I2C1_R {
        PCLK_I2C1_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Peripheral clock selection for SSP0."]
    #[inline(always)]
    pub fn pclk_ssp0(&self) -> PCLK_SSP0_R {
        PCLK_SSP0_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Peripheral clock selection for TIMER2."]
    #[inline(always)]
    pub fn pclk_timer2(&self) -> PCLK_TIMER2_R {
        PCLK_TIMER2_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Peripheral clock selection for TIMER3."]
    #[inline(always)]
    pub fn pclk_timer3(&self) -> PCLK_TIMER3_R {
        PCLK_TIMER3_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Peripheral clock selection for UART2."]
    #[inline(always)]
    pub fn pclk_uart2(&self) -> PCLK_UART2_R {
        PCLK_UART2_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Peripheral clock selection for UART3."]
    #[inline(always)]
    pub fn pclk_uart3(&self) -> PCLK_UART3_R {
        PCLK_UART3_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Peripheral clock selection for I2C2."]
    #[inline(always)]
    pub fn pclk_i2c2(&self) -> PCLK_I2C2_R {
        PCLK_I2C2_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Peripheral clock selection for I2S."]
    #[inline(always)]
    pub fn pclk_i2s(&self) -> PCLK_I2S_R {
        PCLK_I2S_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Peripheral clock selection for Repetitive Interrupt Timer."]
    #[inline(always)]
    pub fn pclk_rit(&self) -> PCLK_RIT_R {
        PCLK_RIT_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Peripheral clock selection for the System Control block."]
    #[inline(always)]
    pub fn pclk_syscon(&self) -> PCLK_SYSCON_R {
        PCLK_SYSCON_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Peripheral clock selection for the Motor Control PWM."]
    #[inline(always)]
    pub fn pclk_mc(&self) -> PCLK_MC_R {
        PCLK_MC_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Peripheral clock selection for the Quadrature Encoder Interface."]
    #[inline(always)]
    pub fn pclk_qei(&mut self) -> PCLK_QEI_W {
        PCLK_QEI_W { w: self }
    }
    #[doc = "Bits 2:3 - Peripheral clock selection for GPIO interrupts."]
    #[inline(always)]
    pub fn pclk_gpioint(&mut self) -> PCLK_GPIOINT_W {
        PCLK_GPIOINT_W { w: self }
    }
    #[doc = "Bits 4:5 - Peripheral clock selection for the Pin Connect block."]
    #[inline(always)]
    pub fn pclk_pcb(&mut self) -> PCLK_PCB_W {
        PCLK_PCB_W { w: self }
    }
    #[doc = "Bits 6:7 - Peripheral clock selection for I2C1."]
    #[inline(always)]
    pub fn pclk_i2c1(&mut self) -> PCLK_I2C1_W {
        PCLK_I2C1_W { w: self }
    }
    #[doc = "Bits 10:11 - Peripheral clock selection for SSP0."]
    #[inline(always)]
    pub fn pclk_ssp0(&mut self) -> PCLK_SSP0_W {
        PCLK_SSP0_W { w: self }
    }
    #[doc = "Bits 12:13 - Peripheral clock selection for TIMER2."]
    #[inline(always)]
    pub fn pclk_timer2(&mut self) -> PCLK_TIMER2_W {
        PCLK_TIMER2_W { w: self }
    }
    #[doc = "Bits 14:15 - Peripheral clock selection for TIMER3."]
    #[inline(always)]
    pub fn pclk_timer3(&mut self) -> PCLK_TIMER3_W {
        PCLK_TIMER3_W { w: self }
    }
    #[doc = "Bits 16:17 - Peripheral clock selection for UART2."]
    #[inline(always)]
    pub fn pclk_uart2(&mut self) -> PCLK_UART2_W {
        PCLK_UART2_W { w: self }
    }
    #[doc = "Bits 18:19 - Peripheral clock selection for UART3."]
    #[inline(always)]
    pub fn pclk_uart3(&mut self) -> PCLK_UART3_W {
        PCLK_UART3_W { w: self }
    }
    #[doc = "Bits 20:21 - Peripheral clock selection for I2C2."]
    #[inline(always)]
    pub fn pclk_i2c2(&mut self) -> PCLK_I2C2_W {
        PCLK_I2C2_W { w: self }
    }
    #[doc = "Bits 22:23 - Peripheral clock selection for I2S."]
    #[inline(always)]
    pub fn pclk_i2s(&mut self) -> PCLK_I2S_W {
        PCLK_I2S_W { w: self }
    }
    #[doc = "Bits 26:27 - Peripheral clock selection for Repetitive Interrupt Timer."]
    #[inline(always)]
    pub fn pclk_rit(&mut self) -> PCLK_RIT_W {
        PCLK_RIT_W { w: self }
    }
    #[doc = "Bits 28:29 - Peripheral clock selection for the System Control block."]
    #[inline(always)]
    pub fn pclk_syscon(&mut self) -> PCLK_SYSCON_W {
        PCLK_SYSCON_W { w: self }
    }
    #[doc = "Bits 30:31 - Peripheral clock selection for the Motor Control PWM."]
    #[inline(always)]
    pub fn pclk_mc(&mut self) -> PCLK_MC_W {
        PCLK_MC_W { w: self }
    }
}
