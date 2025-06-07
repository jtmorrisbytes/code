#[cfg(target_os="linux")]
fn main() -> Result<(),Box<dyn std::error::Error>> {
    println!("{}",code::sys::x11::x::X_PROTOCOL);
    use xcb::x;

    let (conn,screen_num) = xcb::Connection::connect(None)?;

    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();

    let window: xcb::x::Window = conn.generate_id();

        let cookie = conn.send_request_checked(&x::CreateWindow {
        depth: x::COPY_FROM_PARENT as u8,
        wid: window,
        parent: screen.root(),
        x: 0,
        y: 0,
        width: 150,
        height: 150,
        border_width: 0,
        class: x::WindowClass::InputOutput,
        visual: screen.root_visual(),
        // this list must be in same order than `Cw` enum order
        value_list: &[
            x::Cw::BackPixel(screen.white_pixel()),
            x::Cw::EventMask(x::EventMask::EXPOSURE | x::EventMask::KEY_PRESS)
        ],
    });
    conn.check_request(cookie)?;

        conn.send_request(&x::MapWindow {
        window,
    });

    let (wm_protocols, wm_del_window, wm_state, wm_state_maxv, wm_state_maxh) = {
        let cookies = (
            conn.send_request(&x::InternAtom {
                only_if_exists: true,
                name: b"WM_PROTOCOLS",
            }),
            conn.send_request(&x::InternAtom {
                only_if_exists: true,
                name: b"WM_DELETE_WINDOW",
            }),
            conn.send_request(&x::InternAtom {
                only_if_exists: true,
                name: b"_NET_WM_STATE",
            }),
            conn.send_request(&x::InternAtom {
                only_if_exists: true,
                name: b"_NET_WM_STATE_MAXIMIZED_VERT",
            }),
            conn.send_request(&x::InternAtom {
                only_if_exists: true,
                name: b"_NET_WM_STATE_MAXIMIZED_HORZ",
            }),
        );
        (
            conn.wait_for_reply(cookies.0)?.atom(),
            conn.wait_for_reply(cookies.1)?.atom(),
            conn.wait_for_reply(cookies.2)?.atom(),
            conn.wait_for_reply(cookies.3)?.atom(),
            conn.wait_for_reply(cookies.4)?.atom(),
        )
    };
    conn.check_request(conn.send_request_checked(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window,
        property: wm_protocols,
        r#type: x::ATOM_ATOM,
        data: &[wm_del_window],
    }))?;

    loop {
        let event = conn.wait_for_event()?;
        dbg!(event);
    }

    println!("hello world");
    Ok(())
}

#[cfg(target_os="windows")]
pub fn main() {
    
}