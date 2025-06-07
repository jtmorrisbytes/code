// bindings from x11/x.h

// _XSERVER64?
const X_PROTOCOL: usize = 11;
const X_PROTOCOL_VERSION: usize = 0;


type XID = std::ffi::c_ulong;
type Mask = std::ffi::c_ulong;
type VisualID = std::ffi::c_ulong;


type Window = XID;
type Drawable = XID;

/// universal null resource or null atom
const None: std::ffi::c_ulong = 0L;
/// background pixmap in CreateWindow
const ParentRelative: std::ffi::c_ulong = 1L;
/// border pixmap in CreateWindow
const CopyFromParent: std::ffi::c_ulong = 0L;