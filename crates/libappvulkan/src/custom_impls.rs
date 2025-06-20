impl std::fmt::Display for crate::bindings::VkExtensionProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extension_name =
            unsafe { std::ffi::CStr::from_ptr(self.extensionName.as_ptr()) }.to_string_lossy();
        write!(
            f,
            "VkExtensionProperties {{\n extensionName:{}\n specVersion:{}",
            extension_name, self.specVersion
        )
    }
}
impl std::fmt::Debug for crate::bindings::VkExtensionProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extension_name =
            unsafe { std::ffi::CStr::from_ptr(self.extensionName.as_ptr()) }.to_string_lossy();
        write!(
            f,
            "VkExtensionProperties {{\n extensionName: \"{}\",\n specVersion:{}\n}}",
            extension_name, self.specVersion
        )
    }
}
impl std::fmt::Display for crate::bindings::VkLayerProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let layer_name =
            unsafe { std::ffi::CStr::from_ptr(self.layerName.as_ptr()) }.to_string_lossy();
        let description =
            unsafe { std::ffi::CStr::from_ptr(self.description.as_ptr()) }.to_string_lossy();

        write!(f,"VkLayerProperties {{\n layerName: \"{}\", specVersion:{}, implementationVersion:{},description: \"{}\"}}",layer_name,self.specVersion,self.implementationVersion,description)
    }
}
impl std::fmt::Debug for crate::bindings::VkLayerProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let layer_name =
            unsafe { std::ffi::CStr::from_ptr(self.layerName.as_ptr()) }.to_string_lossy();
        let description =
            unsafe { std::ffi::CStr::from_ptr(self.description.as_ptr()) }.to_string_lossy();

        write!(f,"VkLayerProperties {{\n layerName: \"{}\",\n specVersion:{},\n implementationVersion:{},\n description: \"{}\"}}",layer_name,self.specVersion,self.implementationVersion,description)
    }
}
