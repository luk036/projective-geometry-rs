/// Dot-product of two vectors
///
/// The `dot` function calculates the dot product of two vectors.
///
/// Arguments:
///
/// * `v_a`: a is a slice of i32 values, representing the first vector.
/// * `v_b`: The parameter `v_b` is a slice of `i32` values, representing a vector.
///
/// Returns:
///
/// The dot product of the two vectors, which is an `i32` value.
///
/// Examples:
///
/// ```
/// use projective_geometry_rs::pg_common::dot;
/// let v_a = [1, 2, 3];
/// let v_b = [4, 5, 6];
/// assert_eq!(dot(&v_a, &v_b), 32);
/// ```
pub const fn dot(v_a: &[i32], v_b: &[i32]) -> i32 {
    v_a[0] * v_b[0] + v_a[1] * v_b[1] + v_a[2] * v_b[2]
}

/// The `cross` function calculates the cross-product of two vectors.
///
/// Arguments:
///
/// * `v_a`: The parameter `v_a` is an array slice of type `&[i32]`, which represents a reference to a slice
/// of `i32` values. It is used to represent the first vector in the cross product calculation.
/// * `v_b`: The parameter `v_b` represents the second vector in the cross product calculation.
///
/// Returns:
///
/// The function `cross` returns an array of three elements, which represents the cross product of two
/// vectors.
/// Cross-product of two vectors
///
/// Examples:
///
/// ```
/// use projective_geometry_rs::pg_common::cross;
/// let v_a = [1, 2, 3];
/// let v_b = [4, 5, 6];
/// assert_eq!(cross(&v_a, &v_b), [-3, 6, -3]);
/// ```
pub const fn cross(v_a: &[i32], v_b: &[i32]) -> [i32; 3] {
    [
        v_a[1] * v_b[2] - v_a[2] * v_b[1],
        v_a[2] * v_b[0] - v_a[0] * v_b[2],
        v_a[0] * v_b[1] - v_a[1] * v_b[0],
    ]
}

/// The function `plucker` takes two arrays `v_a` and `v_b`, and two scalar values `lambda` and `mu`, and
/// returns a new array where each element is the result of multiplying the corresponding elements of
/// `v_a` and `v_b` by `lambda` and `mu` respectively, and then summing them.
///
/// Arguments:
///
/// * `v_a`: a is a slice of i32 values. It represents the first vector in the plucker function.
/// * `lambda`: The `lambda` parameter is a scalar value that is multiplied with each element of the `v_a`
/// array.
/// * `v_b`: `v_b` is a slice of `i32` values.
/// * `mu`: The parameter `mu` represents the scalar multiplier for the elements in the array `v_b`. It is
/// used to scale the elements of `v_b` before adding them to the corresponding elements of `v_a` in the
/// result array.
///
/// Returns:
///
/// The function `plucker` returns an array of 3 integers.
///
/// Examples:
///
/// ```
/// use projective_geometry_rs::pg_common::plucker;
/// let v_a = [1, 2, 3];
/// let v_b = [4, 5, 6];
/// assert_eq!(plucker(&v_a, 2, &v_b, 3), [14, 19, 24]);
/// ```
pub const fn plucker(v_a: &[i32], lambda: i32, v_b: &[i32], mu: i32) -> [i32; 3] {
    [
        lambda * v_a[0] + mu * v_b[0],
        lambda * v_a[1] + mu * v_b[1],
        lambda * v_a[2] + mu * v_b[2],
    ]
}
