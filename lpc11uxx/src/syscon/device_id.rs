#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DEVICE_ID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DEVICEIDR {
    bits: u32,
}
impl DEVICEIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Device ID numbers for LPC11Uxx parts LPC11U12FHN33/201 = 0x095C 802B/0x295C 802B LPC11U12FBD48/201 = 0x095C 802B/0x295C 802B LPC11U13FBD48/201 = 0x097A 802B/0x297A 802B LPC11U14FHN33/201 = 0x0998 802B/0x2998 802B LPC11U14FHI33/201 = 0x2998 802B LPC11U14FBD48/201 = 0x0998 802B/0x2998 802B LPC11U14FET48/201 = 0x0998 802B/0x2998 802B LPC11U23FBD48/301 = 0x2972 402B LPC11U24FHI33/301 = 0x2988 402B LPC11U24FBD48/301 = 0x2988 402B LPC11U24FET48/301 = 0x2988 402B LPC11U24FHN33/401 = 0x2980 002B LPC11U24FBD48/401 = 0x2980 002B LPC11U24FBD64/401 = 0x2980 002B"]
    #[inline]
    pub fn deviceid(&self) -> DEVICEIDR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        DEVICEIDR { bits }
    }
}
