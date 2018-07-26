use nalgebra::geometry as ngeometry;
use nalgebra::base as nbase;
use geometry::{
    Point2,
    Vector2,
    Point4,
    Vector4,
    Matrix4
};

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

pub fn correct_cursor_coordinates(point: &Point2, width: f32, height: f32, scale_factor: i32) -> Point2 {
    let mut c = point.clone();
    c.x =  (c.x / width as f32 * 2.0 * (scale_factor as f32)) - 1.0;
    c.y = -(c.y / height as f32 * 2.0 * (scale_factor as f32)) + 1.0;
    c
}

pub fn transform_point_2d(point: &Point2, perspective: &Matrix4) -> Point2 {
    vector_to_point_2d(
        &vector_from_4d_to_2d(
            &(perspective
            * &point_to_vector_4d(
                &point_from_2d_to_4d(&point)
            ))
        )
    )
}

pub fn transform_point_4d(point: &Point4, perspective: &Matrix4) -> Point4 {
    vector_to_point_4d(
        &(perspective
        * &point_to_vector_4d(&point))
    )
}

pub fn transform_vector_2d(point: &Vector2, perspective: &Matrix4) -> Vector2 {
    vector_from_4d_to_2d(
        &(perspective
        * &vector_from_2d_to_4d(&point))
    )
}

pub fn transform_vector_4d(point: &Vector4, perspective: &Matrix4) -> Vector4 {
        perspective
        * point
}