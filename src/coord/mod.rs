use std::clone::Clone;
use std::convert::TryFrom;
use std::default::Default;
use std::fmt::Display;
use std::ops::Div;

use image::Pixel;
use ndarray::Array1;

use basic_coords::*;
use matrix::*;
use traits::*;

pub mod basic_coords;
pub mod matrix;
pub mod traits;

pub type PixelCoordinate = PlanarCoordinate<f64>;
pub type RealWorldCoordinate2D = PlanarCoordinate<f64>;
pub type RectGridCoordinate = PlanarCoordinate<i32>;

// TODO: figure out how to not have to specify the Input/Ouptut bounds after the transformation types, since those should be implicit
pub type ScreenToRealWorldTransform = BidirectionalTransform<MatrixTransform<f64, 3, 3>, MatrixTransform<f64, 3, 3>, PlanarCoordinate<f64>, PlanarCoordinate<f64>>;

impl ScreenToRealWorldTransform {
    pub fn from_matrices(transformation_to_pixel: ndarray::Array2<f64>, transformation_to_real: ndarray::Array2<f64>) -> Result<Self, InvalidMatrixDimensionError> {
        let real_to_pixel = MatrixTransform::try_from(transformation_to_pixel)?;
        let pixel_to_real = MatrixTransform::try_from(transformation_to_real)?;

        Ok(BidirectionalTransform::new(real_to_pixel, pixel_to_real))
    }

    // pub fn from_transform_and_offset(linear_transform: ndarray::Array2<f64>, offset: ndarray::Array1<f64>) -> Result<Self, InvalidTransformError> {
    //     unimplemented!()
    // }

    pub fn from_pixels(unit_square: [PixelCoordinate; 4]) -> Result<Self, &'static str> {
        // determines the transformation that would project the unit square with coordinates [(0,0), (1,0), (1,1), (0,1)] to the given point.
        // see https://www-users.cse.umn.edu/~hspark/CSci5980/Lec2_ProjectionMatrix.pdf
        unimplemented!()
    }

    pub fn pixel_to_real(&self, coord: PixelCoordinate) -> RealWorldCoordinate2D {
        self.forward.transform(coord as PlanarCoordinate<f64>) as RealWorldCoordinate2D
    }

    pub fn real_to_pixel(&self, coord: RealWorldCoordinate2D) -> PixelCoordinate {
        // https://stackoverflow.com/questions/631039/how-can-i-find-the-3d-coordinates-of-a-projected-rectangle

        // note that this should transform into the target plane with z=1.0 after normalizing: λ[u, v, 1] = P_rp x

        self.forward.transform(coord as PlanarCoordinate<f64>) as PixelCoordinate
    }
}

#[cfg(test)]
mod test {
    use ndarray::{array, Array2};

    use crate::coord::{PixelCoordinate, RealWorldCoordinate2D, ScreenToRealWorldTransform};
    use crate::coord::basic_coords::PlanarCoordinate;

    fn is_approximately_eq(f1: f64, f2: f64, threshold: f64) -> bool {
        return (f1 - f2).abs() < threshold;
    }

    fn assert_float_approximately_eq(f1: f64, f2: f64, threshold: f64) {
        assert!(is_approximately_eq(f1, f2, threshold), "floats {f1} and {f2} are not within {threshold} of each other")
    }

    fn assert_coord_approximately_eq(c1: PlanarCoordinate<f64>, c2: PlanarCoordinate<f64>, threshold: f64) {
        assert!(is_approximately_eq(c1.x, c2.x, threshold), "x-coordinates of {c1:?} and {c2:?} are not within {threshold} of each other");
        assert!(is_approximately_eq(c1.y, c2.y, threshold), "y-coordinates of {c1:?} and {c2:?} are not within {threshold} of each other");
    }

    // #[test]
    // pub fn unitary_transform_projects_pixels() {
    //     let unitary_transform = ScreenToRealWorldTransform::from_transform(Array2::zeros((3,4)), array![[1.0,0.0,0.0,0.0],[0.0,1.0,0.0,0.0],[0.0,0.0,1.0,0.0]]).unwrap();
    //     let input_coord = PixelCoordinate { x: 5.0, y: -3.0 };
    //     let expected_output_coord = RealWorldCoordinate2D { x: 5.0, y: -3.0 };
    //     let output_coord = unitary_transform.pixel_to_real(input_coord);
    //
    //     assert_approximately_eq(output_coord.x, expected_output_coord.x, 0.01);
    //     assert_approximately_eq(output_coord.y, expected_output_coord.y, 0.01);
    // }
    //
    // #[test]
    // pub fn unitary_transform_projects_real_points() {
    //     let unitary_transform = ScreenToRealWorldTransform::from_transform(array![[1.0,0.0,0.0,0.0],[0.0,1.0,0.0,0.0],[0.0,0.0,1.0,0.0]], Array2::zeros((3,4))).unwrap();
    //     let input_coord = RealWorldCoordinate2D { x: 5.0, y: -3.0 };
    //     let expected_output_coord = PixelCoordinate { x: 5.0, y: -3.0 };
    //     let output_coord = unitary_transform.real_to_pixel(input_coord);
    //
    //     assert_approximately_eq(output_coord.x, expected_output_coord.x, 0.01);
    //     assert_approximately_eq(output_coord.y, expected_output_coord.y, 0.01);
    // }

    // TODO: figure out if these numbers are legit!
    #[test]
    pub fn practice_real_to_pixel() {
        // from https://www.youtube.com/watch?v=fVJeJMWZcq8

        let transform_real_to_pixel = array![
            [1.4003, 0.3827, -136.5900],
            [-0.0785, 1.8049, -83.1054],
            [-0.0003, 0.0016, 1.0000],
        ];
        let transformation = ScreenToRealWorldTransform::from_matrices(transform_real_to_pixel, Array2::zeros((3, 3))).unwrap();

        let real_coord = RealWorldCoordinate2D{x: 600.0, y: 100.0 };
        let pixel_coord = transformation.real_to_pixel(real_coord);

        assert_coord_approximately_eq(PixelCoordinate { x: 757.000, y: 51.311 }, pixel_coord, 0.001);
    }
}