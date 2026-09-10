use crate::document::geometry::Point;

#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub center: Point,
    pub zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            center: Point::default(),
            zoom: 1.0,
        }
    }
}

impl Camera {
    pub fn document_to_screen(self, point: Point) -> Point {
        Point {
            x: (point.x - self.center.x) * self.zoom,
            y: (point.y - self.center.y) * self.zoom,
        }
    }

    pub fn screen_to_document(self, point: Point) -> Point {
        Point {
            x: point.x / self.zoom + self.center.x,
            y: point.y / self.zoom + self.center.y,
        }
    }
}
