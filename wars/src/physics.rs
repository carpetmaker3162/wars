/// Describes the collision between two objects. Each member indicates whether
/// the two objects in question intersects with each other at that specific di-
/// rection. For example, if `left` is true, it means that the target objects  
/// is intersecting with the other object at it's left side.
pub struct Collision {
    left: bool,
    right: bool,
    top: bool,
    bottom: bool,
}

pub struct AABCollider {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
}

impl AABCollider {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> AABCollider {
        AABCollider {
            left: x,
            right: x + width,
            top: y,
            bottom: y + height,
        }
    }

    pub fn move_collider(&mut self, x: f64, y: f64) {
        self.left += x;
        self.right += x;

        self.top += y;
        self.bottom += y;
    }

    pub fn get_collision_with_aabc(&self, another_collider: &AABCollider) -> Collision {
        let mut result = Collision {
            left: false,
            right: false,
            top: false,
            bottom: false,
        };

        if (another_collider.left < self.left) && (self.left < another_collider.right) {
            result.left = true;
        }

        if (self.left < another_collider.left) && (another_collider.left < self.right) {
            result.right = true;
        }

        if (another_collider.top < self.top) && (self.top < another_collider.bottom) {
            result.top = true;
        }
        
        if (self.top < another_collider.top) && (another_collider.top < self.bottom) {
            result.bottom = true;
        }

        result
    }
}
