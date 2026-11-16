#[doc = "Register `CFG1` reader"]
pub type R = crate::R<Cfg1Spec>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<Cfg1Spec>;
#[doc = "Field `LOCKUP_LOCK` reader - Cortex-M23 LOCKUP output lock"]
pub type LockupLockR = crate::BitReader;
#[doc = "Field `LOCKUP_LOCK` writer - Cortex-M23 LOCKUP output lock"]
pub type LockupLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM_ECC_LOCK` reader - SRAM ECC check error lock"]
pub type SramEccLockR = crate::BitReader;
#[doc = "Field `SRAM_ECC_LOCK` writer - SRAM ECC check error lock"]
pub type SramEccLockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Cortex-M23 LOCKUP output lock"]
    #[inline(always)]
    pub fn lockup_lock(&self) -> LockupLockR {
        LockupLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM ECC check error lock"]
    #[inline(always)]
    pub fn sram_ecc_lock(&self) -> SramEccLockR {
        SramEccLockR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cortex-M23 LOCKUP output lock"]
    #[inline(always)]
    pub fn lockup_lock(&mut self) -> LockupLockW<'_, Cfg1Spec> {
        LockupLockW::new(self, 0)
    }
    #[doc = "Bit 1 - SRAM ECC check error lock"]
    #[inline(always)]
    pub fn sram_ecc_lock(&mut self) -> SramEccLockW<'_, Cfg1Spec> {
        SramEccLockW::new(self, 1)
    }
}
#[doc = "System configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg1Spec;
impl crate::RegisterSpec for Cfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for Cfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for Cfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for Cfg1Spec {}
