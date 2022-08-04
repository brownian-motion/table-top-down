use std::clone::Clone;
use std::convert::TryFrom;
use std::default::Default;
use std::ops::Div;

use ndarray::Array1;
use num_traits::identities::One;

#[derive(Debug, Copy, Clone, Default)]
pub struct PlanarCoordinate<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct HomogeneousCoordinate<T> {
    pub u: T,
    // essentially the scaled up x
    pub v: T,
    // essentially the scaled up y
    pub w: T, // basically represents the scale
}

impl<T> From<HomogeneousCoordinate<T>> for PlanarCoordinate<T> where T: Div<Output=T> + Clone {
    fn from(homogenous: HomogeneousCoordinate<T>) -> PlanarCoordinate<T> {
        return PlanarCoordinate { x: homogenous.u / homogenous.w.clone(), y: homogenous.v / homogenous.w };
    }
}

impl<T> From<PlanarCoordinate<T>> for HomogeneousCoordinate<T> where T: One {
    fn from(planar: PlanarCoordinate<T>) -> Self {
        HomogeneousCoordinate { u: planar.x, v: planar.y, w: One::one() }
    }
}

impl<T> From<PlanarCoordinate<T>> for Array1<T> {
    fn from(coord: PlanarCoordinate<T>) -> Array1<T> {
        ndarray::array!(coord.x, coord.y)
    }
}

impl<T> TryFrom<Array1<T>> for PlanarCoordinate<T> where T: Clone {
    type Error = String;

    fn try_from(arr: Array1<T>) -> Result<Self, Self::Error> {
        if arr.len() < 2 {
            return Err(format!("expected array of length 2 but was {}", arr.len()));
        }
        Ok(PlanarCoordinate { x: arr[0].clone(), y: arr[1].clone() })
    }
}

impl<T> From<HomogeneousCoordinate<T>> for Array1<T> {
    fn from(coord: HomogeneousCoordinate<T>) -> Array1<T> {
        ndarray::array!(coord.u, coord.v, coord.w)
    }
}

impl<T> TryFrom<Array1<T>> for HomogeneousCoordinate<T> where T: Clone {
    type Error = String;

    fn try_from(arr: Array1<T>) -> Result<Self, Self::Error> {
        if arr.len() < 3 {
            return Err(format!("expected array of length 3 but was actually {}", arr.len()));
        }
        Ok(HomogeneousCoordinate { u: arr[0].clone(), v: arr[1].clone(), w: arr[2].clone() })
    }
}

#[cfg(test)]
mod test {
    use crate::coord::basic_coords::{HomogeneousCoordinate, PlanarCoordinate};

    fn assert_approx_eq(c1: PlanarCoordinate<f64>, c2: PlanarCoordinate<f64>, threshold: f64) {
        assert!((c1.x - c2.x).abs() < threshold, "x-coordinates differed by more than {threshold:?} between {c1:?} and {c2:?}")
    }

    #[test]
    fn homog_convert_to_planar() {
        let in_homo = HomogeneousCoordinate { u: 12.0, v: 18.0, w: 2.0 };
        let expected_planar = PlanarCoordinate { x: 6.0, y: 9.0 };
        let actual_planar: PlanarCoordinate<f64> = in_homo.into();
        assert_approx_eq(expected_planar, actual_planar, 0.001);
    }
}