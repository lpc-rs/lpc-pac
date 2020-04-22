#[doc = "Reader of register PINSEL7"]
pub type R = crate::R<u32, super::PINSEL7>;
#[doc = "Writer for register PINSEL7"]
pub type W = crate::W<u32, super::PINSEL7>;
#[doc = "Register PINSEL7 `reset()`'s with value 0"]
impl crate::ResetValue for super::PINSEL7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pin function select P3.25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P3_25_A {
    #[doc = "0: GPIO P3.25"]
    GPIO_P3 = 0,
    #[doc = "2: MAT0.0"]
    MAT0 = 2,
    #[doc = "3: PWM1.2"]
    PWM1 = 3,
}
impl From<P3_25_A> for u8 {
    #[inline(always)]
    fn from(variant: P3_25_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P3_25`"]
pub type P3_25_R = crate::R<u8, P3_25_A>;
impl P3_25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P3_25_A {
        match self.bits {
            0 => P3_25_A::GPIO_P3,
            2 => P3_25_A::MAT0,
            3 => P3_25_A::PWM1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P3`"]
    #[inline(always)]
    pub fn is_gpio_p3(&self) -> bool {
        *self == P3_25_A::GPIO_P3
    }
    #[doc = "Checks if the value of the field is `MAT0`"]
    #[inline(always)]
    pub fn is_mat0(&self) -> bool {
        *self == P3_25_A::MAT0
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P3_25_A::PWM1
    }
}
#[doc = "Write proxy for field `P3_25`"]
pub struct P3_25_W<'a> {
    w: &'a mut W,
}
impl<'a> P3_25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P3_25_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P3.25"]
    #[inline(always)]
    pub fn gpio_p3(self) -> &'a mut W {
        self.variant(P3_25_A::GPIO_P3)
    }
    #[doc = "MAT0.0"]
    #[inline(always)]
    pub fn mat0(self) -> &'a mut W {
        self.variant(P3_25_A::MAT0)
    }
    #[doc = "PWM1.2"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P3_25_A::PWM1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Pin function select P3.26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P3_26_A {
    #[doc = "0: GPIO P3.26"]
    GPIO_P3 = 0,
    #[doc = "1: STCLK"]
    STCLK = 1,
    #[doc = "2: MAT0.1"]
    MAT0 = 2,
    #[doc = "3: PWM1.3"]
    PWM1 = 3,
}
impl From<P3_26_A> for u8 {
    #[inline(always)]
    fn from(variant: P3_26_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P3_26`"]
pub type P3_26_R = crate::R<u8, P3_26_A>;
impl P3_26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P3_26_A {
        match self.bits {
            0 => P3_26_A::GPIO_P3,
            1 => P3_26_A::STCLK,
            2 => P3_26_A::MAT0,
            3 => P3_26_A::PWM1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P3`"]
    #[inline(always)]
    pub fn is_gpio_p3(&self) -> bool {
        *self == P3_26_A::GPIO_P3
    }
    #[doc = "Checks if the value of the field is `STCLK`"]
    #[inline(always)]
    pub fn is_stclk(&self) -> bool {
        *self == P3_26_A::STCLK
    }
    #[doc = "Checks if the value of the field is `MAT0`"]
    #[inline(always)]
    pub fn is_mat0(&self) -> bool {
        *self == P3_26_A::MAT0
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P3_26_A::PWM1
    }
}
#[doc = "Write proxy for field `P3_26`"]
pub struct P3_26_W<'a> {
    w: &'a mut W,
}
impl<'a> P3_26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P3_26_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P3.26"]
    #[inline(always)]
    pub fn gpio_p3(self) -> &'a mut W {
        self.variant(P3_26_A::GPIO_P3)
    }
    #[doc = "STCLK"]
    #[inline(always)]
    pub fn stclk(self) -> &'a mut W {
        self.variant(P3_26_A::STCLK)
    }
    #[doc = "MAT0.1"]
    #[inline(always)]
    pub fn mat0(self) -> &'a mut W {
        self.variant(P3_26_A::MAT0)
    }
    #[doc = "PWM1.3"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P3_26_A::PWM1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 18:19 - Pin function select P3.25."]
    #[inline(always)]
    pub fn p3_25(&self) -> P3_25_R {
        P3_25_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Pin function select P3.26."]
    #[inline(always)]
    pub fn p3_26(&self) -> P3_26_R {
        P3_26_R::new(((self.bits >> 20) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 18:19 - Pin function select P3.25."]
    #[inline(always)]
    pub fn p3_25(&mut self) -> P3_25_W {
        P3_25_W { w: self }
    }
    #[doc = "Bits 20:21 - Pin function select P3.26."]
    #[inline(always)]
    pub fn p3_26(&mut self) -> P3_26_W {
        P3_26_W { w: self }
    }
}
