/// Dot-product of two vectors
///
/// The `dot` function calculates the dot product of two vectors.
///
/// Arguments:
///
/// * `a`: a is a slice of i32 values, representing the first vector.
/// * `b`: The parameter `b` is a slice of `i32` values, representing a vector.
///
/// Returns:
///
/// The dot product of the two vectors, which is an `i32` value.
///
/// Examples:
///
/// ```
/// use projective_geometry_rs::pg_common::dot;
/// let a = [1, 2, 3];
/// let b = [4, 5, 6];
/// assert_eq!(dot(&a, &b), 32);
/// ```
pub const fn dot(a: &[i32], b: &[i32]) -> i32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// The `cross` function calculates the cross-product of two vectors.
///
/// Arguments:
///
/// * `a`: The parameter `a` is an array slice of type `&[i32]`, which represents a reference to a slice
/// of `i32` values. It is used to represent the first vector in the cross product calculation.
/// * `b`: The parameter `b` represents the second vector in the cross product calculation.
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
/// let a = [1, 2, 3];
/// let b = [4, 5, 6];
/// assert_eq!(cross(&a, &b), [-3, 6, -3]);
/// ```
pub const fn cross(a: &[i32], b: &[i32]) -> [i32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// The function `plucker` takes two arrays `a` and `b`, and two scalar values `lambda` and `mu`, and
/// returns a new array where each element is the result of multiplying the corresponding elements of
/// `a` and `b` by `lambda` and `mu` respectively, and then summing them.
///
/// Arguments:
///
/// * `a`: a is a slice of i32 values. It represents the first vector in the plucker function.
/// * `lambda`: The `lambda` parameter is a scalar value that is multiplied with each element of the `a`
/// array.
/// * `b`: `b` is a slice of `i32` values.
/// * `mu`: The parameter `mu` represents the scalar multiplier for the elements in the array `b`. It is
/// used to scale the elements of `b` before adding them to the corresponding elements of `a` in the
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
/// let a = [1, 2, 3];
/// let b = [4, 5, 6];
/// assert_eq!(plucker(&a, 2, &b, 3), [14, 19, 24]);
/// ```
pub const fn plucker(a: &[i32], lambda: i32, b: &[i32], mu: i32) -> [i32; 3] {
    [
        lambda * a[0] + mu * b[0],
        lambda * a[1] + mu * b[1],
        lambda * a[2] + mu * b[2],
    ]
}
