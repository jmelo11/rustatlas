use crate::{
    alm::traits::AdvanceInTime,
    rates::traits::{HasReferenceDate, YieldProvider},
};

pub trait ObjectSafeClone {
    fn clone_box(&self) -> Box<dyn YieldTermStructureTrait>;
}

impl<T: 'static + YieldTermStructureTrait + Clone> ObjectSafeClone for T {
    fn clone_box(&self) -> Box<dyn YieldTermStructureTrait> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn YieldTermStructureTrait> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub trait YieldTermStructureTrait:
    YieldProvider + HasReferenceDate + ObjectSafeClone + AdvanceTermStructureInTime
{
}
