pub mod opengl;
pub mod vulkan;






pub trait Backend {
}

pub trait Initialize where Self: Backend {
    type Args;
    fn try_initialize(args:Self::Args) -> Result<Box<dyn Backend>,Box<dyn std::error::Error>>;
}


pub fn intialize() -> Result<Box<dyn Backend>,Box<dyn std::error::Error>> {

    // if let Ok(vulkan) = self::vulkan::Backend::try_initialize(vulkan::CreateVulkanInstanceArgs{}) {
    //     return Ok(vulkan);
    // }
    // else if let Ok(opengl) = self::opengl::Backend::try_initialize(opengl::CreateOpenGLArgs{}) {
    //     return Ok(opengl)
    // }
    // else {return Err("Failed to initialize a graphics backend".into())}
    todo!()

}