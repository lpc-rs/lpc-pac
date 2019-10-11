#[doc = "Reader of register TIMEOUT"]
pub type R = crate::R<u32, super::TIMEOUT>;
#[doc = "Writer for register TIMEOUT"]
pub type W = crate::W<u32, super::TIMEOUT>;
#[doc = "Register TIMEOUT `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::TIMEOUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `TOMIN`"]
pub type TOMIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TOMIN`"]
pub struct TOMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOMIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `TO`"]
pub type TO_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TO`"]
pub struct TO_W<'a> {
    w: &'a mut W,
}
impl<'a> TO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | (((value as u32) & 0x0fff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Time-out time value, bottom four bits. These are hard-wired to 0xF. This gives a minimum time-out of 16 I2C function clocks and also a time-out resolution of 16 I2C function clocks."]
    #[inline(always)]
    pub fn tomin(&self) -> TOMIN_R {
        TOMIN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Time-out time value. Specifies the time-out interval value in increments of 16 I 2C function clocks, as defined by the CLKDIV register. To change this value while I2C is in operation, disable all time-outs, write a new value to TIMEOUT, then re-enable time-outs. 0x000 = A time-out will occur after 16 counts of the I2C function clock. 0x001 = A time-out will occur after 32 counts of the I2C function clock. 0xFFF = A time-out will occur after 65,536 counts of the I2C function clock."]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Time-out time value, bottom four bits. These are hard-wired to 0xF. This gives a minimum time-out of 16 I2C function clocks and also a time-out resolution of 16 I2C function clocks."]
    #[inline(always)]
    pub fn tomin(&mut self) -> TOMIN_W {
        TOMIN_W { w: self }
    }
    #[doc = "Bits 4:15 - Time-out time value. Specifies the time-out interval value in increments of 16 I 2C function clocks, as defined by the CLKDIV register. To change this value while I2C is in operation, disable all time-outs, write a new value to TIMEOUT, then re-enable time-outs. 0x000 = A time-out will occur after 16 counts of the I2C function clock. 0x001 = A time-out will occur after 32 counts of the I2C function clock. 0xFFF = A time-out will occur after 65,536 counts of the I2C function clock."]
    #[inline(always)]
    pub fn to(&mut self) -> TO_W {
        TO_W { w: self }
    }
}
