use wasm_bindgen::prelude::*;
use crate::{FrontendApp,FrontendAppProperties,Route};

#[wasm_bindgen::prelude::wasm_bindgen(start)]
#[no_mangle]
/// starts the client side application
pub async fn start() -> Result<(), wasm_bindgen::JsValue> {
    use std::str::FromStr;

    use wasm_bindgen::prelude::*;
    use yew::Renderer;
    // set a panic handler
    std::panic::set_hook(Box::new(|panic_info: &std::panic::PanicInfo|{
        if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            let err_msg = format!("Web App Panicked! panic message {s}");
            let err_msg_jsvalue = JsValue::from_str(&err_msg);
            web_sys::console::error_1(&err_msg_jsvalue)
        }
        else {
            
            let err_msg_jsvalue = JsValue::from_str("Web App Panicked! no panic message available");
            web_sys::console::error_1(&err_msg_jsvalue)
        }
    }));
    // create a renderer that mounts at DocumentElement
    web_sys::console::log_1(&JsValue::from_str("starting web app"));
    let window = web_sys::window().unwrap();
    
    

    // we expect the server to insert an <input type="text" hidden=true id="initial-data" />
    // into the body of the webpage which is a base64-encoded messagepack data structure that holds the current route information

    let document_element = window.document().unwrap();
    let element = document_element.query_selector(r#"body > input[type="text"]#initial-data"#).expect("Query selector error").expect("Failed to find element");
    let html_element: web_sys::HtmlInputElement = element.dyn_into().expect("Failed to cast element to input element for initial data");
    let value = html_element.value();
    
    use base64::Engine;
    let bytes = base64::prelude::BASE64_STANDARD.decode(value).expect("Failed to base64 decode initial data");
    let initial_properties: FrontendAppProperties = rmp_serde::from_slice(&bytes).expect("failed to msgpack decode intial data");
    // yew expects an exact copy of what the client will render after the webpage loads, which DOES NOT include the intitial data element. 
    html_element.remove();



    // mount to body element
    let body_element = document_element.body().unwrap();
    // let props = FrontendAppProperties {route};
    let mut renderer: Renderer<FrontendApp> = Renderer::with_root_and_props(web_sys::Element::from(body_element),initial_properties);
    // let mut renderer: Renderer<FrontendApp> = Renderer::with_root(web_sys::Element::from(body_element));

    renderer.hydrate();
    Ok(())
}
