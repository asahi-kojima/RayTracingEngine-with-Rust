use super::vector as vector;

struct AABB
{
    v_min : vector::Vec3,
    v_max : vector::Vec3,
}

pub struct Object
{
    name : str,
}