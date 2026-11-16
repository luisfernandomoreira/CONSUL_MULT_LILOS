#[doc = "Register `INT` reader"]
pub type R = crate::R<IntSpec>;
#[doc = "Register `INT` writer"]
pub type W = crate::W<IntSpec>;
#[doc = "Field `IRC32KSTBIF` reader - IRC32K stabilization interrupt flag"]
pub type Irc32kstbifR = crate::BitReader;
#[doc = "Field `LXTALSTBIF` reader - LXTAL stabilization interrupt flag"]
pub type LxtalstbifR = crate::BitReader;
#[doc = "Field `IRC48MSTBIF` reader - IRC48M stabilization interrupt flag"]
pub type Irc48mstbifR = crate::BitReader;
#[doc = "Field `HXTALSTBIF` reader - HXTAL stabilization interrupt flag"]
pub type HxtalstbifR = crate::BitReader;
#[doc = "Field `LCKMIF` reader - LXTAL clock stuck interrupt flag"]
pub type LckmifR = crate::BitReader;
#[doc = "Field `CKMIF` reader - HXTAL clock stuck interrupt flag"]
pub type CkmifR = crate::BitReader;
#[doc = "Field `IRC32KSTBIE` reader - IRC32K stabilization interrupt enable"]
pub type Irc32kstbieR = crate::BitReader;
#[doc = "Field `IRC32KSTBIE` writer - IRC32K stabilization interrupt enable"]
pub type Irc32kstbieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LXTALSTBIE` reader - LXTAL stabilization interrupt enable"]
pub type LxtalstbieR = crate::BitReader;
#[doc = "Field `LXTALSTBIE` writer - LXTAL stabilization interrupt enable"]
pub type LxtalstbieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRC48MSTBIE` reader - IRC48M stabilization interrupt enable"]
pub type Irc48mstbieR = crate::BitReader;
#[doc = "Field `IRC48MSTBIE` writer - IRC48M stabilization interrupt enable"]
pub type Irc48mstbieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HXTALSTBIE` reader - HXTAL stabilization interrupt enable"]
pub type HxtalstbieR = crate::BitReader;
#[doc = "Field `HXTALSTBIE` writer - HXTAL stabilization interrupt enable"]
pub type HxtalstbieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRC32KSTBIC` writer - IRC32K stabilization interrupt clear"]
pub type Irc32kstbicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LXTALSTBIC` writer - LXTAL stabilization interrupt clear"]
pub type LxtalstbicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRC48MSTBIC` writer - IRC48M stabilization interrupt clear"]
pub type Irc48mstbicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HXTALSTBIC` writer - HXTAL stabilization interrupt clear"]
pub type HxtalstbicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCKMIC` writer - LXTAL clock stuck interrupt clear"]
pub type LckmicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKMIC` writer - HXTAL clock stuck interrupt clear"]
pub type CkmicW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IRC32K stabilization interrupt flag"]
    #[inline(always)]
    pub fn irc32kstbif(&self) -> Irc32kstbifR {
        Irc32kstbifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LXTAL stabilization interrupt flag"]
    #[inline(always)]
    pub fn lxtalstbif(&self) -> LxtalstbifR {
        LxtalstbifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IRC48M stabilization interrupt flag"]
    #[inline(always)]
    pub fn irc48mstbif(&self) -> Irc48mstbifR {
        Irc48mstbifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HXTAL stabilization interrupt flag"]
    #[inline(always)]
    pub fn hxtalstbif(&self) -> HxtalstbifR {
        HxtalstbifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - LXTAL clock stuck interrupt flag"]
    #[inline(always)]
    pub fn lckmif(&self) -> LckmifR {
        LckmifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HXTAL clock stuck interrupt flag"]
    #[inline(always)]
    pub fn ckmif(&self) -> CkmifR {
        CkmifR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IRC32K stabilization interrupt enable"]
    #[inline(always)]
    pub fn irc32kstbie(&self) -> Irc32kstbieR {
        Irc32kstbieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LXTAL stabilization interrupt enable"]
    #[inline(always)]
    pub fn lxtalstbie(&self) -> LxtalstbieR {
        LxtalstbieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IRC48M stabilization interrupt enable"]
    #[inline(always)]
    pub fn irc48mstbie(&self) -> Irc48mstbieR {
        Irc48mstbieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HXTAL stabilization interrupt enable"]
    #[inline(always)]
    pub fn hxtalstbie(&self) -> HxtalstbieR {
        HxtalstbieR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - IRC32K stabilization interrupt enable"]
    #[inline(always)]
    pub fn irc32kstbie(&mut self) -> Irc32kstbieW<'_, IntSpec> {
        Irc32kstbieW::new(self, 8)
    }
    #[doc = "Bit 9 - LXTAL stabilization interrupt enable"]
    #[inline(always)]
    pub fn lxtalstbie(&mut self) -> LxtalstbieW<'_, IntSpec> {
        LxtalstbieW::new(self, 9)
    }
    #[doc = "Bit 10 - IRC48M stabilization interrupt enable"]
    #[inline(always)]
    pub fn irc48mstbie(&mut self) -> Irc48mstbieW<'_, IntSpec> {
        Irc48mstbieW::new(self, 10)
    }
    #[doc = "Bit 11 - HXTAL stabilization interrupt enable"]
    #[inline(always)]
    pub fn hxtalstbie(&mut self) -> HxtalstbieW<'_, IntSpec> {
        HxtalstbieW::new(self, 11)
    }
    #[doc = "Bit 16 - IRC32K stabilization interrupt clear"]
    #[inline(always)]
    pub fn irc32kstbic(&mut self) -> Irc32kstbicW<'_, IntSpec> {
        Irc32kstbicW::new(self, 16)
    }
    #[doc = "Bit 17 - LXTAL stabilization interrupt clear"]
    #[inline(always)]
    pub fn lxtalstbic(&mut self) -> LxtalstbicW<'_, IntSpec> {
        LxtalstbicW::new(self, 17)
    }
    #[doc = "Bit 18 - IRC48M stabilization interrupt clear"]
    #[inline(always)]
    pub fn irc48mstbic(&mut self) -> Irc48mstbicW<'_, IntSpec> {
        Irc48mstbicW::new(self, 18)
    }
    #[doc = "Bit 19 - HXTAL stabilization interrupt clear"]
    #[inline(always)]
    pub fn hxtalstbic(&mut self) -> HxtalstbicW<'_, IntSpec> {
        HxtalstbicW::new(self, 19)
    }
    #[doc = "Bit 22 - LXTAL clock stuck interrupt clear"]
    #[inline(always)]
    pub fn lckmic(&mut self) -> LckmicW<'_, IntSpec> {
        LckmicW::new(self, 22)
    }
    #[doc = "Bit 23 - HXTAL clock stuck interrupt clear"]
    #[inline(always)]
    pub fn ckmic(&mut self) -> CkmicW<'_, IntSpec> {
        CkmicW::new(self, 23)
    }
}
#[doc = "Interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntSpec;
impl crate::RegisterSpec for IntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int::R`](R) reader structure"]
impl crate::Readable for IntSpec {}
#[doc = "`write(|w| ..)` method takes [`int::W`](W) writer structure"]
impl crate::Writable for IntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT to value 0"]
impl crate::Resettable for IntSpec {}
