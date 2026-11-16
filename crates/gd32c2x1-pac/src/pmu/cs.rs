#[doc = "Register `CS` reader"]
pub type R = crate::R<CsSpec>;
#[doc = "Register `CS` writer"]
pub type W = crate::W<CsSpec>;
#[doc = "Field `WUF` reader - Wakeup Flag"]
pub type WufR = crate::BitReader;
#[doc = "Field `STBF` reader - Standby Flag"]
pub type StbfR = crate::BitReader;
#[doc = "Field `WUPEN0` reader - WKUP Pin0(PA0) enable"]
pub type Wupen0R = crate::BitReader;
#[doc = "Field `WUPEN0` writer - WKUP Pin0(PA0) enable"]
pub type Wupen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUPEN1` reader - WKUP Pin1(PC13 / PA4) enable"]
pub type Wupen1R = crate::BitReader;
#[doc = "Field `WUPEN1` writer - WKUP Pin1(PC13 / PA4) enable"]
pub type Wupen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUPEN2` reader - WKUP Pin2 (PB6) enable"]
pub type Wupen2R = crate::BitReader;
#[doc = "Field `WUPEN2` writer - WKUP Pin2 (PB6) enable"]
pub type Wupen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUPEN3` reader - WKUP Pin3 (PA2) enable"]
pub type Wupen3R = crate::BitReader;
#[doc = "Field `WUPEN3` writer - WKUP Pin3 (PA2) enable"]
pub type Wupen3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUPEN5` reader - WKUP Pin5 (PB5) Enable"]
pub type Wupen5R = crate::BitReader;
#[doc = "Field `WUPEN5` writer - WKUP Pin5 (PB5) Enable"]
pub type Wupen5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDOVSRF` reader - LDO voltage select ready flag."]
pub type LdovsrfR = crate::BitReader;
#[doc = "Field `NPRDY` reader - NPLDO ready flag"]
pub type NprdyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Wakeup Flag"]
    #[inline(always)]
    pub fn wuf(&self) -> WufR {
        WufR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Standby Flag"]
    #[inline(always)]
    pub fn stbf(&self) -> StbfR {
        StbfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - WKUP Pin0(PA0) enable"]
    #[inline(always)]
    pub fn wupen0(&self) -> Wupen0R {
        Wupen0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WKUP Pin1(PC13 / PA4) enable"]
    #[inline(always)]
    pub fn wupen1(&self) -> Wupen1R {
        Wupen1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - WKUP Pin2 (PB6) enable"]
    #[inline(always)]
    pub fn wupen2(&self) -> Wupen2R {
        Wupen2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WKUP Pin3 (PA2) enable"]
    #[inline(always)]
    pub fn wupen3(&self) -> Wupen3R {
        Wupen3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - WKUP Pin5 (PB5) Enable"]
    #[inline(always)]
    pub fn wupen5(&self) -> Wupen5R {
        Wupen5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LDO voltage select ready flag."]
    #[inline(always)]
    pub fn ldovsrf(&self) -> LdovsrfR {
        LdovsrfR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - NPLDO ready flag"]
    #[inline(always)]
    pub fn nprdy(&self) -> NprdyR {
        NprdyR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - WKUP Pin0(PA0) enable"]
    #[inline(always)]
    pub fn wupen0(&mut self) -> Wupen0W<'_, CsSpec> {
        Wupen0W::new(self, 8)
    }
    #[doc = "Bit 9 - WKUP Pin1(PC13 / PA4) enable"]
    #[inline(always)]
    pub fn wupen1(&mut self) -> Wupen1W<'_, CsSpec> {
        Wupen1W::new(self, 9)
    }
    #[doc = "Bit 10 - WKUP Pin2 (PB6) enable"]
    #[inline(always)]
    pub fn wupen2(&mut self) -> Wupen2W<'_, CsSpec> {
        Wupen2W::new(self, 10)
    }
    #[doc = "Bit 11 - WKUP Pin3 (PA2) enable"]
    #[inline(always)]
    pub fn wupen3(&mut self) -> Wupen3W<'_, CsSpec> {
        Wupen3W::new(self, 11)
    }
    #[doc = "Bit 13 - WKUP Pin5 (PB5) Enable"]
    #[inline(always)]
    pub fn wupen5(&mut self) -> Wupen5W<'_, CsSpec> {
        Wupen5W::new(self, 13)
    }
}
#[doc = "Control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsSpec;
impl crate::RegisterSpec for CsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cs::R`](R) reader structure"]
impl crate::Readable for CsSpec {}
#[doc = "`write(|w| ..)` method takes [`cs::W`](W) writer structure"]
impl crate::Writable for CsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CS to value 0"]
impl crate::Resettable for CsSpec {}
