#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LPM {
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
#[doc = r"Reader of the field"]
pub type HIRD_HW_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _HIRD_HWW<'a> {
    w: &'a mut W,
}
impl<'a> _HIRD_HWW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type HIRD_SW_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _HIRD_SWW<'a> {
    w: &'a mut W,
}
impl<'a> _HIRD_SWW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DATA_PENDING_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DATA_PENDINGW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_PENDINGW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Host Initiated Resume Duration - HW. This is the HIRD value from the last received LPM token"]
    #[inline(always)]
    pub fn hird_hw(&self) -> HIRD_HW_R {
        HIRD_HW_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Host Initiated Resume Duration - SW. This is the time duration required by the USB device system to come out of LPM initiated suspend after receiving the host initiated LPM resume."]
    #[inline(always)]
    pub fn hird_sw(&self) -> HIRD_SW_R {
        HIRD_SW_R::new(((self.bits() >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - As long as this bit is set to one and LPM supported bit is set to one, HW will return a NYET handshake on every LPM token it receives. If LPM supported bit is set to one and this bit is zero, HW will return an ACK handshake on every LPM token it receives. If SW has still data pending and LPM is supported, it must set this bit to 1."]
    #[inline(always)]
    pub fn data_pending(&self) -> DATA_PENDING_R {
        DATA_PENDING_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Host Initiated Resume Duration - HW. This is the HIRD value from the last received LPM token"]
    #[inline(always)]
    pub fn hird_hw(&mut self) -> _HIRD_HWW {
        _HIRD_HWW { w: self }
    }
    #[doc = "Bits 4:7 - Host Initiated Resume Duration - SW. This is the time duration required by the USB device system to come out of LPM initiated suspend after receiving the host initiated LPM resume."]
    #[inline(always)]
    pub fn hird_sw(&mut self) -> _HIRD_SWW {
        _HIRD_SWW { w: self }
    }
    #[doc = "Bit 8 - As long as this bit is set to one and LPM supported bit is set to one, HW will return a NYET handshake on every LPM token it receives. If LPM supported bit is set to one and this bit is zero, HW will return an ACK handshake on every LPM token it receives. If SW has still data pending and LPM is supported, it must set this bit to 1."]
    #[inline(always)]
    pub fn data_pending(&mut self) -> _DATA_PENDINGW {
        _DATA_PENDINGW { w: self }
    }
}
