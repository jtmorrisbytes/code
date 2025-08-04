pub struct Camera {
    /// where is our camera in world space?
    position: glam::Vec3,
    /// what is the camera looking at?
    target: glam::Vec3,
    /// what direction is up, relative to the camera
    up: glam::Vec3,
    /// what direction is right relative to the camera
    right: glam::Vec3,
    // the vector starting at the target pointing to the camera along its Z axis
    direction: glam::Vec3,
}
impl Camera {
    pub fn calculate_direction_vector(position: glam::Vec3, target: glam::Vec3) -> glam::Vec3 {
        (position - target).normalize()
    }
    pub fn calculate_right_vector(up: glam::Vec3, direction: glam::Vec3) -> glam::Vec3 {
        // the normalized cross product of the direction vector and the up vector
        up.cross(direction).normalize()
    }
    pub fn calculate_up_vector(direction: glam::Vec3, right: glam::Vec3) -> glam::Vec3 {
        direction.cross(right)
    }
    pub fn new(position: glam::Vec3, target: glam::Vec3, up: glam::Vec3) -> Self {
        let camera_direction = Self::calculate_direction_vector(position, target);
        let camera_right = Self::calculate_right_vector(up, camera_direction);
        let camera_up = Self::calculate_up_vector(camera_direction, camera_right);
        Self {
            position,
            target,
            up: camera_up,
            right: camera_right,
            direction: camera_direction,
        }
    }
    pub fn get_rotation_matrix(&self) -> glam::Mat4 {
        glam::mat4(
            glam::vec4(self.right.x, self.right.y, self.right.z, 0.0),
            glam::vec4(self.up.x, self.up.y, self.up.z, 0.0),
            glam::vec4(self.direction.x, self.direction.y, self.direction.z, 0.0),
            glam::vec4(0.0, 0.0, 0.0, 1.0),
        )
    }
    pub fn get_translation_matrix(&self) -> glam::Mat4 {
        glam::mat4(
            glam::vec4(1.0, 0.0, 0.0, -self.position.x),
            glam::vec4(0.0, 1.0, 0.0, -self.position.y),
            glam::vec4(0.0, 0.0, 1.0, self.position.z),
            glam::vec4(0.0, 0.0, 0.0, 1.0),
        )
    }
    pub fn look_at(&self) -> glam::Mat4 {
        glam::Mat4::look_at_rh(self.position, self.target, self.up)
    }
    pub fn position(&self) -> glam::Vec3 {
        self.position
    }
    pub fn set_position(&mut self, position: glam::Vec3) {
        self.position = position
    }
}
