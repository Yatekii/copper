#![allow(dead_code)]

use ncollide2d::bounding_volume;
use ncollide2d::math::{ Vector, Point };
use nalgebra;


pub type Vector2D = Vector<f32>;
pub type Vector3 = nalgebra::base::Vector3<f32>;
pub type Point2D = Point<f32>;
pub type Vector4 = nalgebra::base::Vector4<f32>;
pub type Point4 = nalgebra::geometry::Point4<f32>;
pub type AABB = bounding_volume::AABB<f32>;
pub type Matrix3 = nalgebra::base::Matrix3<f32>;
pub type Matrix4 = nalgebra::base::Matrix4<f32>;