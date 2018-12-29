#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINSEL7 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `P3_25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P3_25R {
    #[doc = "GPIO P3.25"]
    GPIO_P3,
    #[doc = "MAT0.0"]
    MAT0,
    #[doc = "PWM1.2"]
    PWM1,
}
impl P3_25R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P3_25R::GPIO_P3 => 0,
            P3_25R::MAT0 => 2,
            P3_25R::PWM1 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P3_25R {
        match value {
            0 => P3_25R::GPIO_P3,
            2 => P3_25R::MAT0,
            3 => P3_25R::PWM1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P3`"]
    #[inline]
    pub fn is_gpio_p3(&self) -> bool {
        *self == P3_25R::GPIO_P3
    }
    #[doc = "Checks if the value of the field is `MAT0`"]
    #[inline]
    pub fn is_mat0(&self) -> bool {
        *self == P3_25R::MAT0
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline]
    pub fn is_pwm1(&self) -> bool {
        *self == P3_25R::PWM1
    }
}
#[doc = "Possible values of the field `P3_26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P3_26R {
    #[doc = "GPIO P3.26"]
    GPIO_P3,
    #[doc = "STCLK"]
    STCLK,
    #[doc = "MAT0.1"]
    MAT0,
    #[doc = "PWM1.3"]
    PWM1,
}
impl P3_26R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P3_26R::GPIO_P3 => 0,
            P3_26R::STCLK => 1,
            P3_26R::MAT0 => 2,
            P3_26R::PWM1 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P3_26R {
        match value {
            0 => P3_26R::GPIO_P3,
            1 => P3_26R::STCLK,
            2 => P3_26R::MAT0,
            3 => P3_26R::PWM1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P3`"]
    #[inline]
    pub fn is_gpio_p3(&self) -> bool {
        *self == P3_26R::GPIO_P3
    }
    #[doc = "Checks if the value of the field is `STCLK`"]
    #[inline]
    pub fn is_stclk(&self) -> bool {
        *self == P3_26R::STCLK
    }
    #[doc = "Checks if the value of the field is `MAT0`"]
    #[inline]
    pub fn is_mat0(&self) -> bool {
        *self == P3_26R::MAT0
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline]
    pub fn is_pwm1(&self) -> bool {
        *self == P3_26R::PWM1
    }
}
#[doc = "Values that can be written to the field `P3_25`"]
pub enum P3_25W {
    #[doc = "GPIO P3.25"]
    GPIO_P3,
    #[doc = "MAT0.0"]
    MAT0,
    #[doc = "PWM1.2"]
    PWM1,
}
impl P3_25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P3_25W::GPIO_P3 => 0,
            P3_25W::MAT0 => 2,
            P3_25W::PWM1 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P3_25W<'a> {
    w: &'a mut W,
}
impl<'a> _P3_25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P3_25W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P3.25"]
    #[inline]
    pub fn gpio_p3(self) -> &'a mut W {
        self.variant(P3_25W::GPIO_P3)
    }
    #[doc = "MAT0.0"]
    #[inline]
    pub fn mat0(self) -> &'a mut W {
        self.variant(P3_25W::MAT0)
    }
    #[doc = "PWM1.2"]
    #[inline]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P3_25W::PWM1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P3_26`"]
pub enum P3_26W {
    #[doc = "GPIO P3.26"]
    GPIO_P3,
    #[doc = "STCLK"]
    STCLK,
    #[doc = "MAT0.1"]
    MAT0,
    #[doc = "PWM1.3"]
    PWM1,
}
impl P3_26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P3_26W::GPIO_P3 => 0,
            P3_26W::STCLK => 1,
            P3_26W::MAT0 => 2,
            P3_26W::PWM1 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P3_26W<'a> {
    w: &'a mut W,
}
impl<'a> _P3_26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P3_26W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P3.26"]
    #[inline]
    pub fn gpio_p3(self) -> &'a mut W {
        self.variant(P3_26W::GPIO_P3)
    }
    #[doc = "STCLK"]
    #[inline]
    pub fn stclk(self) -> &'a mut W {
        self.variant(P3_26W::STCLK)
    }
    #[doc = "MAT0.1"]
    #[inline]
    pub fn mat0(self) -> &'a mut W {
        self.variant(P3_26W::MAT0)
    }
    #[doc = "PWM1.3"]
    #[inline]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P3_26W::PWM1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 18:19 - Pin function select P3.25."]
    #[inline]
    pub fn p3_25(&self) -> P3_25R {
        P3_25R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Pin function select P3.26."]
    #[inline]
    pub fn p3_26(&self) -> P3_26R {
        P3_26R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 18:19 - Pin function select P3.25."]
    #[inline]
    pub fn p3_25(&mut self) -> _P3_25W {
        _P3_25W { w: self }
    }
    #[doc = "Bits 20:21 - Pin function select P3.26."]
    #[inline]
    pub fn p3_26(&mut self) -> _P3_26W {
        _P3_26W { w: self }
    }
}
