#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Field `WD0E` reader - Analog watchdog 0 event flag"]
pub type Wd0eR = crate::BitReader;
#[doc = "Field `WD0E` writer - Analog watchdog 0 event flag"]
pub type Wd0eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC` reader - End flag of sequence conversion"]
pub type EocR = crate::BitReader;
#[doc = "Field `EOC` writer - End flag of sequence conversion"]
pub type EocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOIC` reader - End flag of inserted sequence conversion"]
pub type EoicR = crate::BitReader;
#[doc = "Field `EOIC` writer - End flag of inserted sequence conversion"]
pub type EoicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STIC` reader - Start flag of inserted sequence"]
pub type SticR = crate::BitReader;
#[doc = "Field `STIC` writer - Start flag of inserted sequence"]
pub type SticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRC` reader - Start flag of routine sequence"]
pub type StrcR = crate::BitReader;
#[doc = "Field `STRC` writer - Start flag of routine sequence"]
pub type StrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WD1E` reader - Analog watchdog 1 event flag"]
pub type Wd1eR = crate::BitReader;
#[doc = "Field `WD1E` writer - Analog watchdog 1 event flag"]
pub type Wd1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WD2E` reader - Analog watchdog 2 event flag"]
pub type Wd2eR = crate::BitReader;
#[doc = "Field `WD2E` writer - Analog watchdog 2 event flag"]
pub type Wd2eW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Analog watchdog 0 event flag"]
    #[inline(always)]
    pub fn wd0e(&self) -> Wd0eR {
        Wd0eR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End flag of sequence conversion"]
    #[inline(always)]
    pub fn eoc(&self) -> EocR {
        EocR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End flag of inserted sequence conversion"]
    #[inline(always)]
    pub fn eoic(&self) -> EoicR {
        EoicR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start flag of inserted sequence"]
    #[inline(always)]
    pub fn stic(&self) -> SticR {
        SticR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Start flag of routine sequence"]
    #[inline(always)]
    pub fn strc(&self) -> StrcR {
        StrcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 30 - Analog watchdog 1 event flag"]
    #[inline(always)]
    pub fn wd1e(&self) -> Wd1eR {
        Wd1eR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Analog watchdog 2 event flag"]
    #[inline(always)]
    pub fn wd2e(&self) -> Wd2eR {
        Wd2eR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog watchdog 0 event flag"]
    #[inline(always)]
    pub fn wd0e(&mut self) -> Wd0eW<'_, StatSpec> {
        Wd0eW::new(self, 0)
    }
    #[doc = "Bit 1 - End flag of sequence conversion"]
    #[inline(always)]
    pub fn eoc(&mut self) -> EocW<'_, StatSpec> {
        EocW::new(self, 1)
    }
    #[doc = "Bit 2 - End flag of inserted sequence conversion"]
    #[inline(always)]
    pub fn eoic(&mut self) -> EoicW<'_, StatSpec> {
        EoicW::new(self, 2)
    }
    #[doc = "Bit 3 - Start flag of inserted sequence"]
    #[inline(always)]
    pub fn stic(&mut self) -> SticW<'_, StatSpec> {
        SticW::new(self, 3)
    }
    #[doc = "Bit 4 - Start flag of routine sequence"]
    #[inline(always)]
    pub fn strc(&mut self) -> StrcW<'_, StatSpec> {
        StrcW::new(self, 4)
    }
    #[doc = "Bit 30 - Analog watchdog 1 event flag"]
    #[inline(always)]
    pub fn wd1e(&mut self) -> Wd1eW<'_, StatSpec> {
        Wd1eW::new(self, 30)
    }
    #[doc = "Bit 31 - Analog watchdog 2 event flag"]
    #[inline(always)]
    pub fn wd2e(&mut self) -> Wd2eW<'_, StatSpec> {
        Wd2eW::new(self, 31)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {}
