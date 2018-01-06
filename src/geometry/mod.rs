use euclid;


pub struct ScreenSpace;
pub struct SchemaSpace;

pub type TSchemaScreen = euclid::TypedTransform3D<f32, SchemaSpace, ScreenSpace>;
pub type TSchemaSchema = euclid::TypedTransform3D<f32, SchemaSpace, SchemaSpace>;

pub type SchemaPoint2D = euclid::TypedPoint2D::<f32, SchemaSpace>;
pub type SchemaPoint3D = euclid::TypedPoint3D::<f32, SchemaSpace>;

pub type ScreenPoint2D = euclid::TypedPoint2D::<f32, ScreenSpace>;
pub type ScreenPoint3D = euclid::TypedPoint3D::<f32, ScreenSpace>;

pub type SchemaVector2D = euclid::TypedVector2D::<f32, SchemaSpace>;
pub type SchemaVector3D = euclid::TypedVector3D::<f32, SchemaSpace>;

pub type ScreenVector2D = euclid::TypedVector2D::<f32, ScreenSpace>;
pub type ScreenVector3D = euclid::TypedVector3D::<f32, ScreenSpace>;

pub type ScreenRect = euclid::TypedRect::<f32, ScreenSpace>;
pub type SchemaRect = euclid::TypedRect::<f32, SchemaSpace>;