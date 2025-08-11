#![allow(non_camel_case_types, non_upper_case_globals)]
// what do we do if we are compiling for solaris and not the gnu env
#[cfg(all(target_os = "solaris", not(target_env = "gnu")))]
compile_error!("The crate author is not sure what to do in the case of compiling for solaris before sun version 4");

/* Window stacking method (in configureWindow) */

const Above: c_int = 0;
///special Resource ID passed to KillClient
const AllTemporary: c_int = 0;
const AllocAll: c_int = 1; /* allocate entire map writeable */

/*****************************************************************
 *  COLOR MAP STUFF
 *****************************************************************/

/* For CreateColormap */

const AllocNone: c_int = 0; /* create map with no entries */
const AllowExposures: c_int = 1;
const AlreadyGrabbed: c_int = 1;
const Always: c_int = 2;
//special Button Code, passed to GrabButton
const AnyButton: c_int = 0;
//special Key Code, passed to GrabKey
const AnyKey: KeyCode = 0;

const AnyModifier: c_int = 1 << 15; /* used in GrabButton, GrabKey */

/// special Atom, passed to GetProperty
const AnyPropertyType: Atom = 0;

/* Arc modes for PolyFillArc */

const ArcChord: c_int = 0; /* join endpoints of arc */
const ArcPieSlice: c_int = 1; /* join endpoints to center of arc */
const AsyncBoth: c_int = 6;
const AsyncKeyboard: c_int = 3;

/* AllowEvents modes */

const AsyncPointer: c_int = 0;
const AutoRepeatModeDefault: c_int = 2;

/*****************************************************************
 * KEYBOARD/POINTER STUFF
 *****************************************************************/

const AutoRepeatModeOff: c_int = 0;
const AutoRepeatModeOn: c_int = 1;
///depending on context:
///
/// - key/button already grabbed
/// - attempt to free an illegal cmap entry
/// - attempt to store into a read-only color map entry.
/// - attempt to modify the access control list from other than the local host.
const BadAccess: c_int = 10;
///insufficient resources
const BadAlloc: c_int = 11;
///parameter not an Atom
const BadAtom: c_int = 5;
///no such colormap
const BadColor: c_int = 12;
///parameter not a Cursor
const BadCursor: c_int = 6;
///parameter not a Pixmap or Window
const BadDrawable: c_int = 9;
///parameter not a Font
const BadFont: c_int = 7;
///parameter not a GC
const BadGC: c_int = 13;
///choice not in range or already used
const BadIDChoice: c_int = 14;
/* server is defective */
const BadImplementation: c_int = 17;
///Request length incorrect
const BadLength: c_int = 16;
///parameter mismatch
const BadMatch: c_int = 8;
//font or color name doesn't exist */
const BadName: c_int = 15;
/* parameter not a Pixmap */
const BadPixmap: c_int = 4;
/* bad request code */
const BadRequest: c_int = 1;
/* int parameter out of range */
const BadValue: c_int = 2;
/* parameter not a Window */
const BadWindow: c_int = 3;
const Below: c_int = 1;
const BottomIf: c_int = 3;

/* button names. Used as arguments to GrabButton and as detail in ButtonPress
and ButtonRelease events.  Not to be confused with button masks above.
Note that 0 is already defined above as "AnyButton".  */

const Button1: c_int = 1;

/* button masks.  Used in same manner as Key masks above. Not to be confused
with button names below. */

const Button1Mask: c_int = 1 << 8;
const Button1MotionMask: c_int = 1 << 8;
const Button2: c_int = 2;
const Button2Mask: c_int = 1 << 9;
const Button2MotionMask: c_int = 1 << 9;
const Button3: c_int = 3;
const Button3Mask: c_int = 1 << 10;
const Button3MotionMask: c_int = 1 << 10;
const Button4: c_int = 4;
const Button4Mask: c_int = 1 << 11;
const Button4MotionMask: c_int = 1 << 11;
const Button5: c_int = 5;
const Button5Mask: c_int = 1 << 12;
const Button5MotionMask: c_int = 1 << 12;
const ButtonMotionMask: c_int = 1 << 13;
const ButtonPress: c_int = 4;
const ButtonPressMask: c_int = 1 << 2;
const ButtonRelease: c_int = 5;
const ButtonReleaseMask: c_int = 1 << 3;
const CWBackPixel: c_int = 1 << 1;

/* Window attributes for CreateWindow and ChangeWindowAttributes */

const CWBackPixmap: c_int = 1 << 0;
const CWBackingPixel: c_int = 1 << 8;
const CWBackingPlanes: c_int = 1 << 7;
const CWBackingStore: c_int = 1 << 6;
const CWBitGravity: c_int = 1 << 4;
const CWBorderPixel: c_int = 1 << 3;
const CWBorderPixmap: c_int = 1 << 2;
const CWBorderWidth: c_int = 1 << 4;
const CWColormap: c_int = 1 << 13;
const CWCursor: c_int = 1 << 14;
const CWDontPropagate: c_int = 1 << 12;
const CWEventMask: c_int = 1 << 11;
const CWHeight: c_int = 1 << 3;
const CWOverrideRedirect: c_int = 1 << 9;
const CWSaveUnder: c_int = 1 << 10;
const CWSibling: c_int = 1 << 5;
const CWStackMode: c_int = 1 << 6;
const CWWidth: c_int = 1 << 2;
const CWWinGravity: c_int = 1 << 5;

/* ConfigureWindow structure */

const CWX: c_int = 1 << 0;
const CWY: c_int = 1 << 1;
const CapButt: c_int = 1;

/* capStyle */

const CapNotLast: c_int = 0;
const CapProjecting: c_int = 3;
const CapRound: c_int = 2;
const CenterGravity: c_int = 5;
const CirculateNotify: c_int = 26;
const CirculateRequest: c_int = 27;
const ClientMessage: c_int = 33;

/* subwindow mode */

const ClipByChildren: c_int = 0;
const ColormapChangeMask: c_int = 1 << 23;
const ColormapInstalled: c_int = 1;
const ColormapNotify: c_int = 32;

/* Color Map notification */

const ColormapUninstalled: c_int = 0;

/* Polygon shapes */

const Complex: c_int = 0; /* paths may intersect */
const ConfigureNotify: c_int = 22;
const ConfigureRequest: c_int = 23;
const ControlMapIndex: c_int = 2;
const ControlMask: c_int = 1 << 2;
const Convex: c_int = 2; /* wholly convex */

/* CoordinateMode for drawing routines */

const CoordModeOrigin: c_int = 0; /* relative to the origin */
const CoordModePrevious: c_int = 1; /* relative to previous point */
///border pixmap in CreateWindow and ChangeWindowAttributes special VisualID and special window class passed to CreateWindow */
const CopyFromParent: c_int = 0;
const CreateNotify: c_int = 16;
///special Time
const CurrentTime: c_int = 0;

/*****************************************************************
 * CURSOR STUFF
 *****************************************************************/

/* QueryBestSize Class */

const CursorShape: c_int = 0; /* largest size that can be displayed */
const DefaultBlanking: c_int = 2;
const DefaultExposures: c_int = 2;

/* Used in ChangeCloseDownMode */

const DestroyAll: c_int = 0;
const DestroyNotify: c_int = 17;
const DirectColor: c_int = 5;
const DisableAccess: c_int = 0;
const DisableScreenInterval: c_int = 0;

const DisableScreenSaver: c_int = 0;
const DoBlue: c_int = 1 << 2;
const DoGreen: c_int = 1 << 1;

/* Flags used in StoreNamedColor, StoreColors */

const DoRed: c_int = 1 << 0;

const DontAllowExposures: c_int = 0;

/*****************************************************************
 * SCREEN SAVER STUFF
 *****************************************************************/

const DontPreferBlanking: c_int = 0;
const EastGravity: c_int = 6;

/* for ChangeAccessControl */

const EnableAccess: c_int = 1;
const EnterNotify: c_int = 7;
const EnterWindowMask: c_int = 1 << 4;

/* fillRule */

const EvenOddRule: c_int = 0;
const Expose: c_int = 12;
const ExposureMask: c_int = 1 << 15;
const FamilyChaos: c_int = 2;
const FamilyDECnet: c_int = 1;
const FamilyInternet6: c_int = 6; /* IPv6 */

/* protocol families */

const FamilyInternet: c_int = 0; /* IPv4 */

/* authentication families not tied to a specific protocol */
const FamilyServerInterpreted: c_int = 5;
const FillOpaqueStippled: c_int = 3;

/* fillStyle */

const FillSolid: c_int = 0;
const FillStippled: c_int = 2;
const FillTiled: c_int = 1;

const FirstExtensionError: c_int = 128;
const FocusChangeMask: c_int = 1 << 21;
const FocusIn: c_int = 9;
const FocusOut: c_int = 10;

const FontChange: c_int = 255;

/*****************************************************************
 * FONTS
 *****************************************************************/

/* used in QueryFont -- draw direction */

const FontLeftToRight: c_int = 0;
const FontRightToLeft: c_int = 1;

/* Bit Gravity */

const ForgetGravity: c_int = 0;
const GCArcMode: c_int = 1 << 22;
const GCBackground: c_int = 1 << 3;
const GCCapStyle: c_int = 1 << 6;
const GCClipMask: c_int = 1 << 19;
const GCClipXOrigin: c_int = 1 << 17;
const GCClipYOrigin: c_int = 1 << 18;
const GCDashList: c_int = 1 << 21;
const GCDashOffset: c_int = 1 << 20;
const GCFillRule: c_int = 1 << 9;
const GCFillStyle: c_int = 1 << 8;
const GCFont: c_int = 1 << 14;
const GCForeground: c_int = 1 << 2;

/* GC components: masks used in CreateGC, CopyGC, ChangeGC, OR'ed into
GC.stateChanges */

const GCFunction: c_int = 1 << 0;
const GCGraphicsExposures: c_int = 1 << 16;
const GCJoinStyle: c_int = 1 << 7;

const GCLastBit: c_int = 22;
const GCLineStyle: c_int = 1 << 5;
const GCLineWidth: c_int = 1 << 4;
const GCPlaneMask: c_int = 1 << 1;
const GCStipple: c_int = 1 << 11;
const GCSubwindowMode: c_int = 1 << 15;
const GCTile: c_int = 1 << 10;
const GCTileStipXOrigin: c_int = 1 << 12;
const GCTileStipYOrigin: c_int = 1 << 13;
const GXand: c_int = 0x1; /* src AND dst */
const GXandInverted: c_int = 0x4; /* NOT src AND dst */
const GXandReverse: c_int = 0x2; /* src AND NOT dst */

/*****************************************************************
 * GRAPHICS DEFINITIONS
 *****************************************************************/

/* graphics functions, as in GC.alu */

const GXclear: c_int = 0x0; /* 0 */
const GXcopy: c_int = 0x3; /* src */
const GXcopyInverted: c_int = 0xc; /* NOT src */
const GXequiv: c_int = 0x9; /* NOT src XOR dst */
const GXinvert: c_int = 0xa; /* NOT dst */
const GXnand: c_int = 0xe; /* NOT src OR NOT dst */
const GXnoop: c_int = 0x5; /* dst */
const GXnor: c_int = 0x8; /* NOT src AND NOT dst */
const GXor: c_int = 0x7; /* src OR dst */
const GXorInverted: c_int = 0xd; /* NOT src OR dst */
const GXorReverse: c_int = 0xb; /* src OR NOT dst */
const GXset: c_int = 0xf; /* 1 */
const GXxor: c_int = 0x6; /* src XOR dst */
const GenericEvent: c_int = 35;
const GrabFrozen: c_int = 4;
const GrabInvalidTime: c_int = 2;
const GrabModeAsync: c_int = 1;

/* GrabPointer, GrabButton, GrabKeyboard, GrabKey Modes */

const GrabModeSync: c_int = 0;
const GrabNotViewable: c_int = 3;

/* GrabPointer, GrabKeyboard reply status */

const GrabSuccess: c_int = 0;
const GraphicsExpose: c_int = 13;
const GravityNotify: c_int = 24;
const GrayScale: c_int = 1;
const HostDelete: c_int = 1;

/*****************************************************************
 * HOSTS AND CONNECTIONS
 *****************************************************************/

/* for ChangeHosts */

const HostInsert: c_int = 0;
const IncludeInferiors: c_int = 1;
///destination window in SendEvent
const InputFocus: c_int = 1;
const InputOnly: c_int = 2;

/*****************************************************************
 * WINDOW DEFINITIONS
 *****************************************************************/

/* Window classes used by CreateWindow */
/* Note that CopyFromParent is already defined as 0 above */

const InputOutput: c_int = 1;

/* Used in GetWindowAttributes reply */

const IsUnmapped: c_int = 0;
const IsUnviewable: c_int = 1;
const IsViewable: c_int = 2;
const JoinBevel: c_int = 2;

/* joinStyle */

const JoinMiter: c_int = 0;
const JoinRound: c_int = 1;
const KBAutoRepeatMode: c_int = 1 << 7;
const KBBellDuration: c_int = 1 << 3;
const KBBellPercent: c_int = 1 << 1;
const KBBellPitch: c_int = 1 << 2;
const KBKey: c_int = 1 << 6;

/* masks for ChangeKeyboardControl */

const KBKeyClickPercent: c_int = 1 << 0;
const KBLed: c_int = 1 << 4;
const KBLedMode: c_int = 1 << 5;

/* Event names.  Used in "type" field in XEvent structures.  Not to be
confused with event masks above.  They start from 2 because 0 and 1
are reserved in the protocol for errors and replies. */

const KeyPress: c_int = 2;
const KeyPressMask: c_int = 1 << 0;
const KeyRelease: c_int = 3;
const KeyReleaseMask: c_int = 1 << 1;
const KeymapNotify: c_int = 11;
const KeymapStateMask: c_int = 1 << 14;
/* must be bigger than any event # */
const LASTEvent: c_int = 36;

/* Byte order  used in imageByteOrder and bitmapBitOrder */

const LSBFirst: c_int = 0;
const LastExtensionError: c_int = 255;
const LeaveNotify: c_int = 8;
const LeaveWindowMask: c_int = 1 << 5;

const LedModeOff: c_int = 0;
const LedModeOn: c_int = 1;
const LineDoubleDash: c_int = 2;
const LineOnOffDash: c_int = 1;

/* LineStyle */

const LineSolid: c_int = 0;
const LockMapIndex: c_int = 1;
const LockMask: c_int = 1 << 1;
const LowerHighest: c_int = 1;
const MSBFirst: c_int = 1;
const MapNotify: c_int = 19;
const MapRequest: c_int = 20;
const MappingBusy: c_int = 1;
const MappingFailed: c_int = 2;
const MappingKeyboard: c_int = 1;

const MappingModifier: c_int = 0;
const MappingNotify: c_int = 34;
const MappingPointer: c_int = 2;

const MappingSuccess: c_int = 0;
const Mod1MapIndex: c_int = 3;
const Mod1Mask: c_int = 1 << 3;
const Mod2MapIndex: c_int = 4;
const Mod2Mask: c_int = 1 << 4;
const Mod3MapIndex: c_int = 5;
const Mod3Mask: c_int = 1 << 5;
const Mod4MapIndex: c_int = 6;
const Mod4Mask: c_int = 1 << 6;
const Mod5MapIndex: c_int = 7;
const Mod5Mask: c_int = 1 << 7;
const MotionNotify: c_int = 6;

/*****************************************************************
 * EVENT DEFINITIONS
 *****************************************************************/

/* Input Event Masks. Used as event-mask window attribute and as arguments
to Grab requests.  Not to be confused with event names.  */

const NoEventMask: c_int = 0;
const NoExpose: c_int = 14;
const Nonconvex: c_int = 1; /* no paths intersect, but not convex */
///universal null resource or null atom
const None: c_int = 0;
const NorthEastGravity: c_int = 3;
const NorthGravity: c_int = 2;
const NorthWestGravity: c_int = 1;

/* Used in CreateWindow for backing-store hint */

const NotUseful: c_int = 0;

/* Notify detail */

const NotifyAncestor: c_int = 0;
const NotifyDetailNone: c_int = 7;
const NotifyGrab: c_int = 1;

const NotifyHint: c_int = 1; /* for MotionNotify events */
const NotifyInferior: c_int = 2;
const NotifyNonlinear: c_int = 3;
const NotifyNonlinearVirtual: c_int = 4;

/* Notify modes */

const NotifyNormal: c_int = 0;
const NotifyPointer: c_int = 5;
const NotifyPointerRoot: c_int = 6;
const NotifyUngrab: c_int = 2;
const NotifyVirtual: c_int = 1;
const NotifyWhileGrabbed: c_int = 3;
const Opposite: c_int = 4;
const OwnerGrabButtonMask: c_int = 1 << 24;
/// background pixmap in CreateWindow and ChangeWindowAttributes */
const ParentRelative: c_int = 1;
const PlaceOnBottom: c_int = 1;

/* Circulation request */

const PlaceOnTop: c_int = 0;
const PointerMotionHintMask: c_int = 1 << 7;
const PointerMotionMask: c_int = 1 << 6;
/// focus window in SetInputFocus
const PointerRoot: c_int = 1;
///destination window in SendEvent
const PointerWindow: c_int = 0;
const PreferBlanking: c_int = 1;
const PropModeAppend: c_int = 2;
const PropModePrepend: c_int = 1;

/* Property modes */

const PropModeReplace: c_int = 0;
const PropertyChangeMask: c_int = 1 << 22;
const PropertyDelete: c_int = 1;

/* Property notification */

const PropertyNewValue: c_int = 0;
const PropertyNotify: c_int = 28;
const PseudoColor: c_int = 3;

/* Circulation direction */

const RaiseLowest: c_int = 0;
const ReparentNotify: c_int = 21;
const ReplayKeyboard: c_int = 5;
const ReplayPointer: c_int = 2;
const ResizeRedirectMask: c_int = 1 << 18;
const ResizeRequest: c_int = 25;
const RetainPermanent: c_int = 1;
const RetainTemporary: c_int = 2;

/* Used in SetInputFocus, GetInputFocus */

const RevertToNone: c_int = None;
const RevertToParent: c_int = 2;
const RevertToPointerRoot: c_int = PointerRoot;
const ScreenSaverActive: c_int = 1;

/* for ForceScreenSaver */

const ScreenSaverReset: c_int = 0;
const SelectionClear: c_int = 29;
const SelectionNotify: c_int = 31;
const SelectionRequest: c_int = 30;
const SetModeDelete: c_int = 1;

/* Used in ChangeSaveSet */

const SetModeInsert: c_int = 0;

/* modifier names.  Used to build a SetModifierMapping request or
to read a GetModifierMapping request.  These correspond to the
masks defined above. */
const ShiftMapIndex: c_int = 0;

/* Key masks. Used as modifiers to GrabButton and GrabKey, results of QueryPointer,
state in various key-, mouse-, and button-related events. */

const ShiftMask: c_int = 1 << 0;
const SouthEastGravity: c_int = 9;
const SouthGravity: c_int = 8;
const SouthWestGravity: c_int = 7;
const StaticColor: c_int = 2;
const StaticGravity: c_int = 10;

/* Display classes  used in opening the connection
 * Note that the statically allocated ones are even numbered and the
 * dynamically changeable ones are odd numbered */

const StaticGray: c_int = 0;
const StippleShape: c_int = 2; /* size stippled fastest */
const StructureNotifyMask: c_int = 1 << 17;
const SubstructureNotifyMask: c_int = 1 << 19;
const SubstructureRedirectMask: c_int = 1 << 20;

/*****************************************************************
 * ERROR CODES
 *****************************************************************/
/* everything's okay */
const Success: c_int = 0;
const SyncBoth: c_int = 7;
const SyncKeyboard: c_int = 4;
const SyncPointer: c_int = 1;
const TileShape: c_int = 1; /* size tiled fastest */
const TopIf: c_int = 2;
const TrueColor: c_int = 4;

/* Window gravity + bit gravity above */

const UnmapGravity: c_int = 0;
const UnmapNotify: c_int = 18;

/* SetClipRectangles ordering */

const Unsorted: c_int = 0;
const VisibilityChangeMask: c_int = 1 << 16;
const VisibilityFullyObscured: c_int = 2;
const VisibilityNotify: c_int = 15;
const VisibilityPartiallyObscured: c_int = 1;

/* Visibility notify */

const VisibilityUnobscured: c_int = 0;
const WestGravity: c_int = 4;
const WhenMapped: c_int = 1;
const WindingRule: c_int = 1;

/*****************************************************************
 *  IMAGING
 *****************************************************************/

/* ImageFormat -- PutImage, GetImage */

const XYBitmap: c_int = 0; /* depth 1, XYFormat */
const XYPixmap: c_int = 1; /* depth == drawable depth */
const YSorted: c_int = 1;
const YXBanded: c_int = 3;
const YXSorted: c_int = 2;
const ZPixmap: c_int = 2; /* depth == drawable depth */
// we have to actually write our own bindings here UGH! because I cant figure out how to get rust to link external library during tests
// these are for the C compatible definitions
// below are the base defiitions for X11/X.h

/* Definitions for the X window system likely to be used by applications */

// --- AUTHOR NOTE --- below copyright sourced from the headers on the local machine used to develop this

/***********************************************************

Copyright 1987, 1998  The Open Group

Permission to use, copy, modify, distribute, and sell this software and its
documentation for any purpose is hereby granted without fee, provided that
the above copyright notice appear in all copies and that both that
copyright notice and this permission notice appear in supporting
documentation.

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL THE
OPEN GROUP BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of The Open Group shall not be
used in advertising or otherwise to promote the sale, use or other dealings
in this Software without prior written authorization from The Open Group.


Copyright 1987 by Digital Equipment Corporation, Maynard, Massachusetts.

                        All Rights Reserved

Permission to use, copy, modify, and distribute this software and its
documentation for any purpose and without fee is hereby granted,
provided that the above copyright notice appear in all copies and that
both that copyright notice and this permission notice appear in
supporting documentation, and that the name of Digital not be
used in advertising or publicity pertaining to distribution of the
software without specific, written prior permission.

DIGITAL DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE, INCLUDING
ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO EVENT SHALL
DIGITAL BE LIABLE FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR
ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS,
WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS
SOFTWARE.

******************************************************************/

pub const X_PROTOCOL: std::ffi::c_int = 11;
pub const X_PROTOCOL_VERSION: std::ffi::c_int = 11;

// X11 XMD.h

// if we are compiling for the sun operating system "__sun" and Sun version 4 "__SVR4"
// then we must include the type defs for the sun version 4 operating system
#[cfg(all(target_os = "solaris", target_env = "gnu"))]
pub mod isa_defs {}

pub mod types;
pub mod xfuncproto;
pub mod xlib;
pub mod xosdefs;
type Atom = CARD32;
type BITS16 = CARD16;

type BITS32 = CARD32;
type BOOL = CARD8;

type BYTE = CARD8;

type CARD16 = std::ffi::c_short;

#[cfg(target_pointer_width = "64")]
type CARD32 = std::ffi::c_int;
#[cfg(not(target_pointer_width = "64"))]
type CARD32 = std::ffi::c_long;

#[cfg(target_pointer_width = "64")]
type CARD64 = std::ffi::c_ulong;
#[cfg(not(target_pointer_width = "64"))]
type CARD64 = std::ffi::c_ulonglong;

type CARD8 = std::ffi::c_char;
type Cursor = XID;
type Drawable = XID;
type Font = XID;
type GContext = XID;
#[cfg(target_pointer_width = "64")]
type INT32 = std::ffi::c_int;
#[cfg(not(target_pointer_width = "64"))]
type INT32 = std::ffi::c_ulong;

// if compiling for a 64 bit archetecture, specifically if the size of a c_ulong is 8
// then define a typedef INT64 to be a c_ulong;

#[cfg(target_pointer_width = "64")]
type INT64 = std::ffi::c_ulong;

type INT8 = std::ffi::c_schar;
type KeyCode = std::ffi::c_uchar;
type KeySym = XID;
type Mask = CARD32;
type Pixmap = XID;
type Time = CARD32;
type VisualID = CARD32;

type Window = XID;

type XID = CARD32;

// a helper type for preprocssor defs

type c_int = std::ffi::c_int;
