mod polygon;
pub use polygon::ConvexPolygon;

mod rectangle;
pub use rectangle::Rectangle;

mod shape;
pub(crate) use shape::LazyShape;
pub use shape::Shape;

mod triangle;
pub use triangle::Triangle;

mod transformer;
pub use transformer::LazyTransformer;
