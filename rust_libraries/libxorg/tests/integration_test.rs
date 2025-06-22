use libxorg::Display;
#[test]
pub fn int_test_display_try_open() {
    let disp =  Display::try_open(None).unwrap();
    let disp = Display::try_open(Some(":1")).unwrap();
}