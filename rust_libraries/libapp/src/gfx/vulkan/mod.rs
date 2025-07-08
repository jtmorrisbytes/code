pub struct Backend {
    vk_instance: Option<vulkano::instance::Instance>
}

pub struct CreateVulkanInstanceArgs {

}


impl super::Backend for Backend{}
impl super::Initialize for Backend {
    type Args = CreateVulkanInstanceArgs;
    fn try_initialize(args:Self::Args) -> Result<Box<dyn super::Backend>, Box<dyn std::error::Error>> {
        

        Ok(Box::new(self::Backend{vk_instance:None}))
    }

}

