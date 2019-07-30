#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BODCTRL {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `BODRSTLEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRSTLEVR {
    #[doc = "Level 0."]
    LEVEL_0,
    #[doc = "Level 1."]
    LEVEL_1,
    #[doc = "Level 2."]
    LEVEL_2,
    #[doc = "Level 3."]
    LEVEL_3,
}
impl crate::ToBits<u8> for BODRSTLEVR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            BODRSTLEVR::LEVEL_0 => 0,
            BODRSTLEVR::LEVEL_1 => 1,
            BODRSTLEVR::LEVEL_2 => 2,
            BODRSTLEVR::LEVEL_3 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BODRSTLEV_R = crate::FR<u8, BODRSTLEVR>;
impl BODRSTLEV_R {
    #[doc = "Checks if the value of the field is `LEVEL_0`"]
    #[inline(always)]
    pub fn is_level_0(&self) -> bool {
        *self == BODRSTLEVR::LEVEL_0
    }
    #[doc = "Checks if the value of the field is `LEVEL_1`"]
    #[inline(always)]
    pub fn is_level_1(&self) -> bool {
        *self == BODRSTLEVR::LEVEL_1
    }
    #[doc = "Checks if the value of the field is `LEVEL_2`"]
    #[inline(always)]
    pub fn is_level_2(&self) -> bool {
        *self == BODRSTLEVR::LEVEL_2
    }
    #[doc = "Checks if the value of the field is `LEVEL_3`"]
    #[inline(always)]
    pub fn is_level_3(&self) -> bool {
        *self == BODRSTLEVR::LEVEL_3
    }
}
#[doc = "Values that can be written to the field `BODRSTLEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRSTLEVW {
    #[doc = "Level 0."]
    LEVEL_0,
    #[doc = "Level 1."]
    LEVEL_1,
    #[doc = "Level 2."]
    LEVEL_2,
    #[doc = "Level 3."]
    LEVEL_3,
}
impl BODRSTLEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            BODRSTLEVW::LEVEL_0 => 0,
            BODRSTLEVW::LEVEL_1 => 1,
            BODRSTLEVW::LEVEL_2 => 2,
            BODRSTLEVW::LEVEL_3 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _BODRSTLEVW<'a> {
    w: &'a mut W,
}
impl<'a> _BODRSTLEVW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODRSTLEVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Level 0."]
    #[inline(always)]
    pub fn level_0(self) -> &'a mut W {
        self.variant(BODRSTLEVW::LEVEL_0)
    }
    #[doc = "Level 1."]
    #[inline(always)]
    pub fn level_1(self) -> &'a mut W {
        self.variant(BODRSTLEVW::LEVEL_1)
    }
    #[doc = "Level 2."]
    #[inline(always)]
    pub fn level_2(self) -> &'a mut W {
        self.variant(BODRSTLEVW::LEVEL_2)
    }
    #[doc = "Level 3."]
    #[inline(always)]
    pub fn level_3(self) -> &'a mut W {
        self.variant(BODRSTLEVW::LEVEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `BODINTVAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODINTVALR {
    #[doc = "Level 2."]
    LEVEL_2,
    #[doc = "Level 3."]
    LEVEL_3,
}
impl crate::ToBits<u8> for BODINTVALR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            BODINTVALR::LEVEL_2 => 2,
            BODINTVALR::LEVEL_3 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BODINTVAL_R = crate::FR<u8, BODINTVALR>;
impl BODINTVAL_R {
    #[doc = "Checks if the value of the field is `LEVEL_2`"]
    #[inline(always)]
    pub fn is_level_2(&self) -> bool {
        *self == BODINTVALR::LEVEL_2
    }
    #[doc = "Checks if the value of the field is `LEVEL_3`"]
    #[inline(always)]
    pub fn is_level_3(&self) -> bool {
        *self == BODINTVALR::LEVEL_3
    }
}
#[doc = "Values that can be written to the field `BODINTVAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODINTVALW {
    #[doc = "Level 2."]
    LEVEL_2,
    #[doc = "Level 3."]
    LEVEL_3,
}
impl BODINTVALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            BODINTVALW::LEVEL_2 => 2,
            BODINTVALW::LEVEL_3 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _BODINTVALW<'a> {
    w: &'a mut W,
}
impl<'a> _BODINTVALW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODINTVALW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Level 2."]
    #[inline(always)]
    pub fn level_2(self) -> &'a mut W {
        self.variant(BODINTVALW::LEVEL_2)
    }
    #[doc = "Level 3."]
    #[inline(always)]
    pub fn level_3(self) -> &'a mut W {
        self.variant(BODINTVALW::LEVEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `BODRSTENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRSTENAR {
    #[doc = "Disable reset function."]
    DISABLE_RESET_FUNCTI,
    #[doc = "Enable reset function."]
    ENABLE_RESET_FUNCTIO,
}
impl crate::ToBits<bool> for BODRSTENAR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            BODRSTENAR::DISABLE_RESET_FUNCTI => false,
            BODRSTENAR::ENABLE_RESET_FUNCTIO => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BODRSTENA_R = crate::FR<bool, BODRSTENAR>;
impl BODRSTENA_R {
    #[doc = "Checks if the value of the field is `DISABLE_RESET_FUNCTI`"]
    #[inline(always)]
    pub fn is_disable_reset_functi(&self) -> bool {
        *self == BODRSTENAR::DISABLE_RESET_FUNCTI
    }
    #[doc = "Checks if the value of the field is `ENABLE_RESET_FUNCTIO`"]
    #[inline(always)]
    pub fn is_enable_reset_functio(&self) -> bool {
        *self == BODRSTENAR::ENABLE_RESET_FUNCTIO
    }
}
#[doc = "Values that can be written to the field `BODRSTENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRSTENAW {
    #[doc = "Disable reset function."]
    DISABLE_RESET_FUNCTI,
    #[doc = "Enable reset function."]
    ENABLE_RESET_FUNCTIO,
}
impl BODRSTENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            BODRSTENAW::DISABLE_RESET_FUNCTI => false,
            BODRSTENAW::ENABLE_RESET_FUNCTIO => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _BODRSTENAW<'a> {
    w: &'a mut W,
}
impl<'a> _BODRSTENAW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODRSTENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable reset function."]
    #[inline(always)]
    pub fn disable_reset_functi(self) -> &'a mut W {
        self.variant(BODRSTENAW::DISABLE_RESET_FUNCTI)
    }
    #[doc = "Enable reset function."]
    #[inline(always)]
    pub fn enable_reset_functio(self) -> &'a mut W {
        self.variant(BODRSTENAW::ENABLE_RESET_FUNCTIO)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - BOD reset level"]
    #[inline(always)]
    pub fn bodrstlev(&self) -> BODRSTLEV_R {
        BODRSTLEV_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - BOD interrupt level"]
    #[inline(always)]
    pub fn bodintval(&self) -> BODINTVAL_R {
        BODINTVAL_R::new(((self.bits() >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - BOD reset enable"]
    #[inline(always)]
    pub fn bodrstena(&self) -> BODRSTENA_R {
        BODRSTENA_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - BOD reset level"]
    #[inline(always)]
    pub fn bodrstlev(&mut self) -> _BODRSTLEVW {
        _BODRSTLEVW { w: self }
    }
    #[doc = "Bits 2:3 - BOD interrupt level"]
    #[inline(always)]
    pub fn bodintval(&mut self) -> _BODINTVALW {
        _BODINTVALW { w: self }
    }
    #[doc = "Bit 4 - BOD reset enable"]
    #[inline(always)]
    pub fn bodrstena(&mut self) -> _BODRSTENAW {
        _BODRSTENAW { w: self }
    }
}
