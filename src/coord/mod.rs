/*!
  The abstraction of the coordinate system
*/
use crate::drawing::backend::BackendCoord;

mod datetime;
mod numeric;
mod ranged;

pub use datetime::{RangedDate, RangedDateTime};
pub use numeric::{
    RangedCoordf32, RangedCoordf64, RangedCoordi32, RangedCoordi64, RangedCoordu32, RangedCoordu64,
};
pub use ranged::{AsRangedCoord, DescreteRanged, MeshLine, Ranged, RangedCoord, ReversableRanged};

/// The trait that translates some customized object to the backend coordinate
pub trait CoordTranslate {
    type From;

    /// Translate the guest coordinate to the guest coordinate
    fn translate(&self, from: &Self::From) -> BackendCoord;
}

/// The trait indicates that the coordinate system supports reverse transform
/// This is useful when we need an interactive plot, thus we need to map the event
/// from the backend coordinate to the logical coordinate
pub trait ReverseCoordTranslate: CoordTranslate {
    /// Reverse translate the coordinate from the drawing coordinate to the
    /// logic coordinate.
    /// Note: the return value is an option, because it's possible that the drawing
    /// coordinate isn't able to be represented in te guest cooredinate system
    fn reverse_translate(&self, input: BackendCoord) -> Option<Self::From>;
}

/// The coordinate translation that only impose shift
#[derive(Debug, Clone)]
pub struct Shift(pub BackendCoord);

impl CoordTranslate for Shift {
    type From = BackendCoord;
    fn translate(&self, from: &Self::From) -> BackendCoord {
        return (from.0 + (self.0).0, from.1 + (self.0).1);
    }
}

impl ReverseCoordTranslate for Shift {
    fn reverse_translate(&self, input: BackendCoord) -> Option<BackendCoord> {
        return Some((input.0 - (self.0).0, input.1 - (self.0).1));
    }
}

/// We can compose an abitray transformation with a shift
pub struct ShiftAndTrans<T: CoordTranslate>(Shift, T);

impl<T: CoordTranslate> CoordTranslate for ShiftAndTrans<T> {
    type From = T::From;
    fn translate(&self, from: &Self::From) -> BackendCoord {
        let temp = self.1.translate(from);
        return self.0.translate(&temp);
    }
}

impl<T: ReverseCoordTranslate> ReverseCoordTranslate for ShiftAndTrans<T> {
    fn reverse_translate(&self, input: BackendCoord) -> Option<T::From> {
        return Some(self.1.reverse_translate(self.0.reverse_translate(input)?)?);
    }
}
