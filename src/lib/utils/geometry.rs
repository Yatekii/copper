use nalgebra::geometry as ngeometry;
use nalgebra::base as nbase;

pub fn vector_to_point_2d(v: &nbase::Vector2<f32>) -> ngeometry::Point2<f32> {
    ngeometry::Point2::<f32>::new(v.x, v.y)
}
pub fn vector_to_point_4d(v: &nbase::Vector4<f32>) -> ngeometry::Point4<f32> {
    ngeometry::Point4::<f32>::new(v.x, v.y, v.z, v.w)
}
pub fn point_to_vector_2d(v: &ngeometry::Point2<f32>) -> nbase::Vector2<f32> {
    nbase::Vector2::<f32>::new(v.x, v.y)
}
pub fn point_to_vector_4d(v: &ngeometry::Point4<f32>) -> nbase::Vector4<f32> {
    nbase::Vector4::<f32>::new(v.x, v.y, v.z, v.w)
}
pub fn point_from_2d_to_4d(v: &ngeometry::Point2<f32>) -> ngeometry::Point4<f32> {
    ngeometry::Point4::<f32>::new(v.x, v.y, 0.0, 1.0)
}
pub fn point_from_4d_to_2d(v: &ngeometry::Point4<f32>) -> ngeometry::Point2<f32> {
    ngeometry::Point2::<f32>::new(v.x, v.y)
}
pub fn vector_from_2d_to_4d(v: &nbase::Vector2<f32>) -> nbase::Vector4<f32> {
    nbase::Vector4::<f32>::new(v.x, v.y, 0.0, 1.0)
}
pub fn vector_from_4d_to_2d(v: &nbase::Vector4<f32>) -> nbase::Vector2<f32> {
    nbase::Vector2::<f32>::new(v.x, v.y)
}