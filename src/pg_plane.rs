/// The code is defining a trait called `ProjectivePlanePrimitive` for a projection plane. This trait
/// has two associated functions:
pub trait ProjectivePlanePrimitive<Dual> : Eq {
    fn incident(&self, dual: &Dual) -> bool;
    fn circ(&self, other: &Self) -> Dual;
}

pub fn check_projection_plane_primitive<Primal, Dual>(p: &Primal, q: &Primal) -> bool
where
    Primal: ProjectivePlanePrimitive<Dual>,
    Dual: ProjectivePlanePrimitive<Primal>,
{
    if p != p || q != q {
        return false;
    }
    let l = p.circ(q);
    let m = q.circ(p);
    if l != m {
        return false;
    }
    l.incident(p) && l.incident(q)
}

/// The `coincident` function checks if three points `p`, `q`, and `r` are collinear in a projective
/// plane.
/// 
/// Arguments:
/// 
/// * `p`: A point in the projective plane.
/// * `q`: The parameter `q` is of type `&Primal`, which means it is a reference to an object of type
/// `Primal`.
/// * `r`: The parameter `r` represents a point in the projective plane.
/// 
/// Returns:
/// 
/// The function `coincident` returns a boolean value.
pub fn coincident<Primal, Dual>(p: &Primal, q: &Primal, r: &Primal) -> bool
where
    Primal: ProjectivePlanePrimitive<Dual>,
    Dual: ProjectivePlanePrimitive<Primal>,
{
    p.circ(q).incident(r)    
}

