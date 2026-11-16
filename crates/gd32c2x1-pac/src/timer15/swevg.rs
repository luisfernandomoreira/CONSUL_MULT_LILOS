#[doc = "Register `SWEVG` writer"]
pub type W = crate::W<SwevgSpec>;
#[doc = "Field `UPG` writer - Update event generation"]
pub type UpgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0G` writer - Channel 0 capture or compare event generation"]
pub type Ch0gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMTG` writer - Channel commutation event generation"]
pub type CmtgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRK0G` writer - Break 0 event generation"]
pub type Brk0gW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Update event generation"]
    #[inline(always)]
    pub fn upg(&mut self) -> UpgW<'_, SwevgSpec> {
        UpgW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 0 capture or compare event generation"]
    #[inline(always)]
    pub fn ch0g(&mut self) -> Ch0gW<'_, SwevgSpec> {
        Ch0gW::new(self, 1)
    }
    #[doc = "Bit 5 - Channel commutation event generation"]
    #[inline(always)]
    pub fn cmtg(&mut self) -> CmtgW<'_, SwevgSpec> {
        CmtgW::new(self, 5)
    }
    #[doc = "Bit 7 - Break 0 event generation"]
    #[inline(always)]
    pub fn brk0g(&mut self) -> Brk0gW<'_, SwevgSpec> {
        Brk0gW::new(self, 7)
    }
}
#[doc = "Software event generation register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swevg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwevgSpec;
impl crate::RegisterSpec for SwevgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swevg::W`](W) writer structure"]
impl crate::Writable for SwevgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWEVG to value 0"]
impl crate::Resettable for SwevgSpec {}
