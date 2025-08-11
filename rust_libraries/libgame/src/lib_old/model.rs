

fn vec3_position_to_quat(vertex:glam::Vec3) -> glam::Quat{
    glam::quat(vertex.x,vertex.y,vertex.z,0.0)
}

impl Model {
    // calculates the 'center' of an object from the average of all its verticies
    pub fn calculate_centroid(verticies: &Verticies) -> glam::Vec4 {
        let mut sum = glam::vec4(0.0, 0.0, 0.0, 0.0);

        for vertex in verticies.values() {
            sum = sum + vertex;
        }
        let count = verticies.iter().count() as f32;
        let average = sum / glam::vec4(count, count, count, count);
        average
    }
    pub fn try_load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = obj::parse_obj(path)?;
        let verticies = obj.into_verticies();
        let centroid = Self::calculate_centroid(&verticies);
        Ok(Self {
            verticies: verticies,
            origin: centroid.xyz(),
            // y_up: glam::vec3(0.0, 1.0, 0.0),
            // x_right: glam::vec3(1.0, 0.0, 0.0),
            // z: glam::vec3(0.0, 0.0, 1.0)
        })
    }
    pub fn verticies_iter(&self) -> std::collections::btree_map::Iter<'_, u32, glam::Vec4> {
        self.verticies.iter()
    }
    pub fn rotate_to_degrees(&self, x_degrees: f32, y_degrees: f32, z_degrees: f32) -> Self {
        // calcluate rotation around X axis.
        let x_degrees = x_degrees.clamp(0.0, 359.999);
        let y_degrees = y_degrees.clamp(0.0, 359.999);
        let z_degrees = z_degrees.clamp(0.0, 359.999);
        let verticies: BTreeMap<_, _> = self
            .verticies
            .clone()
            .into_iter()
            .map(|(index, vertex)| {
                let mut n_vertex = vertex.clone();
                n_vertex.y = vertex.y * x_degrees.to_degrees().cos() - vertex.x * x_degrees.to_degrees().sin();
                n_vertex.z = vertex.y * x_degrees.to_degrees().sin() + vertex.z * x_degrees.to_degrees().cos();
                n_vertex.w = 0.0;
                (index, n_vertex)
            })
            .collect();
        let centroid = Self::calculate_centroid(&verticies);
        Self {
            verticies,
            origin: centroid.xyz(),
        }
    }
    pub fn origin(&self) -> glam::Vec3{
        self.origin
    }
    // translates the model's verticies by the given 'position' and then returns the new verticies
    pub fn translate(&self, position: glam::Vec3) -> Self {
        let verticies: BTreeMap<_, _> = self
            .verticies
            .clone()
            .into_iter()
            .map(|(index, vertex)| {
                let translation_matrix = glam::Mat4::from_translation(position);
                let product = (translation_matrix).transform_point3(vertex.xyz());
                (index, glam::vec4(product.x, product.y, product.z, 1.0))
            })
            .collect();
        let centroid = Self::calculate_centroid(&verticies);
        Self {
            verticies,
            origin: centroid.xyz(),
        }
    }
}


/// calculates a rotation matrix based on Rodrigues' rotation formula thanks go to google gemini for finding this formula
pub fn rodregues_rotation_matrix(axis: glam::Vec3, angle: f32) -> glam::Mat3 {
    // let axis = axis.normalize_or_zero();
    let a = angle.cos();
    let b = angle.sin();
    glam::mat3(
        glam::vec3(
            a + (1.0 - a) * axis.x.powf(2.0),
            (1.0 - a) * axis.x * axis.y - b * axis.z,
            (1.0 - a) * axis.z + b * axis.y,
        ),
        glam::vec3(
            (1.0 - a) * axis.y * axis.x + b * axis.z,
            a + (1.0 - a) * axis.y.powf(2.0),
            (1.0 - a) * axis.y * axis.z - b * axis.x,
        ),
        glam::vec3(
            (1.0 - a) * axis.y * axis.x,
            (1.0 - a) * axis.z * axis.y + b * axis.x,
            a + (1.0 - a) * axis.z.powf(2.0),
        ),
        // glam::vec4(0.0, 0.0, 0.0, 0.0)
    )
}

pub fn rotate_vec3(origin_point: glam::Vec3,vertex:glam::Vec3,x_rotation:f32,y_rotation:f32,z_rotation:f32) -> glam::Vec3 {
    // subtact the vertex from the origin point
    let vertex = vertex - origin_point;
    let mut vertex = vertex_x_rotation_matrix3(x_rotation) * vertex;
    vertex = vertex_y_rotation_matrix3(y_rotation) * vertex;
    vertex = vertex_z_rotation_matrix3(z_rotation) * vertex;
    vertex + origin_point
}
/// combines X,Y,Z rotations in this order assuming positve X,Y,Z axis


#[test]
pub fn test_vec3_rotate_x_quat() {
    let origin = glam::vec3(0.0, 0.0, 0.0);
    let vertex = glam::vec3(1.0,1.0,1.0);
    let x_roation = 1.0;
    let y_rotation = 1.0;
    let z_rotation = 1.0;
    let rotated_vertex = vec3_rotate_using_quaternions(vertex, 1.0, 1.0, 1.0);
}


/// performs a rotation around the X axis, assuming the point is around the origin (0,0,0)
/// if the point of origin is not (0,0,0) then you must translate the vertex so the position is around the origin point
/// then call this function
pub fn vec3_rotate_using_quaternions(p:glam::Vec3,theta_x: f32,theta_y:f32,theta_z:f32) -> glam::Vec3 {
    // glam::Quat::from_axis_angle(axis, angle)
    let p = glam::quat(p.x,p.y,p.z,0.0);
    
    let q_x = glam::Quat::from_axis_angle(glam::Vec3::X,theta_x.to_radians());
    let r_x = ((q_x * p)* q_x.inverse());
    println!("{r_x}");

    // let p_y = glam::quat(r_x.x, r_x.y, r_x.z,0.0);
    let q_y = glam::Quat::from_axis_angle(glam::Vec3::Y,theta_y.to_radians()); 
    let r_y = ((q_y * r_x) * q_y.inverse());

    let p_z = glam::quat(r_y.x, r_y.y, r_y.z,0.0);
    let q_z = glam::Quat::from_axis_angle(glam::Vec3::Z,theta_z.to_radians());
    let r_z = ((q_z * p_z) * q_z.inverse());
    r_z.xyz()
}
pub fn vertex_x_rotation_matrix3(theta: f32) -> glam::Mat3 {
    let sine = theta.sin();
    let cosine = theta.cos();
    glam::mat3(
        glam::vec3(1.0, 0.0, 0.0),
        glam::vec3(0.0, sine, -sine),
        glam::vec3(0.0, sine, cosine),
    )
}
pub fn vertex_y_rotation_matrix3(theta: f32) -> glam::Mat3 {
    let sine = theta.sin();
    let cosine = theta.cos();
      glam::mat3(
        glam::vec3(cosine,0.0,sine),
        glam::vec3(0.0, 1.0,0.0),
        glam::vec3(sine , 0.0,   cosine))
}
pub fn vertex_z_rotation_matrix3(theta: f32) -> glam::Mat3 {
    let sine = theta.sin();
    let cosine = theta.cos();
    glam::mat3(
        glam::vec3(cosine, -sine, 0.0),
        glam::vec3(sine, cosine, 0.0),
        glam::vec3(0.0, 0.0, 1.0),
    )
}

/*
Applying Translation to Individual Vertices Instead of the Model Matrix: When you want to translate an entire object, you need to apply the translation to the model matrix, which represents the object's position, rotation, and scale in the world.
If you're manually translating each vertex individually instead of applying a unified transformation to the model, you could accidentally apply different translations to different vertices, causing them to separate.


*/

#[derive(Clone)]
pub struct Model {
    verticies: Verticies,
    origin: glam::Vec3,
    // y_up: glam::Vec3,
    // x_right: glam::Vec3,
    // z: glam::Vec3,
}

pub type Verticies = std::collections::BTreeMap<u32, glam::Vec4>;

use crate::obj;

use glam::Vec4Swizzles;
use std::collections::BTreeMap;
