#[doc = "Reader of register PCLKSEL0"]
pub type R = crate::R<u32, super::PCLKSEL0>;
#[doc = "Writer for register PCLKSEL0"]
pub type W = crate::W<u32, super::PCLKSEL0>;
#[doc = "Register PCLKSEL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PCLKSEL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Peripheral clock selection for WDT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_WDT_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_WDT_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_WDT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_WDT`"]
pub type PCLK_WDT_R = crate::R<u8, PCLK_WDT_A>;
impl PCLK_WDT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_WDT_A {
        match self.bits {
            0 => PCLK_WDT_A::CCLK_DIV_4,
            1 => PCLK_WDT_A::CCLK,
            2 => PCLK_WDT_A::CCLK_DIV_2,
            3 => PCLK_WDT_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_WDT_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_WDT_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_WDT_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_WDT_A::CCLK_DIV_8
    }
}
#[doc = "Write proxy for field `PCLK_WDT`"]
pub struct PCLK_WDT_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_WDT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_WDT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_WDT_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_WDT_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_WDT_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_WDT_A::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Peripheral clock selection for TIMER0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_TIMER0_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_TIMER0_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_TIMER0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_TIMER0`"]
pub type PCLK_TIMER0_R = crate::R<u8, PCLK_TIMER0_A>;
impl PCLK_TIMER0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_TIMER0_A {
        match self.bits {
            0 => PCLK_TIMER0_A::CCLK_DIV_4,
            1 => PCLK_TIMER0_A::CCLK,
            2 => PCLK_TIMER0_A::CCLK_DIV_2,
            3 => PCLK_TIMER0_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_TIMER0_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_TIMER0_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_TIMER0_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_TIMER0_A::CCLK_DIV_8
    }
}
#[doc = "Write proxy for field `PCLK_TIMER0`"]
pub struct PCLK_TIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_TIMER0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_TIMER0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_TIMER0_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_TIMER0_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_TIMER0_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_TIMER0_A::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Peripheral clock selection for TIMER1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_TIMER1_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_TIMER1_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_TIMER1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_TIMER1`"]
pub type PCLK_TIMER1_R = crate::R<u8, PCLK_TIMER1_A>;
impl PCLK_TIMER1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_TIMER1_A {
        match self.bits {
            0 => PCLK_TIMER1_A::CCLK_DIV_4,
            1 => PCLK_TIMER1_A::CCLK,
            2 => PCLK_TIMER1_A::CCLK_DIV_2,
            3 => PCLK_TIMER1_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_TIMER1_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_TIMER1_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_TIMER1_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_TIMER1_A::CCLK_DIV_8
    }
}
#[doc = "Write proxy for field `PCLK_TIMER1`"]
pub struct PCLK_TIMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_TIMER1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_TIMER1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_TIMER1_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_TIMER1_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_TIMER1_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_TIMER1_A::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Peripheral clock selection for UART0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_UART0_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_UART0_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_UART0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_UART0`"]
pub type PCLK_UART0_R = crate::R<u8, PCLK_UART0_A>;
impl PCLK_UART0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_UART0_A {
        match self.bits {
            0 => PCLK_UART0_A::CCLK_DIV_4,
            1 => PCLK_UART0_A::CCLK,
            2 => PCLK_UART0_A::CCLK_DIV_2,
            3 => PCLK_UART0_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_UART0_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_UART0_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_UART0_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_UART0_A::CCLK_DIV_8
    }
}
#[doc = "Write proxy for field `PCLK_UART0`"]
pub struct PCLK_UART0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_UART0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_UART0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_UART0_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_UART0_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_UART0_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_UART0_A::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Peripheral clock selection for UART1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_UART1_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_UART1_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_UART1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_UART1`"]
pub type PCLK_UART1_R = crate::R<u8, PCLK_UART1_A>;
impl PCLK_UART1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_UART1_A {
        match self.bits {
            0 => PCLK_UART1_A::CCLK_DIV_4,
            1 => PCLK_UART1_A::CCLK,
            2 => PCLK_UART1_A::CCLK_DIV_2,
            3 => PCLK_UART1_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_UART1_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_UART1_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_UART1_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_UART1_A::CCLK_DIV_8
    }
}
#[doc = "Write proxy for field `PCLK_UART1`"]
pub struct PCLK_UART1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_UART1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_UART1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_UART1_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_UART1_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_UART1_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_UART1_A::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Peripheral clock selection for PWM1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_PWM1_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_PWM1_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_PWM1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_PWM1`"]
pub type PCLK_PWM1_R = crate::R<u8, PCLK_PWM1_A>;
impl PCLK_PWM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_PWM1_A {
        match self.bits {
            0 => PCLK_PWM1_A::CCLK_DIV_4,
            1 => PCLK_PWM1_A::CCLK,
            2 => PCLK_PWM1_A::CCLK_DIV_2,
            3 => PCLK_PWM1_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_PWM1_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_PWM1_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_PWM1_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_PWM1_A::CCLK_DIV_8
    }
}
#[doc = "Write proxy for field `PCLK_PWM1`"]
pub struct PCLK_PWM1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_PWM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_PWM1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_PWM1_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_PWM1_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_PWM1_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_PWM1_A::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Peripheral clock selection for I2C0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_I2C0_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_I2C0_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_I2C0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_I2C0`"]
pub type PCLK_I2C0_R = crate::R<u8, PCLK_I2C0_A>;
impl PCLK_I2C0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_I2C0_A {
        match self.bits {
            0 => PCLK_I2C0_A::CCLK_DIV_4,
            1 => PCLK_I2C0_A::CCLK,
            2 => PCLK_I2C0_A::CCLK_DIV_2,
            3 => PCLK_I2C0_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_I2C0_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_I2C0_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_I2C0_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_I2C0_A::CCLK_DIV_8
    }
}
#[doc = "Write proxy for field `PCLK_I2C0`"]
pub struct PCLK_I2C0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_I2C0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_I2C0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_I2C0_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_I2C0_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_I2C0_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_I2C0_A::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Peripheral clock selection for SPI.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_SPI_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_SPI_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_SPI_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_SPI`"]
pub type PCLK_SPI_R = crate::R<u8, PCLK_SPI_A>;
impl PCLK_SPI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_SPI_A {
        match self.bits {
            0 => PCLK_SPI_A::CCLK_DIV_4,
            1 => PCLK_SPI_A::CCLK,
            2 => PCLK_SPI_A::CCLK_DIV_2,
            3 => PCLK_SPI_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_SPI_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_SPI_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_SPI_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_SPI_A::CCLK_DIV_8
    }
}
#[doc = "Write proxy for field `PCLK_SPI`"]
pub struct PCLK_SPI_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_SPI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_SPI_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_SPI_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_SPI_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_SPI_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_SPI_A::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Peripheral clock selection for SSP1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_SSP1_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_SSP1_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_SSP1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_SSP1`"]
pub type PCLK_SSP1_R = crate::R<u8, PCLK_SSP1_A>;
impl PCLK_SSP1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_SSP1_A {
        match self.bits {
            0 => PCLK_SSP1_A::CCLK_DIV_4,
            1 => PCLK_SSP1_A::CCLK,
            2 => PCLK_SSP1_A::CCLK_DIV_2,
            3 => PCLK_SSP1_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_SSP1_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_SSP1_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_SSP1_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_SSP1_A::CCLK_DIV_8
    }
}
#[doc = "Write proxy for field `PCLK_SSP1`"]
pub struct PCLK_SSP1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_SSP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_SSP1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_SSP1_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_SSP1_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_SSP1_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_SSP1_A::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Peripheral clock selection for DAC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_DAC_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_DAC_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_DAC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_DAC`"]
pub type PCLK_DAC_R = crate::R<u8, PCLK_DAC_A>;
impl PCLK_DAC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_DAC_A {
        match self.bits {
            0 => PCLK_DAC_A::CCLK_DIV_4,
            1 => PCLK_DAC_A::CCLK,
            2 => PCLK_DAC_A::CCLK_DIV_2,
            3 => PCLK_DAC_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_DAC_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_DAC_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_DAC_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_DAC_A::CCLK_DIV_8
    }
}
#[doc = "Write proxy for field `PCLK_DAC`"]
pub struct PCLK_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_DAC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_DAC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_DAC_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_DAC_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_DAC_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_DAC_A::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Peripheral clock selection for ADC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_ADC_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_ADC_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_ADC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_ADC`"]
pub type PCLK_ADC_R = crate::R<u8, PCLK_ADC_A>;
impl PCLK_ADC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_ADC_A {
        match self.bits {
            0 => PCLK_ADC_A::CCLK_DIV_4,
            1 => PCLK_ADC_A::CCLK,
            2 => PCLK_ADC_A::CCLK_DIV_2,
            3 => PCLK_ADC_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_ADC_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_ADC_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_ADC_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_ADC_A::CCLK_DIV_8
    }
}
#[doc = "Write proxy for field `PCLK_ADC`"]
pub struct PCLK_ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_ADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_ADC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_ADC_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_ADC_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_ADC_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_ADC_A::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Peripheral clock selection for CAN1.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_CAN1_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 6. PCLK_peripheral = CCLK/6."]
    CCLK_DIV_6 = 3,
}
impl From<PCLK_CAN1_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_CAN1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_CAN1`"]
pub type PCLK_CAN1_R = crate::R<u8, PCLK_CAN1_A>;
impl PCLK_CAN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_CAN1_A {
        match self.bits {
            0 => PCLK_CAN1_A::CCLK_DIV_4,
            1 => PCLK_CAN1_A::CCLK,
            2 => PCLK_CAN1_A::CCLK_DIV_2,
            3 => PCLK_CAN1_A::CCLK_DIV_6,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_CAN1_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_CAN1_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_CAN1_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_6`"]
    #[inline(always)]
    pub fn is_cclk_div_6(&self) -> bool {
        *self == PCLK_CAN1_A::CCLK_DIV_6
    }
}
#[doc = "Write proxy for field `PCLK_CAN1`"]
pub struct PCLK_CAN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_CAN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_CAN1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_CAN1_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_CAN1_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_CAN1_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6."]
    #[inline(always)]
    pub fn cclk_div_6(self) -> &'a mut W {
        self.variant(PCLK_CAN1_A::CCLK_DIV_6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Peripheral clock selection for CAN2.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_CAN2_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 6. PCLK_peripheral = CCLK/6,"]
    CCLK_DIV_6 = 3,
}
impl From<PCLK_CAN2_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_CAN2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_CAN2`"]
pub type PCLK_CAN2_R = crate::R<u8, PCLK_CAN2_A>;
impl PCLK_CAN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_CAN2_A {
        match self.bits {
            0 => PCLK_CAN2_A::CCLK_DIV_4,
            1 => PCLK_CAN2_A::CCLK,
            2 => PCLK_CAN2_A::CCLK_DIV_2,
            3 => PCLK_CAN2_A::CCLK_DIV_6,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_CAN2_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_CAN2_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_CAN2_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_6`"]
    #[inline(always)]
    pub fn is_cclk_div_6(&self) -> bool {
        *self == PCLK_CAN2_A::CCLK_DIV_6
    }
}
#[doc = "Write proxy for field `PCLK_CAN2`"]
pub struct PCLK_CAN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_CAN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_CAN2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_CAN2_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_CAN2_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_CAN2_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6,"]
    #[inline(always)]
    pub fn cclk_div_6(self) -> &'a mut W {
        self.variant(PCLK_CAN2_A::CCLK_DIV_6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Peripheral clock selection for CAN acceptance filtering.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCLK_ACF_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 6. PCLK_peripheral = CCLK/6"]
    CCLK_DIV_6 = 3,
}
impl From<PCLK_ACF_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_ACF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCLK_ACF`"]
pub type PCLK_ACF_R = crate::R<u8, PCLK_ACF_A>;
impl PCLK_ACF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_ACF_A {
        match self.bits {
            0 => PCLK_ACF_A::CCLK_DIV_4,
            1 => PCLK_ACF_A::CCLK,
            2 => PCLK_ACF_A::CCLK_DIV_2,
            3 => PCLK_ACF_A::CCLK_DIV_6,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_ACF_A::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_ACF_A::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_ACF_A::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_6`"]
    #[inline(always)]
    pub fn is_cclk_div_6(&self) -> bool {
        *self == PCLK_ACF_A::CCLK_DIV_6
    }
}
#[doc = "Write proxy for field `PCLK_ACF`"]
pub struct PCLK_ACF_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_ACF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_ACF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_ACF_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_ACF_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_ACF_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6"]
    #[inline(always)]
    pub fn cclk_div_6(self) -> &'a mut W {
        self.variant(PCLK_ACF_A::CCLK_DIV_6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Peripheral clock selection for WDT."]
    #[inline(always)]
    pub fn pclk_wdt(&self) -> PCLK_WDT_R {
        PCLK_WDT_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Peripheral clock selection for TIMER0."]
    #[inline(always)]
    pub fn pclk_timer0(&self) -> PCLK_TIMER0_R {
        PCLK_TIMER0_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Peripheral clock selection for TIMER1."]
    #[inline(always)]
    pub fn pclk_timer1(&self) -> PCLK_TIMER1_R {
        PCLK_TIMER1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Peripheral clock selection for UART0."]
    #[inline(always)]
    pub fn pclk_uart0(&self) -> PCLK_UART0_R {
        PCLK_UART0_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Peripheral clock selection for UART1."]
    #[inline(always)]
    pub fn pclk_uart1(&self) -> PCLK_UART1_R {
        PCLK_UART1_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Peripheral clock selection for PWM1."]
    #[inline(always)]
    pub fn pclk_pwm1(&self) -> PCLK_PWM1_R {
        PCLK_PWM1_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Peripheral clock selection for I2C0."]
    #[inline(always)]
    pub fn pclk_i2c0(&self) -> PCLK_I2C0_R {
        PCLK_I2C0_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Peripheral clock selection for SPI."]
    #[inline(always)]
    pub fn pclk_spi(&self) -> PCLK_SPI_R {
        PCLK_SPI_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Peripheral clock selection for SSP1."]
    #[inline(always)]
    pub fn pclk_ssp1(&self) -> PCLK_SSP1_R {
        PCLK_SSP1_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Peripheral clock selection for DAC."]
    #[inline(always)]
    pub fn pclk_dac(&self) -> PCLK_DAC_R {
        PCLK_DAC_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Peripheral clock selection for ADC."]
    #[inline(always)]
    pub fn pclk_adc(&self) -> PCLK_ADC_R {
        PCLK_ADC_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Peripheral clock selection for CAN1.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    pub fn pclk_can1(&self) -> PCLK_CAN1_R {
        PCLK_CAN1_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Peripheral clock selection for CAN2.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    pub fn pclk_can2(&self) -> PCLK_CAN2_R {
        PCLK_CAN2_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Peripheral clock selection for CAN acceptance filtering.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    pub fn pclk_acf(&self) -> PCLK_ACF_R {
        PCLK_ACF_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Peripheral clock selection for WDT."]
    #[inline(always)]
    pub fn pclk_wdt(&mut self) -> PCLK_WDT_W {
        PCLK_WDT_W { w: self }
    }
    #[doc = "Bits 2:3 - Peripheral clock selection for TIMER0."]
    #[inline(always)]
    pub fn pclk_timer0(&mut self) -> PCLK_TIMER0_W {
        PCLK_TIMER0_W { w: self }
    }
    #[doc = "Bits 4:5 - Peripheral clock selection for TIMER1."]
    #[inline(always)]
    pub fn pclk_timer1(&mut self) -> PCLK_TIMER1_W {
        PCLK_TIMER1_W { w: self }
    }
    #[doc = "Bits 6:7 - Peripheral clock selection for UART0."]
    #[inline(always)]
    pub fn pclk_uart0(&mut self) -> PCLK_UART0_W {
        PCLK_UART0_W { w: self }
    }
    #[doc = "Bits 8:9 - Peripheral clock selection for UART1."]
    #[inline(always)]
    pub fn pclk_uart1(&mut self) -> PCLK_UART1_W {
        PCLK_UART1_W { w: self }
    }
    #[doc = "Bits 12:13 - Peripheral clock selection for PWM1."]
    #[inline(always)]
    pub fn pclk_pwm1(&mut self) -> PCLK_PWM1_W {
        PCLK_PWM1_W { w: self }
    }
    #[doc = "Bits 14:15 - Peripheral clock selection for I2C0."]
    #[inline(always)]
    pub fn pclk_i2c0(&mut self) -> PCLK_I2C0_W {
        PCLK_I2C0_W { w: self }
    }
    #[doc = "Bits 16:17 - Peripheral clock selection for SPI."]
    #[inline(always)]
    pub fn pclk_spi(&mut self) -> PCLK_SPI_W {
        PCLK_SPI_W { w: self }
    }
    #[doc = "Bits 20:21 - Peripheral clock selection for SSP1."]
    #[inline(always)]
    pub fn pclk_ssp1(&mut self) -> PCLK_SSP1_W {
        PCLK_SSP1_W { w: self }
    }
    #[doc = "Bits 22:23 - Peripheral clock selection for DAC."]
    #[inline(always)]
    pub fn pclk_dac(&mut self) -> PCLK_DAC_W {
        PCLK_DAC_W { w: self }
    }
    #[doc = "Bits 24:25 - Peripheral clock selection for ADC."]
    #[inline(always)]
    pub fn pclk_adc(&mut self) -> PCLK_ADC_W {
        PCLK_ADC_W { w: self }
    }
    #[doc = "Bits 26:27 - Peripheral clock selection for CAN1.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    pub fn pclk_can1(&mut self) -> PCLK_CAN1_W {
        PCLK_CAN1_W { w: self }
    }
    #[doc = "Bits 28:29 - Peripheral clock selection for CAN2.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    pub fn pclk_can2(&mut self) -> PCLK_CAN2_W {
        PCLK_CAN2_W { w: self }
    }
    #[doc = "Bits 30:31 - Peripheral clock selection for CAN acceptance filtering.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    pub fn pclk_acf(&mut self) -> PCLK_ACF_W {
        PCLK_ACF_W { w: self }
    }
}
