#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MODE {
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
pub type CRC_POLY_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _CRC_POLYW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC_POLYW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type BIT_RVS_WR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _BIT_RVS_WRW<'a> {
    w: &'a mut W,
}
impl<'a> _BIT_RVS_WRW<'a> {
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
#[doc = r"Reader of the field"]
pub type CMPL_WR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CMPL_WRW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPL_WRW<'a> {
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
#[doc = r"Reader of the field"]
pub type BIT_RVS_SUM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _BIT_RVS_SUMW<'a> {
    w: &'a mut W,
}
impl<'a> _BIT_RVS_SUMW<'a> {
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
#[doc = r"Reader of the field"]
pub type CMPL_SUM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CMPL_SUMW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPL_SUMW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - CRC polynom: 1X= CRC-32 polynomial 01= CRC-16 polynomial 00= CRC-CCITT polynomial"]
    #[inline(always)]
    pub fn crc_poly(&self) -> CRC_POLY_R {
        CRC_POLY_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bit 2 - Data bit order: 1= Bit order reverse for CRC_WR_DATA (per byte) 0= No bit order reverse for CRC_WR_DATA (per byte)"]
    #[inline(always)]
    pub fn bit_rvs_wr(&self) -> BIT_RVS_WR_R {
        BIT_RVS_WR_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data complement: 1= 1's complement for CRC_WR_DATA 0= No 1's complement for CRC_WR_DATA"]
    #[inline(always)]
    pub fn cmpl_wr(&self) -> CMPL_WR_R {
        CMPL_WR_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRC sum bit order: 1= Bit order reverse for CRC_SUM 0= No bit order reverse for CRC_SUM"]
    #[inline(always)]
    pub fn bit_rvs_sum(&self) -> BIT_RVS_SUM_R {
        BIT_RVS_SUM_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CRC sum complement: 1= 1's complement for CRC_SUM 0=No 1's complement for CRC_SUM"]
    #[inline(always)]
    pub fn cmpl_sum(&self) -> CMPL_SUM_R {
        CMPL_SUM_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - CRC polynom: 1X= CRC-32 polynomial 01= CRC-16 polynomial 00= CRC-CCITT polynomial"]
    #[inline(always)]
    pub fn crc_poly(&mut self) -> _CRC_POLYW {
        _CRC_POLYW { w: self }
    }
    #[doc = "Bit 2 - Data bit order: 1= Bit order reverse for CRC_WR_DATA (per byte) 0= No bit order reverse for CRC_WR_DATA (per byte)"]
    #[inline(always)]
    pub fn bit_rvs_wr(&mut self) -> _BIT_RVS_WRW {
        _BIT_RVS_WRW { w: self }
    }
    #[doc = "Bit 3 - Data complement: 1= 1's complement for CRC_WR_DATA 0= No 1's complement for CRC_WR_DATA"]
    #[inline(always)]
    pub fn cmpl_wr(&mut self) -> _CMPL_WRW {
        _CMPL_WRW { w: self }
    }
    #[doc = "Bit 4 - CRC sum bit order: 1= Bit order reverse for CRC_SUM 0= No bit order reverse for CRC_SUM"]
    #[inline(always)]
    pub fn bit_rvs_sum(&mut self) -> _BIT_RVS_SUMW {
        _BIT_RVS_SUMW { w: self }
    }
    #[doc = "Bit 5 - CRC sum complement: 1= 1's complement for CRC_SUM 0=No 1's complement for CRC_SUM"]
    #[inline(always)]
    pub fn cmpl_sum(&mut self) -> _CMPL_SUMW {
        _CMPL_SUMW { w: self }
    }
}
