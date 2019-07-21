#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::END {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct ENDR {
    bits: u32,
}
impl ENDR {
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
    #[doc = "Bits 0:31 - End Marker"]
    #[inline]
    pub fn end(&self) -> ENDR {
        let bits = ((self.bits >> 0) & 0xffff_ffff) as u32;
        ENDR { bits }
    }
}
