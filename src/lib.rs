use geo::point;
use geo::prelude::*;
use pyo3::prelude::*;
use rayon::prelude::*;

#[pyfunction]
pub fn geodesic(
    latitude_a: f64,
    longitude_a: f64,
    latitude_b: f64,
    longitude_b: f64,
) -> PyResult<f64> {
    let point_a = point!(x: longitude_a, y: latitude_a);
    let point_b = point!(x: longitude_b, y: latitude_b);

    let distance: f64 = point_a.geodesic_distance(&point_b);

    Ok(distance)
}

#[pyfunction]
pub fn batch_geodesic(
    latitude: f64,
    longitude: f64,
    points_of_interest: Vec<(f64, f64)>,
) -> PyResult<Vec<f64>> {
    let p1 = point!(x: longitude, y: latitude);

    let distances: Vec<f64> = points_of_interest
        .into_par_iter()
        .map(|point| {
            let tmp_point = point!(x: point.1, y: point.0);

            return p1.geodesic_distance(&tmp_point);
        })
        .collect();

    Ok(distances)
}

/// A Python module implemented in Rust.
#[pymodule]
fn fast_geo_distance(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(geodesic, m)?)?;
    m.add_function(wrap_pyfunction!(batch_geodesic, m)?)?;
    Ok(())
}
