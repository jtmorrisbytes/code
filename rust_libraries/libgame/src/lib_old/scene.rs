use super::camera::Camera;
use super::actor::Actor;
// a structure that contains all of the objects or entites in the 'world'
pub struct Scene {
    last_created_actor_id: u32,
    actors: std::collections::BTreeMap<u32,Box<dyn Actor>>,
    camera: Camera,
}
impl Scene {
    pub const fn origin() -> glam::Vec3 {
        glam::Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn new(camera:Camera) -> Self {
        Self {
            last_created_actor_id: 0,
            actors: Default::default(),
            camera
        }
    }
    pub fn insert_actor(&mut self, actor:Box<dyn Actor>) {
        self.actors.insert(self.last_created_actor_id + 1, actor);
        self.last_created_actor_id = self.last_created_actor_id + 1;
    }
    pub fn actors_iter_mut(&mut self) -> std::collections::btree_map::IterMut<'_, u32, Box<dyn super::actor::Actor>> {
        self.actors.iter_mut()
    }
    pub fn actors_iter(&self) -> std::collections::btree_map::Iter<'_, u32, Box<dyn super::actor::Actor>> {
        self.actors.iter()
    }
    pub fn camera<'b,'a:'b>(&'a self) -> &'b Camera {
        &self.camera
    }
    pub fn camera_mut<'b,'a:'b>(&'a mut self) -> &'b mut Camera {
        &mut self.camera
    }
}