fn main() -> Result<(),Box<dyn std::error::Error>> {
    // println!("{}",libapp::sys::x11::x::X_PROTOCOL);
    
    use libapp::Application;
    let event_loop = libapp::create_event_loop()?;
    let mut app = Application::new();
    event_loop.run_app(&mut app)?;
    Ok(())   
}