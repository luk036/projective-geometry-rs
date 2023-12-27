/// `ProjectivePlanePrimitive` is a trait that defines the basic operations needed
/// for points and lines to interact in a projective plane. It requires the
/// implementing type to be `Eq` and provides two functions:
///
/// - `incident` - Checks if a line is incident to (passes through) a point.
/// - `meet` - Joins two points by creating a line between them.
pub trait ProjectivePlanePrimitive<Line>: Eq {
    fn incident(&self, dual: &Line) -> bool;
    fn meet(&self, other: &Self) -> Line; // join or meet
}

/// The function checks if two points lie on the same line in a projective plane.
///
/// Arguments:
///
/// * `pt_p`: The parameter `pt_p` represents a point on a projective plane.
/// * `pt_q`: The parameter `pt_q` represents a point in a projective plane.
///
/// Returns:
///
/// a boolean value.
pub fn check_projection_plane_primitive<Point, Line>(pt_p: &Point, pt_q: &Point) -> bool
where
    Point: ProjectivePlanePrimitive<Line>,
    Line: ProjectivePlanePrimitive<Point>,
{
    let ln_l = pt_p.meet(pt_q);
    let ln_m = pt_q.meet(pt_p);
    if ln_l != ln_m {
        return false;
    }
    ln_l.incident(pt_p) && ln_l.incident(pt_q)
}

/// The `coincident` function checks if three points `pt_p`, `pt_q`, and `pt_r` are collinear in a projective
/// plane.
///
/// Arguments:
///
/// * `pt_p`: A point in the projective plane.
/// * `pt_q`: The parameter `pt_q` is of type `&Point`, which means it is a reference to an object of type
/// `Point`.
/// * `pt_r`: The parameter `pt_r` represents a point in the projective plane.
///
/// Returns:
///
/// The function `coincident` returns a boolean value.
pub fn coincident<Point, Line>(pt_p: &Point, pt_q: &Point, pt_r: &Point) -> bool
where
    Point: ProjectivePlanePrimitive<Line>,
    Line: ProjectivePlanePrimitive<Point>,
{
    pt_p.meet(pt_q).incident(pt_r)
}

// test_pg_plane.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_projection_plane_primitive() {
        // Arrange
        #[derive(PartialEq, Eq)]
        struct Point;

        #[derive(PartialEq, Eq)]
        struct Line;

        impl ProjectivePlanePrimitive<Line> for Point {
            fn incident(&self, _dual: &Line) -> bool {
                true
            }

            fn meet(&self, _other: &Self) -> Line {
                Line {}
            }
        }

        impl ProjectivePlanePrimitive<Point> for Line {
            fn incident(&self, _dual: &Point) -> bool {
                true
            }

            fn meet(&self, _other: &Self) -> Point {
                Point {}
            }
        }

        let pt_p = Point;
        let pt_q = Point;

        // Act
        let result = check_projection_plane_primitive(&pt_p, &pt_q);

        // Assert
        assert!(result);
    }

    #[test]
    fn test_check_projection_plane_primitive_false() {
        // Arrange
        #[derive(PartialEq, Eq)]
        struct Point;

        #[derive(PartialEq, Eq)]
        struct Line;

        impl ProjectivePlanePrimitive<Line> for Point {
            fn incident(&self, _dual: &Line) -> bool {
                true
            }

            fn meet(&self, _other: &Self) -> Line {
                Line {}
            }
        }

        impl ProjectivePlanePrimitive<Point> for Line {
            fn incident(&self, _dual: &Point) -> bool {
                true
            }

            fn meet(&self, _other: &Self) -> Point {
                Point {}
            }
        }

        let pt_p = Point;
        let pt_q = Point;

        // Act
        let result = check_projection_plane_primitive(&pt_p, &pt_q);

        // Assert
        assert!(result);
    }

    #[test]
    fn test_coincident() {
        // Arrange
        #[derive(PartialEq, Eq)]
        struct Point;

        #[derive(PartialEq, Eq)]
        struct Line;

        impl ProjectivePlanePrimitive<Line> for Point {
            fn incident(&self, _dual: &Line) -> bool {
                true
            }

            fn meet(&self, _other: &Self) -> Line {
                Line {}
            }
        }

        impl ProjectivePlanePrimitive<Point> for Line {
            fn incident(&self, _dual: &Point) -> bool {
                true
            }

            fn meet(&self, _other: &Self) -> Point {
                Point {}
            }
        }

        let pt_p = Point;
        let pt_q = Point;
        let pt_r = Point;

        // Act
        let result = coincident(&pt_p, &pt_q, &pt_r);

        // Assert
        assert!(result);
    }

    #[test]
    fn test_coincident_false() {
        // Arrange
        #[derive(PartialEq, Eq)]
        struct Point;

        #[derive(PartialEq, Eq)]
        struct Line;

        impl ProjectivePlanePrimitive<Line> for Point {
            fn incident(&self, _dual: &Line) -> bool {
                true
            }

            fn meet(&self, _other: &Self) -> Line {
                Line {}
            }
        }

        impl ProjectivePlanePrimitive<Point> for Line {
            fn incident(&self, _dual: &Point) -> bool {
                true
            }

            fn meet(&self, _other: &Self) -> Point {
                Point {}
            }
        }

        let pt_p = Point;
        let pt_q = Point;
        let pt_r = Point;

        // Act
        let result = coincident(&pt_p, &pt_q, &pt_r);

        // Assert
        assert!(result);
    }
}



