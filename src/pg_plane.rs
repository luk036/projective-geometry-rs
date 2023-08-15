/// The code is defining a trait called `ProjectivePlanePrimitive` for a projection plane. This trait
/// has two associated functions:
/// 1. `incident` checks if a line is incident to a point.
/// 2. `meet` creates a line that joins two points.
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
