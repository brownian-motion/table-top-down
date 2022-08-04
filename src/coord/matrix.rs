use std::convert::TryFrom;

use ndarray::{Array1, LinalgScalar};
use num_traits::identities::One;

use crate::coord::basic_coords::{HomogeneousCoordinate, PlanarCoordinate};
use crate::coord::traits::Transformation;

pub type InvalidMatrixDimensionError = String;

pub struct MatrixTransform<T, const IDim: usize, const ODim: usize> {
    pub transform_matrix: ndarray::Array2<T>,
}

impl<T, const IDim: usize, const ODim: usize> TryFrom<ndarray::Array2<T>> for MatrixTransform<T, IDim, ODim> {
    type Error = InvalidMatrixDimensionError;

    fn try_from(transform_matrix: ndarray::Array2<T>) -> Result<Self, Self::Error> {
        let shape = transform_matrix.shape();
        if shape != &[IDim, ODim] {
            return Err(format!("expected transform with shape {}x{} but received {}x{}", IDim, ODim, shape[0], shape[1]));
        }

        return Ok(MatrixTransform { transform_matrix });
    }
}

impl<T, const IDim: usize, const ODim: usize> Transformation<Array1<T>, Array1<T>> for MatrixTransform<T, IDim, ODim> where T: LinalgScalar {
    fn transform(&self, in_coord: Array1<T>) -> Array1<T> {
        let mut result = Array1::zeros(ODim);
        ndarray::linalg::general_mat_vec_mul(One::one(), &self.transform_matrix, &in_coord, One::one(), &mut result);

        result
    }
}

impl<T, const IDim: usize, const ODim: usize> Transformation<HomogeneousCoordinate<T>, HomogeneousCoordinate<T>> for MatrixTransform<T, IDim, ODim> where T: LinalgScalar {
    fn transform(&self, in_coord: HomogeneousCoordinate<T>) -> HomogeneousCoordinate<T> {
        let in_array: Array1<T> = in_coord.into();
        let out_array: Array1<T> = self.transform(in_array);
        return HomogeneousCoordinate::try_from(out_array).unwrap();
    }
}

impl<T, const IDim: usize, const ODim: usize> Transformation<PlanarCoordinate<T>, PlanarCoordinate<T>> for MatrixTransform<T, IDim, ODim> where T: LinalgScalar {
    fn transform(&self, in_coord: PlanarCoordinate<T>) -> PlanarCoordinate<T> {
        let in_homo: HomogeneousCoordinate<T> = HomogeneousCoordinate::from(in_coord);
        let out_homo: HomogeneousCoordinate<T> = self.transform(in_homo);
        return PlanarCoordinate::from(out_homo);
    }
}