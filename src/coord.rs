use image::Pixel;
use ndarray::Array1;

#[derive(Debug, Copy, Clone, Default)]
pub struct PixelCoordinate {
    pub x: f64,
    pub y: f64,
}

impl Into<Array1<f64>> for PixelCoordinate {
    fn into(self) -> Array1<f64> {
        ndarray::array!(self.x, self.y)
    }
}

impl From<Array1<f64>> for PixelCoordinate {
    fn from(arr: Array1<f64>) -> Self {
        let mut c = Self::default();
        if arr.len() >= 1 {
            c.x = arr[0];
        }
        if arr.len() >= 2 {
            c.y = arr[1]
        }

        c
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct RealWorldCoordinate2D {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct RectGridCoordinate {
    pub x: i32,
    pub y: i32,
}

// todo: figure out what kind of traits make sense for this, maybe Transform<C1, C2>
pub struct ScreenToRealWorldTransform {
    pub transformation_to_pixel: ndarray::Array2<f64>, // 3 tall, 4 wide; the shape must always be &[3,4]
    pub transformation_from_pixel: ndarray::Array2<f64>, // 3 tall, 4 wide; the shape must always be &[3,4]
}

pub type InvalidTransformError = String;

impl ScreenToRealWorldTransform {
    pub fn from_transform(transformation_to_pixel: ndarray::Array2<f64>, transformation_to_real: ndarray::Array2<f64>) -> Result<Self, InvalidTransformError> {
        let shape = transformation_to_pixel.shape();
        if shape != &[3, 4] {
            return Err(format!("expected transform with shape 3x4 but received {}x{}", shape[0], shape[1]));
        }

        let shape = transformation_to_real.shape();
        if shape != &[3, 4] {
            return Err(format!("expected transform with shape 3x4 but received {}x{}", shape[0], shape[1]));
        }

        return Ok(ScreenToRealWorldTransform { transformation_to_pixel, transformation_from_pixel: transformation_to_real });
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
        // https://stackoverflow.com/questions/631039/how-can-i-find-the-3d-coordinates-of-a-projected-rectangle
        let input_arr = ndarray::array![coord.x, coord.y, 0.0, 1.0];
        let mut result = ndarray::array![0.0, 0.0, 0.0];
        ndarray::linalg::general_mat_vec_mul(1.0, &self.transformation_from_pixel, &input_arr, 1.0, &mut result);

        // note that this should transform into the target plane with z=1.0 after normalizing: λ[u, v, 1] = P_pr x

        RealWorldCoordinate2D { x: result[0]/result[2], y: result[1]/result[2] }
    }

    pub fn real_to_pixel(&self, coord: RealWorldCoordinate2D) -> PixelCoordinate {
        // https://stackoverflow.com/questions/631039/how-can-i-find-the-3d-coordinates-of-a-projected-rectangle
        let input_arr = ndarray::array![coord.x, coord.y, 0.0, 1.0];
        let mut result = Array1::zeros(3);
        ndarray::linalg::general_mat_vec_mul(1.0, &self.transformation_to_pixel, &input_arr, 1.0, &mut result);

        // note that this should transform into the target plane with z=1.0 after normalizing: λ[u, v, 1] = P_rp x

        PixelCoordinate { x: result[0]/result[2], y: result[1]/result[2] }
    }
}

#[cfg(test)]
mod test {
    use ndarray::{array, Array2};
    use crate::coord::{PixelCoordinate, RealWorldCoordinate2D, ScreenToRealWorldTransform};

    fn is_approximately_eq(f1: f64, f2: f64, threshold: f64) -> bool {
        return (f1 - f2).abs() < threshold;
    }

    fn assert_approximately_eq(f1: f64, f2: f64, threshold: f64) {
        assert!(is_approximately_eq(f1, f2, threshold), "floats {f1} and {f2} are not within {threshold} of each other")
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
        //from https://www-users.cse.umn.edu/~hspark/CSci5980/Lec2_ProjectionMatrix.pdf

        let transform_real_to_pixel = ndarray::array![
            [-0.2374, -0.9565, 2.0138, 1.2723],
            [0.0578, -1.5763, 0.2404, 1.2508],
            [0.0004, -0.0005, 0.0007, 0.0006],
        ];
        let transformation = ScreenToRealWorldTransform::from_transform(transform_real_to_pixel, Array2::zeros((3,4))).unwrap();

        let x_vanishing_point = transformation.real_to_pixel(RealWorldCoordinate2D{x: 1e9 as f64, y: 0.0});
        // assert_eq!((-568, 138), (x_vanishing_point.x as i32, x_vanishing_point.y as i32));
        assert_eq!((-593, 144), (x_vanishing_point.x as i32, x_vanishing_point.y as i32));

        let y_vanishing_point = transformation.real_to_pixel(RealWorldCoordinate2D{x: 0.0, y: 1e9 as f64});
        assert_eq!((1912, 3152), (y_vanishing_point.x as i32, y_vanishing_point.y as i32));
        // assert_eq!((1805, 2975), (y_vanishing_point.x as i32, y_vanishing_point.y as i32));

        let origin = transformation.real_to_pixel(RealWorldCoordinate2D{x: 0.0, y: 0.0});
        assert_eq!((2120, 2084), (origin.x as i32, origin.y as i32));
        // assert_eq!((1805, 2975), (y_vanishing_point.x as i32, y_vanishing_point.y as i32));

    }
}