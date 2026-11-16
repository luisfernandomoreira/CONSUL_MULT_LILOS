#[doc = "Register `INTF` reader"]
pub type R = crate::R<IntfSpec>;
#[doc = "Field `GIF0` reader - Channel 0 Global interrupt flag"]
pub type Gif0R = crate::BitReader;
#[doc = "Field `FTFIF0` reader - Channel 0 Full Transfer Finish flag"]
pub type Ftfif0R = crate::BitReader;
#[doc = "Field `HTFIF0` reader - Channel 0 Half Transfer Finish flag"]
pub type Htfif0R = crate::BitReader;
#[doc = "Field `ERRIF0` reader - Channel 0 Error flag"]
pub type Errif0R = crate::BitReader;
#[doc = "Field `GIF1` reader - Channel 1 Global interrupt flag"]
pub type Gif1R = crate::BitReader;
#[doc = "Field `FTFIF1` reader - Channel 1 Full Transfer Finish flag"]
pub type Ftfif1R = crate::BitReader;
#[doc = "Field `HTFIF1` reader - Channel 1 Half Transfer Finish flag"]
pub type Htfif1R = crate::BitReader;
#[doc = "Field `ERRIF1` reader - Channel 1 Error flag"]
pub type Errif1R = crate::BitReader;
#[doc = "Field `GIF2` reader - Channel 2 Global interrupt flag"]
pub type Gif2R = crate::BitReader;
#[doc = "Field `FTFIF2` reader - Channel 2 Full Transfer Finish flag"]
pub type Ftfif2R = crate::BitReader;
#[doc = "Field `HTFIF2` reader - Channel 2 Half Transfer Finish flag"]
pub type Htfif2R = crate::BitReader;
#[doc = "Field `ERRIF2` reader - Channel 2 Error flag"]
pub type Errif2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 0 Global interrupt flag"]
    #[inline(always)]
    pub fn gif0(&self) -> Gif0R {
        Gif0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 Full Transfer Finish flag"]
    #[inline(always)]
    pub fn ftfif0(&self) -> Ftfif0R {
        Ftfif0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 0 Half Transfer Finish flag"]
    #[inline(always)]
    pub fn htfif0(&self) -> Htfif0R {
        Htfif0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 0 Error flag"]
    #[inline(always)]
    pub fn errif0(&self) -> Errif0R {
        Errif0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 1 Global interrupt flag"]
    #[inline(always)]
    pub fn gif1(&self) -> Gif1R {
        Gif1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 1 Full Transfer Finish flag"]
    #[inline(always)]
    pub fn ftfif1(&self) -> Ftfif1R {
        Ftfif1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 1 Half Transfer Finish flag"]
    #[inline(always)]
    pub fn htfif1(&self) -> Htfif1R {
        Htfif1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 1 Error flag"]
    #[inline(always)]
    pub fn errif1(&self) -> Errif1R {
        Errif1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 2 Global interrupt flag"]
    #[inline(always)]
    pub fn gif2(&self) -> Gif2R {
        Gif2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 2 Full Transfer Finish flag"]
    #[inline(always)]
    pub fn ftfif2(&self) -> Ftfif2R {
        Ftfif2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 Half Transfer Finish flag"]
    #[inline(always)]
    pub fn htfif2(&self) -> Htfif2R {
        Htfif2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 2 Error flag"]
    #[inline(always)]
    pub fn errif2(&self) -> Errif2R {
        Errif2R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "DMA interrupt flag register (DMA_INTF)\n\nYou can [`read`](crate::Reg::read) this register and get [`intf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfSpec;
impl crate::RegisterSpec for IntfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf::R`](R) reader structure"]
impl crate::Readable for IntfSpec {}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for IntfSpec {}
