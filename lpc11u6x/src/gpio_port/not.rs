#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NOT {
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
#[doc = r"Proxy"]
pub struct _NOTP00W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP00W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP01W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP01W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP02W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP02W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP03W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP03W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP04W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP04W<'a> {
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
#[doc = r"Proxy"]
pub struct _NOTP05W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP05W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP06W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP06W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP07W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP07W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP08W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP08W<'a> {
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
#[doc = r"Proxy"]
pub struct _NOTP09W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP09W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP010W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP010W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP011W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP011W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP012W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP012W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP013W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP013W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP014W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP014W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP015W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP015W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP016W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP016W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP017W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP017W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP018W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP018W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP019W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP019W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP020W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP020W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP021W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP021W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP022W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP022W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP023W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP023W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP024W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP024W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP025W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP025W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP026W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP026W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP027W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP027W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP028W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP028W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP029W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP029W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP030W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP030W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _NOTP031W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP031W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp00(&mut self) -> _NOTP00W {
        _NOTP00W { w: self }
    }
    #[doc = "Bit 1 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp01(&mut self) -> _NOTP01W {
        _NOTP01W { w: self }
    }
    #[doc = "Bit 2 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp02(&mut self) -> _NOTP02W {
        _NOTP02W { w: self }
    }
    #[doc = "Bit 3 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp03(&mut self) -> _NOTP03W {
        _NOTP03W { w: self }
    }
    #[doc = "Bit 4 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp04(&mut self) -> _NOTP04W {
        _NOTP04W { w: self }
    }
    #[doc = "Bit 5 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp05(&mut self) -> _NOTP05W {
        _NOTP05W { w: self }
    }
    #[doc = "Bit 6 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp06(&mut self) -> _NOTP06W {
        _NOTP06W { w: self }
    }
    #[doc = "Bit 7 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp07(&mut self) -> _NOTP07W {
        _NOTP07W { w: self }
    }
    #[doc = "Bit 8 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp08(&mut self) -> _NOTP08W {
        _NOTP08W { w: self }
    }
    #[doc = "Bit 9 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp09(&mut self) -> _NOTP09W {
        _NOTP09W { w: self }
    }
    #[doc = "Bit 10 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp010(&mut self) -> _NOTP010W {
        _NOTP010W { w: self }
    }
    #[doc = "Bit 11 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp011(&mut self) -> _NOTP011W {
        _NOTP011W { w: self }
    }
    #[doc = "Bit 12 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp012(&mut self) -> _NOTP012W {
        _NOTP012W { w: self }
    }
    #[doc = "Bit 13 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp013(&mut self) -> _NOTP013W {
        _NOTP013W { w: self }
    }
    #[doc = "Bit 14 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp014(&mut self) -> _NOTP014W {
        _NOTP014W { w: self }
    }
    #[doc = "Bit 15 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp015(&mut self) -> _NOTP015W {
        _NOTP015W { w: self }
    }
    #[doc = "Bit 16 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp016(&mut self) -> _NOTP016W {
        _NOTP016W { w: self }
    }
    #[doc = "Bit 17 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp017(&mut self) -> _NOTP017W {
        _NOTP017W { w: self }
    }
    #[doc = "Bit 18 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp018(&mut self) -> _NOTP018W {
        _NOTP018W { w: self }
    }
    #[doc = "Bit 19 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp019(&mut self) -> _NOTP019W {
        _NOTP019W { w: self }
    }
    #[doc = "Bit 20 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp020(&mut self) -> _NOTP020W {
        _NOTP020W { w: self }
    }
    #[doc = "Bit 21 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp021(&mut self) -> _NOTP021W {
        _NOTP021W { w: self }
    }
    #[doc = "Bit 22 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp022(&mut self) -> _NOTP022W {
        _NOTP022W { w: self }
    }
    #[doc = "Bit 23 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp023(&mut self) -> _NOTP023W {
        _NOTP023W { w: self }
    }
    #[doc = "Bit 24 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp024(&mut self) -> _NOTP024W {
        _NOTP024W { w: self }
    }
    #[doc = "Bit 25 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp025(&mut self) -> _NOTP025W {
        _NOTP025W { w: self }
    }
    #[doc = "Bit 26 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp026(&mut self) -> _NOTP026W {
        _NOTP026W { w: self }
    }
    #[doc = "Bit 27 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp027(&mut self) -> _NOTP027W {
        _NOTP027W { w: self }
    }
    #[doc = "Bit 28 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp028(&mut self) -> _NOTP028W {
        _NOTP028W { w: self }
    }
    #[doc = "Bit 29 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp029(&mut self) -> _NOTP029W {
        _NOTP029W { w: self }
    }
    #[doc = "Bit 30 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp030(&mut self) -> _NOTP030W {
        _NOTP030W { w: self }
    }
    #[doc = "Bit 31 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp031(&mut self) -> _NOTP031W {
        _NOTP031W { w: self }
    }
}
