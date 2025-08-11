

#[cfg(target_os="windows")]
pub enum XInputGetStateResult {
    ControllerConnected(XboxControllerInputState),
    ControllerNotConnected,
    Error(u32)
}


#[cfg(target_os="windows")]
pub fn load_xinput_dll() -> Result<windows::Win32::Foundation::HMODULE, windows::core::Error> {
    unsafe {    
        windows::Win32::System::LibraryLoader::LoadLibraryA(
            windows::Win32::UI::Input::XboxController::XINPUT_DLL_A
        )
    }

}

#[cfg(target_os="windows")]
#[test]
pub fn test_xbox_controller_xinput() -> Result<(),Box<dyn std::error::Error>> {
    let module = load_xinput_dll()?;
    
    for _ in 0..65535 {

        let state = xinput_get_state(module,0);
        match state {
            XInputGetStateResult::ControllerConnected(gamepad )=>{
                println!("{:?}",gamepad)
            },
            _=>{}
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
    }




    unload_xinput_dll(module)?;
    Ok(())
}#[cfg(target_os="windows")]
pub fn unload_xinput_dll(module:windows::Win32::Foundation::HMODULE) -> Result<(), windows::core::Error> {
    unsafe {
        windows::Win32::Foundation::FreeLibrary(module)
    }
}

#[cfg(target_os="windows")]
pub fn xinput_get_state(module:windows::Win32::Foundation::HMODULE,controller_number: u32) -> XInputGetStateResult {

    const FN_NAME: &std::ffi::CStr = c"XInputGetState";
    let fn_name_bytes = FN_NAME.to_bytes_with_nul();
    let fn_name_ptr = fn_name_bytes.as_ptr();
    let fn_name_pcstr = windows::core::PCSTR::from_raw(fn_name_ptr);
    let fn_pointer = unsafe {windows::Win32::System::LibraryLoader::GetProcAddress(module, fn_name_pcstr )};
    let fn_pointer_xinput_get_state: Option<unsafe extern "system" fn(u32,*mut windows::Win32::UI::Input::XboxController::XINPUT_STATE) -> u32> = unsafe {std::mem::transmute(fn_pointer)};
    let fn_pointer_xinput_get_state = fn_pointer_xinput_get_state.expect("XInputGetState is not available");

    let mut input_state: windows::Win32::UI::Input::XboxController::XINPUT_STATE = Default::default();

    let status = unsafe {fn_pointer_xinput_get_state(0,&mut input_state)};
    match windows::Win32::Foundation::WIN32_ERROR(status)  {
        windows::Win32::Foundation::ERROR_SUCCESS =>{},
        windows::Win32::Foundation::ERROR_DEVICE_NOT_CONNECTED => return XInputGetStateResult::ControllerNotConnected,
        error => {return XInputGetStateResult::Error(error.0)}
    }
    // process the controller input and return a structure that represents the controller
    let mut controller_state = XboxControllerInputState::default();
    controller_state.raw_button_flags = input_state.Gamepad.wButtons.0;
    controller_state.left_thumbstick_x_axis = input_state.Gamepad.sThumbLX;
    controller_state.left_thumbstick_y_axis = input_state.Gamepad.sThumbLY;
    controller_state.right_thumbstick_x_axis = input_state.Gamepad.sThumbRX;
    controller_state.right_thumbstick_y_axis = input_state.Gamepad.sThumbRY;
    controller_state.left_trigger = input_state.Gamepad.bLeftTrigger;
    controller_state.right_trigger = input_state.Gamepad.bRightTrigger;




    // check the button state
    let wbuttons = input_state.Gamepad.wButtons;
    dbg!(wbuttons);
    if wbuttons &  windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_DPAD_UP == windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_DPAD_UP {
        controller_state.dpad_up = true;
    }
    if wbuttons & windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_DPAD_DOWN == windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_DPAD_DOWN {
        controller_state.dpad_down = true;
    }
    if wbuttons & windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_DPAD_LEFT == windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_DPAD_LEFT {
        controller_state.dpad_left = true
    }
    if wbuttons & windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_DPAD_RIGHT == windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_DPAD_RIGHT {
        controller_state.dpad_right = true
    }
    if wbuttons &  windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_START ==  windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_START {
        controller_state.start = true;
    }
    if wbuttons.0 & 0b100000 == 32 {
        controller_state.select = true;
    }
    if wbuttons &  windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_LEFT_THUMB ==  windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_LEFT_THUMB {
        controller_state.left_thumbstick_pressed = true;
    }
    if wbuttons & windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_RIGHT_THUMB  == windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_RIGHT_THUMB {
        controller_state.right_thumbstick_pressed = true;
    }
    if wbuttons & windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_RIGHT_SHOULDER  == windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_LEFT_SHOULDER  {
        controller_state.left_shoulder = true;
    }
    if wbuttons & windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_RIGHT_SHOULDER == windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_RIGHT_SHOULDER  {
        controller_state.right_shoulder = true;
    }
    // some bits unused
    if wbuttons & windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_A == windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_A {
        controller_state.a = true;
    }
    if wbuttons & windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_B == windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_B {
        controller_state.b = true;
    }
    if wbuttons & windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_X == windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_X {
        controller_state.x = true;
    }
    if wbuttons & windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_Y == windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_Y {
        controller_state.y = true;
    }
    XInputGetStateResult::ControllerConnected(controller_state)

}
#[derive(Default,Debug,PartialEq,Eq)]
pub struct XboxControllerInputState {
    pub controller_id: u8,
    pub raw_button_flags:u16,
    pub left_trigger: u8,
    pub right_trigger:u8,
    pub left_thumbstick_x_axis:  i16,
    pub left_thumbstick_y_axis:  i16,
    pub right_thumbstick_x_axis: i16,
    pub right_thumbstick_y_axis: i16,
    pub a:bool,
    pub b:bool,
    pub x:bool,
    pub y:bool,
    pub dpad_up:bool,
    pub dpad_left:bool,
    pub dpad_right:bool,
    pub dpad_down:bool,
    pub left_thumbstick_pressed:bool,
    pub right_thumbstick_pressed:bool,
    pub right_shoulder:bool,
    pub left_shoulder:bool,
    pub start:bool,
    pub select:bool
}
// note Xinput api is only available on windows

#[cfg(target_os="windows")]
use windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD;
