use geo::point;
use geo::prelude::*;
use pyo3::prelude::*;
use rayon::prelude::*;

#[pyfunction]
fn geodesic(latitude_a: f64, longitude_a: f64, latitude_b: f64, longitude_b: f64) -> PyResult<f64> {
    let point_a = point!(x: latitude_a, y: longitude_a);
    let point_b = point!(x: latitude_b, y: longitude_b);

    let distance: f64 = point_a.geodesic_distance(&point_b);

    Ok(distance)
}

#[pyfunction]
fn batch_geodesic(latitude: f64, longitude: f64, points_of_interest: Vec<(f64, f64)>) -> PyResult<Vec<f64>> {
    let p1 = point!(x: latitude, y: longitude);

    let distances: Vec<f64> = points_of_interest.into_par_iter().map(|point| {
        let tmp_point = point!(x: point.0, y: point.1);

        return  p1.geodesic_distance(&tmp_point);
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

#[cfg(test)]
mod tests {
    use crate::*;

    const LATITUDE_A: f64 = 11.572231488797142;
    const LONGITUDE_A: f64 = 48.14000452866368;
    const LATITUDE_B: f64 = 52.5170365;
    const LONGITUDE_B: f64 = 13.3888599;
    const DISTANCE: f64 = 5388768.0;

    #[test]
    fn test_geodesic() {
        let distance: f64 = geodesic(LATITUDE_A, LONGITUDE_A, LATITUDE_B, LONGITUDE_B).unwrap();
        
        assert_eq!(distance.round(), DISTANCE);
    }

    #[test]
    fn test_batch_geodesic() {
        let points_of_interest: Vec<(f64, f64)> = vec![(LATITUDE_B, LONGITUDE_B)];
        let distances: Vec<f64> = batch_geodesic(LATITUDE_A, LONGITUDE_A, points_of_interest).unwrap();
        
        assert_eq!(distances[0].round(), DISTANCE);
    }
}
