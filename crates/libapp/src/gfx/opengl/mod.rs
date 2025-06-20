#![allow(unused)]

use x11::xlib::XDefaultScreenOfDisplay;

// original copyright inclued from sourced headers
/*
 * Mesa 3-D graphics library
 *
 * Copyright (C) 1999-2006  Brian Paul   All Rights Reserved.
 * Copyright (C) 2009  VMware, Inc.  All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */
#[cfg(feature = "target_os_linux")]
pub mod glx;

// c defines this type as c_uchar, but we use u8 here
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub enum GLboolean {
    GL_FALSE = 0,
    GL_TRUE = 1,
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct GLbitfield(std::ffi::c_uint);
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GLPrimitives {
    GL_POINTS = 0x0000,
    GL_LINES = 0x0001,
    GL_LINE_LOOP = 0x0002,
    GL_LINE_STRIP = 0x0003,
    GL_TRIANGLES = 0x0004,
    GL_TRIANGLE_STRIP = 0x0005,
    GL_TRIANGLE_FAN = 0x0006,
    GL_QUADS = 0x0007,
    GL_QUAD_STRIP = 0x0008,
    GL_POLYGON = 0x0009,
}
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GLVertexArrays {
    /* Vertex Arrays */
    GL_VERTEX_ARRAY = 0x8074,
    GL_NORMAL_ARRAY = 0x8075,
    GL_COLOR_ARRAY = 0x8076,
    GL_INDEX_ARRAY = 0x8077,
    GL_TEXTURE_COORD_ARRAY = 0x8078,
    GL_EDGE_FLAG_ARRAY = 0x8079,
    GL_VERTEX_ARRAY_SIZE = 0x807A,
    GL_VERTEX_ARRAY_TYPE = 0x807B,
    GL_VERTEX_ARRAY_STRIDE = 0x807C,
    GL_NORMAL_ARRAY_TYPE = 0x807E,
    GL_NORMAL_ARRAY_STRIDE = 0x807F,
    GL_COLOR_ARRAY_SIZE = 0x8081,
    GL_COLOR_ARRAY_TYPE = 0x8082,
    GL_COLOR_ARRAY_STRIDE = 0x8083,
    GL_INDEX_ARRAY_TYPE = 0x8085,
    GL_INDEX_ARRAY_STRIDE = 0x8086,
    GL_TEXTURE_COORD_ARRAY_SIZE = 0x8088,
    GL_TEXTURE_COORD_ARRAY_TYPE = 0x8089,
    GL_TEXTURE_COORD_ARRAY_STRIDE = 0x808A,
    GL_EDGE_FLAG_ARRAY_STRIDE = 0x808C,
    GL_VERTEX_ARRAY_POINTER = 0x808E,
    GL_NORMAL_ARRAY_POINTER = 0x808F,
    GL_COLOR_ARRAY_POINTER = 0x8090,
    GL_INDEX_ARRAY_POINTER = 0x8091,
    GL_TEXTURE_COORD_ARRAY_POINTER = 0x8092,
    GL_EDGE_FLAG_ARRAY_POINTER = 0x8093,
    GL_V2F = 0x2A20,
    GL_V3F = 0x2A21,
    GL_C4UB_V2F = 0x2A22,
    GL_C4UB_V3F = 0x2A23,
    GL_C3F_V3F = 0x2A24,
    GL_N3F_V3F = 0x2A25,
    GL_C4F_N3F_V3F = 0x2A26,
    GL_T2F_V3F = 0x2A27,
    GL_T4F_V4F = 0x2A28,
    GL_T2F_C4UB_V3F = 0x2A29,
    GL_T2F_C3F_V3F = 0x2A2A,
    GL_T2F_N3F_V3F = 0x2A2B,
    GL_T2F_C4F_N3F_V3F = 0x2A2C,
    GL_T4F_C4F_N3F_V4F = 0x2A2D,
}
/* Matrix Mode */
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GLMatrixMode {
    GL_MATRIX_MODE = 0x0BA0,
    GL_MODELVIEW = 0x1700,
    GL_PROJECTION = 0x1701,
    GL_TEXTURE = 0x1702,
}
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GLPoints {
    /* Points */
    GL_POINT_SMOOTH = 0x0B10,
    GL_POINT_SIZE = 0x0B11,
    GL_POINT_SIZE_GRANULARITY = 0x0B13,
    GL_POINT_SIZE_RANGE = 0x0B12,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GLLines {
    /* Lines */
    GL_LINE_SMOOTH = 0x0B20,
    GL_LINE_STIPPLE = 0x0B24,
    GL_LINE_STIPPLE_PATTERN = 0x0B25,
    GL_LINE_STIPPLE_REPEAT = 0x0B26,
    GL_LINE_WIDTH = 0x0B21,
    GL_LINE_WIDTH_GRANULARITY = 0x0B23,
    GL_LINE_WIDTH_RANGE = 0x0B22,
}
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GLPolygons {
    /* Polygons */
    GL_POINT = 0x1B00,
    GL_LINE = 0x1B01,
    GL_FILL = 0x1B02,
    GL_CW = 0x0900,
    GL_CCW = 0x0901,
    GL_FRONT = 0x0404,
    GL_BACK = 0x0405,
    GL_POLYGON_MODE = 0x0B40,
    GL_POLYGON_SMOOTH = 0x0B41,
    GL_POLYGON_STIPPLE = 0x0B42,
    GL_EDGE_FLAG = 0x0B43,
    GL_CULL_FACE = 0x0B44,
    GL_CULL_FACE_MODE = 0x0B45,
    GL_FRONT_FACE = 0x0B46,
    GL_POLYGON_OFFSET_FACTOR = 0x8038,
    GL_POLYGON_OFFSET_UNITS = 0x2A00,
    GL_POLYGON_OFFSET_POINT = 0x2A01,
    GL_POLYGON_OFFSET_LINE = 0x2A02,
    GL_POLYGON_OFFSET_FILL = 0x8037,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GLDisplayLists {
    /* Display Lists */
    GL_COMPILE = 0x1300,
    GL_COMPILE_AND_EXECUTE = 0x1301,
    GL_LIST_BASE = 0x0B32,
    GL_LIST_INDEX = 0x0B33,
    GL_LIST_MODE = 0x0B30,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GLDepthBuffer {
    /* Depth buffer */
    GL_NEVER = 0x0200,
    GL_LESS = 0x0201,
    GL_EQUAL = 0x0202,
    GL_LEQUAL = 0x0203,
    GL_GREATER = 0x0204,
    GL_NOTEQUAL = 0x0205,
    GL_GEQUAL = 0x0206,
    GL_ALWAYS = 0x0207,
    GL_DEPTH_TEST = 0x0B71,
    GL_DEPTH_BITS = 0x0D56,
    GL_DEPTH_CLEAR_VALUE = 0x0B73,
    GL_DEPTH_FUNC = 0x0B74,
    GL_DEPTH_RANGE = 0x0B70,
    GL_DEPTH_WRITEMASK = 0x0B72,
    GL_DEPTH_COMPONENT = 0x1902,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GLLighting {
    /* Lighting */
    GL_LIGHTING = 0x0B50,
    GL_LIGHT0 = 0x4000,
    GL_LIGHT1 = 0x4001,
    GL_LIGHT2 = 0x4002,
    GL_LIGHT3 = 0x4003,
    GL_LIGHT4 = 0x4004,
    GL_LIGHT5 = 0x4005,
    GL_LIGHT6 = 0x4006,
    GL_LIGHT7 = 0x4007,
    GL_SPOT_EXPONENT = 0x1205,
    GL_SPOT_CUTOFF = 0x1206,
    GL_CONSTANT_ATTENUATION = 0x1207,
    GL_LINEAR_ATTENUATION = 0x1208,
    GL_QUADRATIC_ATTENUATION = 0x1209,
    GL_AMBIENT = 0x1200,
    GL_DIFFUSE = 0x1201,
    GL_SPECULAR = 0x1202,
    GL_SHININESS = 0x1601,
    GL_EMISSION = 0x1600,
    GL_POSITION = 0x1203,
    GL_SPOT_DIRECTION = 0x1204,
    GL_AMBIENT_AND_DIFFUSE = 0x1602,
    GL_COLOR_INDEXES = 0x1603,
    GL_LIGHT_MODEL_TWO_SIDE = 0x0B52,
    GL_LIGHT_MODEL_LOCAL_VIEWER = 0x0B51,
    GL_LIGHT_MODEL_AMBIENT = 0x0B53,
    GL_FRONT_AND_BACK = 0x0408,
    GL_SHADE_MODEL = 0x0B54,
    GL_FLAT = 0x1D00,
    GL_SMOOTH = 0x1D01,
    GL_COLOR_MATERIAL = 0x0B57,
    GL_COLOR_MATERIAL_FACE = 0x0B55,
    GL_COLOR_MATERIAL_PARAMETER = 0x0B56,
    GL_NORMALIZE = 0x0BA1,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GLCLippingPlanes {
    /* User clipping planes */
    GL_CLIP_PLANE0 = 0x3000,
    GL_CLIP_PLANE1 = 0x3001,
    GL_CLIP_PLANE2 = 0x3002,
    GL_CLIP_PLANE3 = 0x3003,
    GL_CLIP_PLANE4 = 0x3004,
    GL_CLIP_PLANE5 = 0x3005,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GLAccumulationBuffer {
    /* Accumulation buffer */
    GL_ACCUM_RED_BITS = 0x0D58,
    GL_ACCUM_GREEN_BITS = 0x0D59,
    GL_ACCUM_BLUE_BITS = 0x0D5A,
    GL_ACCUM_ALPHA_BITS = 0x0D5B,
    GL_ACCUM_CLEAR_VALUE = 0x0B80,
    GL_ACCUM = 0x0100,
    GL_ADD = 0x0104,
    GL_LOAD = 0x0101,
    GL_MULT = 0x0103,
    GL_RETURN = 0x0102,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GLAlphaTest {
    /* Alpha testing */
    GL_ALPHA_TEST = 0x0BC0,
    GL_ALPHA_TEST_REF = 0x0BC2,
    GL_ALPHA_TEST_FUNC = 0x0BC1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GLAlphaBlend {
    /* Blending */
    GL_BLEND = 0x0BE2,
    GL_BLEND_SRC = 0x0BE1,
    GL_BLEND_DST = 0x0BE0,
    GL_ZERO = 0,
    GL_ONE = 1,
    GL_SRC_COLOR = 0x0300,
    GL_ONE_MINUS_SRC_COLOR = 0x0301,
    GL_SRC_ALPHA = 0x0302,
    GL_ONE_MINUS_SRC_ALPHA = 0x0303,
    GL_DST_ALPHA = 0x0304,
    GL_ONE_MINUS_DST_ALPHA = 0x0305,
    GL_DST_COLOR = 0x0306,
    GL_ONE_MINUS_DST_COLOR = 0x0307,
    GL_SRC_ALPHA_SATURATE = 0x0308,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GLRenderMode {
    /* Render Mode */
    GL_FEEDBACK = 0x1C01,
    GL_RENDER = 0x1C00,
    GL_SELECT = 0x1C02,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GLFeedback {
    /* Feedback */
    GL_2D = 0x0600,
    GL_3D = 0x0601,
    GL_3D_COLOR = 0x0602,
    GL_3D_COLOR_TEXTURE = 0x0603,
    GL_4D_COLOR_TEXTURE = 0x0604,
    GL_POINT_TOKEN = 0x0701,
    GL_LINE_TOKEN = 0x0702,
    GL_LINE_RESET_TOKEN = 0x0707,
    GL_POLYGON_TOKEN = 0x0703,
    GL_BITMAP_TOKEN = 0x0704,
    GL_DRAW_PIXEL_TOKEN = 0x0705,
    GL_COPY_PIXEL_TOKEN = 0x0706,
    GL_PASS_THROUGH_TOKEN = 0x0700,
    GL_FEEDBACK_BUFFER_POINTER = 0x0DF0,
    GL_FEEDBACK_BUFFER_SIZE = 0x0DF1,
    GL_FEEDBACK_BUFFER_TYPE = 0x0DF2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GLSelection {
    /* Selection */
    GL_SELECTION_BUFFER_POINTER = 0x0DF3,
    GL_SELECTION_BUFFER_SIZE = 0x0DF4,
}
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GLFog {
    /* Logic Ops */
    GL_LOGIC_OP = 0x0BF1,
    // GL_INDEX_LOGIC_OP = 0x0BF1,
    GL_COLOR_LOGIC_OP = 0x0BF2,
    GL_LOGIC_OP_MODE = 0x0BF0,
    GL_CLEAR = 0x1500,
    GL_SET = 0x150F,
    GL_COPY = 0x1503,
    GL_COPY_INVERTED = 0x150C,
    GL_NOOP = 0x1505,
    GL_INVERT = 0x150A,
    GL_AND = 0x1501,
    GL_NAND = 0x150E,
    GL_OR = 0x1507,
    GL_NOR = 0x1508,
    GL_XOR = 0x1506,
    GL_EQUIV = 0x1509,
    GL_AND_REVERSE = 0x1502,
    GL_AND_INVERTED = 0x1504,
    GL_OR_REVERSE = 0x150B,
    GL_OR_INVERTED = 0x150D,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GLStencil {
    /* Stencil */
    GL_STENCIL_BITS = 0x0D57,
    GL_STENCIL_TEST = 0x0B90,
    GL_STENCIL_CLEAR_VALUE = 0x0B91,
    GL_STENCIL_FUNC = 0x0B92,
    GL_STENCIL_VALUE_MASK = 0x0B93,
    GL_STENCIL_FAIL = 0x0B94,
    GL_STENCIL_PASS_DEPTH_FAIL = 0x0B95,
    GL_STENCIL_PASS_DEPTH_PASS = 0x0B96,
    GL_STENCIL_REF = 0x0B97,
    GL_STENCIL_WRITEMASK = 0x0B98,
    GL_STENCIL_INDEX = 0x1901,
    GL_KEEP = 0x1E00,
    GL_REPLACE = 0x1E01,
    GL_INCR = 0x1E02,
    GL_DECR = 0x1E03,
}
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GLStencilBits {
    /* Stencil */
    GL_STENCIL_BITS = 0x0D57,
    GL_STENCIL_TEST = 0x0B90,
    GL_STENCIL_CLEAR_VALUE = 0x0B91,
    GL_STENCIL_FUNC = 0x0B92,
    GL_STENCIL_VALUE_MASK = 0x0B93,
    GL_STENCIL_FAIL = 0x0B94,
    GL_STENCIL_PASS_DEPTH_FAIL = 0x0B95,
    GL_STENCIL_PASS_DEPTH_PASS = 0x0B96,
    GL_STENCIL_REF = 0x0B97,
    GL_STENCIL_WRITEMASK = 0x0B98,
    GL_STENCIL_INDEX = 0x1901,
    GL_KEEP = 0x1E00,
    GL_REPLACE = 0x1E01,
    GL_INCR = 0x1E02,
    GL_DECR = 0x1E03,
}
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GLBufferEnum {
    /* Buffers, Pixel Drawing/Reading */
    GL_NONE = 0,
    GL_LEFT = 0x0406,
    GL_RIGHT = 0x0407,
    GL_FRONT = 0x0404,
    GL_BACK = 0x0405,
    GL_FRONT_AND_BACK = 0x0408,
    GL_FRONT_LEFT = 0x0400,
    GL_FRONT_RIGHT = 0x0401,
    GL_BACK_LEFT = 0x0402,
    GL_BACK_RIGHT = 0x0403,
    GL_AUX0 = 0x0409,
    GL_AUX1 = 0x040A,
    GL_AUX2 = 0x040B,
    GL_AUX3 = 0x040C,
    GL_COLOR_INDEX = 0x1900,
    GL_RED = 0x1903,
    GL_GREEN = 0x1904,
    GL_BLUE = 0x1905,
    GL_ALPHA = 0x1906,
    GL_LUMINANCE = 0x1909,
    GL_LUMINANCE_ALPHA = 0x190A,
    GL_ALPHA_BITS = 0x0D55,
    GL_RED_BITS = 0x0D52,
    GL_GREEN_BITS = 0x0D53,
    GL_BLUE_BITS = 0x0D54,
    GL_INDEX_BITS = 0x0D51,
    GL_SUBPIXEL_BITS = 0x0D50,
    GL_AUX_BUFFERS = 0x0C00,
    GL_READ_BUFFER = 0x0C02,
    GL_DRAW_BUFFER = 0x0C01,
    GL_DOUBLEBUFFER = 0x0C32,
    GL_STEREO = 0x0C33,
    GL_BITMAP = 0x1A00,
    GL_COLOR = 0x1800,
    GL_DEPTH = 0x1801,
    GL_STENCIL = 0x1802,
    GL_DITHER = 0x0BD0,
    GL_RGB = 0x1907,
    GL_RGBA = 0x1908,
}
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GLImplementationLimits {
    /* Implementation limits */
    GL_MAX_LIST_NESTING = 0x0B31,
    GL_MAX_EVAL_ORDER = 0x0D30,
    GL_MAX_LIGHTS = 0x0D31,
    GL_MAX_CLIP_PLANES = 0x0D32,
    GL_MAX_TEXTURE_SIZE = 0x0D33,
    GL_MAX_PIXEL_MAP_TABLE = 0x0D34,
    GL_MAX_ATTRIB_STACK_DEPTH = 0x0D35,
    GL_MAX_MODELVIEW_STACK_DEPTH = 0x0D36,
    GL_MAX_NAME_STACK_DEPTH = 0x0D37,
    GL_MAX_PROJECTION_STACK_DEPTH = 0x0D38,
    GL_MAX_TEXTURE_STACK_DEPTH = 0x0D39,
    GL_MAX_VIEWPORT_DIMS = 0x0D3A,
    GL_MAX_CLIENT_ATTRIB_STACK_DEPTH = 0x0D3B,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GLGets {
    /* Gets */
    GL_ATTRIB_STACK_DEPTH = 0x0BB0,
    GL_CLIENT_ATTRIB_STACK_DEPTH = 0x0BB1,
    GL_COLOR_CLEAR_VALUE = 0x0C22,
    GL_COLOR_WRITEMASK = 0x0C23,
    GL_CURRENT_INDEX = 0x0B01,
    GL_CURRENT_COLOR = 0x0B00,
    GL_CURRENT_NORMAL = 0x0B02,
    GL_CURRENT_RASTER_COLOR = 0x0B04,
    GL_CURRENT_RASTER_DISTANCE = 0x0B09,
    GL_CURRENT_RASTER_INDEX = 0x0B05,
    GL_CURRENT_RASTER_POSITION = 0x0B07,
    GL_CURRENT_RASTER_TEXTURE_COORDS = 0x0B06,
    GL_CURRENT_RASTER_POSITION_VALID = 0x0B08,
    GL_CURRENT_TEXTURE_COORDS = 0x0B03,
    GL_INDEX_CLEAR_VALUE = 0x0C20,
    GL_INDEX_MODE = 0x0C30,
    GL_INDEX_WRITEMASK = 0x0C21,
    GL_MODELVIEW_MATRIX = 0x0BA6,
    GL_MODELVIEW_STACK_DEPTH = 0x0BA3,
    GL_NAME_STACK_DEPTH = 0x0D70,
    GL_PROJECTION_MATRIX = 0x0BA7,
    GL_PROJECTION_STACK_DEPTH = 0x0BA4,
    GL_RENDER_MODE = 0x0C40,
    GL_RGBA_MODE = 0x0C31,
    GL_TEXTURE_MATRIX = 0x0BA8,
    GL_TEXTURE_STACK_DEPTH = 0x0BA5,
    GL_VIEWPORT = 0x0BA2,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
/// Evaluators
pub enum GLEvaluators {
    GL_AUTO_NORMAL = 0x0D80,
    GL_MAP1_COLOR_4 = 0x0D90,
    GL_MAP1_INDEX = 0x0D91,
    GL_MAP1_NORMAL = 0x0D92,
    GL_MAP1_TEXTURE_COORD_1 = 0x0D93,
    GL_MAP1_TEXTURE_COORD_2 = 0x0D94,
    GL_MAP1_TEXTURE_COORD_3 = 0x0D95,
    GL_MAP1_TEXTURE_COORD_4 = 0x0D96,
    GL_MAP1_VERTEX_3 = 0x0D97,
    GL_MAP1_VERTEX_4 = 0x0D98,
    GL_MAP2_COLOR_4 = 0x0DB0,
    GL_MAP2_INDEX = 0x0DB1,
    GL_MAP2_NORMAL = 0x0DB2,
    GL_MAP2_TEXTURE_COORD_1 = 0x0DB3,
    GL_MAP2_TEXTURE_COORD_2 = 0x0DB4,
    GL_MAP2_TEXTURE_COORD_3 = 0x0DB5,
    GL_MAP2_TEXTURE_COORD_4 = 0x0DB6,
    GL_MAP2_VERTEX_3 = 0x0DB7,
    GL_MAP2_VERTEX_4 = 0x0DB8,
    GL_MAP1_GRID_DOMAIN = 0x0DD0,
    GL_MAP1_GRID_SEGMENTS = 0x0DD1,
    GL_MAP2_GRID_DOMAIN = 0x0DD2,
    GL_MAP2_GRID_SEGMENTS = 0x0DD3,
    GL_COEFF = 0x0A00,
    GL_ORDER = 0x0A01,
    GL_DOMAIN = 0x0A02,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
/// Hints?
pub enum GLHints {
    GL_PERSPECTIVE_CORRECTION_HINT = 0x0C50,
    GL_POINT_SMOOTH_HINT = 0x0C51,
    GL_LINE_SMOOTH_HINT = 0x0C52,
    GL_POLYGON_SMOOTH_HINT = 0x0C53,
    GL_FOG_HINT = 0x0C54,
    GL_DONT_CARE = 0x1100,
    GL_FASTEST = 0x1101,
    GL_NICEST = 0x1102,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum GLScissorBox {
    GL_SCISSOR_BOX = 0x0C10,
    GL_SCISSOR_TEST = 0x0C11,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
/// enum for pixel mode / transfer properties ?
pub enum GLPixelModeEnum {
    GL_MAP_COLOR = 0x0D10,
    GL_MAP_STENCIL = 0x0D11,
    GL_INDEX_SHIFT = 0x0D12,
    GL_INDEX_OFFSET = 0x0D13,
    GL_RED_SCALE = 0x0D14,
    GL_RED_BIAS = 0x0D15,
    GL_GREEN_SCALE = 0x0D18,
    GL_GREEN_BIAS = 0x0D19,
    GL_BLUE_SCALE = 0x0D1A,
    GL_BLUE_BIAS = 0x0D1B,
    GL_ALPHA_SCALE = 0x0D1C,
    GL_ALPHA_BIAS = 0x0D1D,
    GL_DEPTH_SCALE = 0x0D1E,
    GL_DEPTH_BIAS = 0x0D1F,
    GL_PIXEL_MAP_S_TO_S_SIZE = 0x0CB1,
    GL_PIXEL_MAP_I_TO_I_SIZE = 0x0CB0,
    GL_PIXEL_MAP_I_TO_R_SIZE = 0x0CB2,
    GL_PIXEL_MAP_I_TO_G_SIZE = 0x0CB3,
    GL_PIXEL_MAP_I_TO_B_SIZE = 0x0CB4,
    GL_PIXEL_MAP_I_TO_A_SIZE = 0x0CB5,
    GL_PIXEL_MAP_R_TO_R_SIZE = 0x0CB6,
    GL_PIXEL_MAP_G_TO_G_SIZE = 0x0CB7,
    GL_PIXEL_MAP_B_TO_B_SIZE = 0x0CB8,
    GL_PIXEL_MAP_A_TO_A_SIZE = 0x0CB9,
    GL_PIXEL_MAP_S_TO_S = 0x0C71,
    GL_PIXEL_MAP_I_TO_I = 0x0C70,
    GL_PIXEL_MAP_I_TO_R = 0x0C72,
    GL_PIXEL_MAP_I_TO_G = 0x0C73,
    GL_PIXEL_MAP_I_TO_B = 0x0C74,
    GL_PIXEL_MAP_I_TO_A = 0x0C75,
    GL_PIXEL_MAP_R_TO_R = 0x0C76,
    GL_PIXEL_MAP_G_TO_G = 0x0C77,
    GL_PIXEL_MAP_B_TO_B = 0x0C78,
    GL_PIXEL_MAP_A_TO_A = 0x0C79,
    GL_PACK_ALIGNMENT = 0x0D05,
    GL_PACK_LSB_FIRST = 0x0D01,
    GL_PACK_ROW_LENGTH = 0x0D02,
    GL_PACK_SKIP_PIXELS = 0x0D04,
    GL_PACK_SKIP_ROWS = 0x0D03,
    GL_PACK_SWAP_BYTES = 0x0D00,
    GL_UNPACK_ALIGNMENT = 0x0CF5,
    GL_UNPACK_LSB_FIRST = 0x0CF1,
    GL_UNPACK_ROW_LENGTH = 0x0CF2,
    GL_UNPACK_SKIP_PIXELS = 0x0CF4,
    GL_UNPACK_SKIP_ROWS = 0x0CF3,
    GL_UNPACK_SWAP_BYTES = 0x0CF0,
    GL_ZOOM_X = 0x0D16,
    GL_ZOOM_Y = 0x0D17,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
/// GLEnum for texture properties ?
pub enum GLTextureEnum {
    /* Texture mapping */
    GL_TEXTURE_ENV = 0x2300,
    GL_TEXTURE_ENV_MODE = 0x2200,
    GL_TEXTURE_1D = 0x0DE0,
    GL_TEXTURE_2D = 0x0DE1,
    GL_TEXTURE_WRAP_S = 0x2802,
    GL_TEXTURE_WRAP_T = 0x2803,
    GL_TEXTURE_MAG_FILTER = 0x2800,
    GL_TEXTURE_MIN_FILTER = 0x2801,
    GL_TEXTURE_ENV_COLOR = 0x2201,
    GL_TEXTURE_GEN_S = 0x0C60,
    GL_TEXTURE_GEN_T = 0x0C61,
    GL_TEXTURE_GEN_R = 0x0C62,
    GL_TEXTURE_GEN_Q = 0x0C63,
    GL_TEXTURE_GEN_MODE = 0x2500,
    GL_TEXTURE_BORDER_COLOR = 0x1004,
    GL_TEXTURE_WIDTH = 0x1000,
    GL_TEXTURE_HEIGHT = 0x1001,
    GL_TEXTURE_BORDER = 0x1005,
    GL_TEXTURE_COMPONENTS = 0x1003,
    GL_TEXTURE_RED_SIZE = 0x805C,
    GL_TEXTURE_GREEN_SIZE = 0x805D,
    GL_TEXTURE_BLUE_SIZE = 0x805E,
    GL_TEXTURE_ALPHA_SIZE = 0x805F,
    GL_TEXTURE_LUMINANCE_SIZE = 0x8060,
    GL_TEXTURE_INTENSITY_SIZE = 0x8061,
    GL_NEAREST_MIPMAP_NEAREST = 0x2700,
    GL_NEAREST_MIPMAP_LINEAR = 0x2702,
    GL_LINEAR_MIPMAP_NEAREST = 0x2701,
    GL_LINEAR_MIPMAP_LINEAR = 0x2703,
    GL_OBJECT_LINEAR = 0x2401,
    GL_OBJECT_PLANE = 0x2501,
    GL_EYE_LINEAR = 0x2400,
    GL_EYE_PLANE = 0x2502,
    GL_SPHERE_MAP = 0x2402,
    GL_DECAL = 0x2101,
    GL_MODULATE = 0x2100,
    GL_NEAREST = 0x2600,
    GL_REPEAT = 0x2901,
    GL_CLAMP = 0x2900,
    GL_S = 0x2000,
    GL_T = 0x2001,
    GL_R = 0x2002,
    GL_Q = 0x2003,
}

/// opengl 1.1 ?
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum GL11Enum {
    GL_PROXY_TEXTURE_1D = 0x8063,
    GL_PROXY_TEXTURE_2D = 0x8064,
    GL_TEXTURE_PRIORITY = 0x8066,
    GL_TEXTURE_RESIDENT = 0x8067,
    GL_TEXTURE_BINDING_1D = 0x8068,
    GL_TEXTURE_BINDING_2D = 0x8069,
    GL_TEXTURE_INTERNAL_FORMAT = 0x1003,
    GL_ALPHA4 = 0x803B,
    GL_ALPHA8 = 0x803C,
    GL_ALPHA12 = 0x803D,
    GL_ALPHA16 = 0x803E,
    GL_LUMINANCE4 = 0x803F,
    GL_LUMINANCE8 = 0x8040,
    GL_LUMINANCE12 = 0x8041,
    GL_LUMINANCE16 = 0x8042,
    GL_LUMINANCE4_ALPHA4 = 0x8043,
    GL_LUMINANCE6_ALPHA2 = 0x8044,
    GL_LUMINANCE8_ALPHA8 = 0x8045,
    GL_LUMINANCE12_ALPHA4 = 0x8046,
    GL_LUMINANCE12_ALPHA12 = 0x8047,
    GL_LUMINANCE16_ALPHA16 = 0x8048,
    GL_INTENSITY = 0x8049,
    GL_INTENSITY4 = 0x804A,
    GL_INTENSITY8 = 0x804B,
    GL_INTENSITY12 = 0x804C,
    GL_INTENSITY16 = 0x804D,
    GL_R3_G3_B2 = 0x2A10,
    GL_RGB4 = 0x804F,
    GL_RGB5 = 0x8050,
    GL_RGB8 = 0x8051,
    GL_RGB10 = 0x8052,
    GL_RGB12 = 0x8053,
    GL_RGB16 = 0x8054,
    GL_RGBA2 = 0x8055,
    GL_RGBA4 = 0x8056,
    GL_RGB5_A1 = 0x8057,
    GL_RGBA8 = 0x8058,
    GL_RGB10_A2 = 0x8059,
    GL_RGBA12 = 0x805A,
    GL_RGBA16 = 0x805B,
}
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]

///Errors
pub enum GLError {
    GL_NO_ERROR = 0,
    GL_INVALID_ENUM = 0x0500,
    GL_INVALID_VALUE = 0x0501,
    GL_INVALID_OPERATION = 0x0502,
    GL_STACK_OVERFLOW = 0x0503,
    GL_STACK_UNDERFLOW = 0x0504,
    GL_OUT_OF_MEMORY = 0x0505,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
/// I believe these to be GLenum values for saying what type something is
pub enum GLtype {
    GL_BYTE = 0x1400,
    GL_UNSIGNED_BYTE = 0x1401,
    GL_SHORT = 0x1402,
    GL_UNSIGNED_SHORT = 0x1403,
    GL_INT = 0x1404,
    GL_UNSIGNED_INT = 0x1405,
    GL_FLOAT = 0x1406,
    GL_2_BYTES = 0x1407,
    GL_3_BYTES = 0x1408,
    GL_4_BYTES = 0x1409,
    GL_DOUBLE = 0x140A,
}

type GLenum = std::ffi::c_uint;

#[repr(transparent)]
#[derive(Debug)]
pub struct GLvoid(std::ffi::c_void);

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct GLbyte(std::ffi::c_schar);

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct GLshort(std::ffi::c_short);

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct GLint(std::ffi::c_uint);

// #[repr(transparent)]
// #[derive(Clone, Copy, Debug)]
// pub struct GLubyte(std::ffi::c_char);
// impl GLubyte {
//     pub fn const_ptr_to_string(ptr: *const Self) -> Result<String,()> {
//         let cstr = unsafe {std::ffi::CStr::from_ptr(ptr.0)};
//     }
// }

type GLubyte = std::ffi::c_char;

// a wrapper type around a *const GLubyte for type saftey
// allows me to implement string handling in one place (hopefully)
#[repr(transparent)]
pub struct ConstGLubytePointer(*const GLubyte);
impl ConstGLubytePointer {
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }
    pub fn is_zero(&self) -> bool {
        if self.is_null() {
            return true;
        }
        (unsafe { *self.0 } == 0)
    }

    pub fn to_string_lossy(&self) -> Result<String, String> {
        if self.is_null() {
            return Err("Null pointer from GLubyte".to_string());
        }
        // assuming that UTF-8 is used.
        let cstr = unsafe { std::ffi::CStr::from_ptr(self.0) };
        let string = cstr.to_string_lossy().to_string();
        Ok(string)
    }
    pub fn try_into_string(&self) -> Result<String, String> {
        if self.is_null() {
            return Err("Null pointer from GLubyte".to_string());
        }
        // assuming that UTF-8 is used.
        let cstr = unsafe { std::ffi::CStr::from_ptr(self.0) };
        let str = cstr.to_str().map_err(|e| e.to_string())?;
        Ok(str.to_string())
    }
    pub fn try_into_str(&self) -> Result<&'_ str, std::str::Utf8Error> {
        let cstr = unsafe { std::ffi::CStr::from_ptr(self.0) };
        let str = cstr.to_str()?;
        Ok(str)
    }
}

#[repr(transparent)]
pub struct MutableGLubytePointer(*mut GLubyte);

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct GLushort(std::ffi::c_ushort);

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct GLuint(std::ffi::c_uint);

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct GLsizei(std::ffi::c_int);

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct GLfloat(std::ffi::c_float);

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct GLclampf(std::ffi::c_float);

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct GLdouble(std::ffi::c_double);

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct GLclampd(std::ffi::c_double);

// constants

/* Boolean values */
const GL_FALSE: std::ffi::c_uint = 0;
const GL_TRUE: std::ffi::c_uint = 1;
/* Data types */
const GL_BYTE: std::ffi::c_uint = 0x1400;
const GL_UNSIGNED_BYTE: std::ffi::c_uint = 0x1401;
const GL_SHORT: std::ffi::c_uint = 0x1402;
const GL_UNSIGNED_SHORT: std::ffi::c_uint = 0x1403;
const GL_INT: std::ffi::c_uint = 0x1404;
const GL_UNSIGNED_INT: std::ffi::c_uint = 0x1405;
const GL_FLOAT: std::ffi::c_uint = 0x1406;
const GL_2_BYTES: std::ffi::c_uint = 0x1407;
const GL_3_BYTES: std::ffi::c_uint = 0x1408;
const GL_4_BYTES: std::ffi::c_uint = 0x1409;
const GL_DOUBLE: std::ffi::c_uint = 0x140A;
/* Primitives */
const GL_POINTS: std::ffi::c_uint = 0x0000;
const GL_LINES: std::ffi::c_uint = 0x0001;
const GL_LINE_LOOP: std::ffi::c_uint = 0x0002;
const GL_LINE_STRIP: std::ffi::c_uint = 0x0003;
const GL_TRIANGLES: std::ffi::c_uint = 0x0004;
const GL_TRIANGLE_STRIP: std::ffi::c_uint = 0x0005;
const GL_TRIANGLE_FAN: std::ffi::c_uint = 0x0006;
const GL_QUADS: std::ffi::c_uint = 0x0007;
const GL_QUAD_STRIP: std::ffi::c_uint = 0x0008;
const GL_POLYGON: std::ffi::c_uint = 0x0009;
/* Vertex Arrays */
const GL_VERTEX_ARRAY: std::ffi::c_uint = 0x8074;
const GL_NORMAL_ARRAY: std::ffi::c_uint = 0x8075;
const GL_COLOR_ARRAY: std::ffi::c_uint = 0x8076;
const GL_INDEX_ARRAY: std::ffi::c_uint = 0x8077;
const GL_TEXTURE_COORD_ARRAY: std::ffi::c_uint = 0x8078;
const GL_EDGE_FLAG_ARRAY: std::ffi::c_uint = 0x8079;
const GL_VERTEX_ARRAY_SIZE: std::ffi::c_uint = 0x807A;
const GL_VERTEX_ARRAY_TYPE: std::ffi::c_uint = 0x807B;
const GL_VERTEX_ARRAY_STRIDE: std::ffi::c_uint = 0x807C;
const GL_NORMAL_ARRAY_TYPE: std::ffi::c_uint = 0x807E;
const GL_NORMAL_ARRAY_STRIDE: std::ffi::c_uint = 0x807F;
const GL_COLOR_ARRAY_SIZE: std::ffi::c_uint = 0x8081;
const GL_COLOR_ARRAY_TYPE: std::ffi::c_uint = 0x8082;
const GL_COLOR_ARRAY_STRIDE: std::ffi::c_uint = 0x8083;
const GL_INDEX_ARRAY_TYPE: std::ffi::c_uint = 0x8085;
const GL_INDEX_ARRAY_STRIDE: std::ffi::c_uint = 0x8086;
const GL_TEXTURE_COORD_ARRAY_SIZE: std::ffi::c_uint = 0x8088;
const GL_TEXTURE_COORD_ARRAY_TYPE: std::ffi::c_uint = 0x8089;
const GL_TEXTURE_COORD_ARRAY_STRIDE: std::ffi::c_uint = 0x808A;
const GL_EDGE_FLAG_ARRAY_STRIDE: std::ffi::c_uint = 0x808C;
const GL_VERTEX_ARRAY_POINTER: std::ffi::c_uint = 0x808E;
const GL_NORMAL_ARRAY_POINTER: std::ffi::c_uint = 0x808F;
const GL_COLOR_ARRAY_POINTER: std::ffi::c_uint = 0x8090;
const GL_INDEX_ARRAY_POINTER: std::ffi::c_uint = 0x8091;
const GL_TEXTURE_COORD_ARRAY_POINTER: std::ffi::c_uint = 0x8092;
const GL_EDGE_FLAG_ARRAY_POINTER: std::ffi::c_uint = 0x8093;
const GL_V2F: std::ffi::c_uint = 0x2A20;
const GL_V3F: std::ffi::c_uint = 0x2A21;
const GL_C4UB_V2F: std::ffi::c_uint = 0x2A22;
const GL_C4UB_V3F: std::ffi::c_uint = 0x2A23;
const GL_C3F_V3F: std::ffi::c_uint = 0x2A24;
const GL_N3F_V3F: std::ffi::c_uint = 0x2A25;
const GL_C4F_N3F_V3F: std::ffi::c_uint = 0x2A26;
const GL_T2F_V3F: std::ffi::c_uint = 0x2A27;
const GL_T4F_V4F: std::ffi::c_uint = 0x2A28;
const GL_T2F_C4UB_V3F: std::ffi::c_uint = 0x2A29;
const GL_T2F_C3F_V3F: std::ffi::c_uint = 0x2A2A;
const GL_T2F_N3F_V3F: std::ffi::c_uint = 0x2A2B;
const GL_T2F_C4F_N3F_V3F: std::ffi::c_uint = 0x2A2C;
const GL_T4F_C4F_N3F_V4F: std::ffi::c_uint = 0x2A2D;
/* Matrix Mode */
const GL_MATRIX_MODE: std::ffi::c_uint = 0x0BA0;
const GL_MODELVIEW: std::ffi::c_uint = 0x1700;
const GL_PROJECTION: std::ffi::c_uint = 0x1701;
const GL_TEXTURE: std::ffi::c_uint = 0x1702;
/* Points */
const GL_POINT_SMOOTH: std::ffi::c_uint = 0x0B10;
const GL_POINT_SIZE: std::ffi::c_uint = 0x0B11;
const GL_POINT_SIZE_GRANULARITY: std::ffi::c_uint = 0x0B13;
const GL_POINT_SIZE_RANGE: std::ffi::c_uint = 0x0B12;
/* Lines */
const GL_LINE_SMOOTH: std::ffi::c_uint = 0x0B20;
const GL_LINE_STIPPLE: std::ffi::c_uint = 0x0B24;
const GL_LINE_STIPPLE_PATTERN: std::ffi::c_uint = 0x0B25;
const GL_LINE_STIPPLE_REPEAT: std::ffi::c_uint = 0x0B26;
const GL_LINE_WIDTH: std::ffi::c_uint = 0x0B21;
const GL_LINE_WIDTH_GRANULARITY: std::ffi::c_uint = 0x0B23;
const GL_LINE_WIDTH_RANGE: std::ffi::c_uint = 0x0B22;
/* Polygons */
const GL_POINT: std::ffi::c_uint = 0x1B00;
const GL_LINE: std::ffi::c_uint = 0x1B01;
const GL_FILL: std::ffi::c_uint = 0x1B02;
const GL_CW: std::ffi::c_uint = 0x0900;
const GL_CCW: std::ffi::c_uint = 0x0901;
const GL_FRONT: std::ffi::c_uint = 0x0404;
const GL_BACK: std::ffi::c_uint = 0x0405;
const GL_POLYGON_MODE: std::ffi::c_uint = 0x0B40;
const GL_POLYGON_SMOOTH: std::ffi::c_uint = 0x0B41;
const GL_POLYGON_STIPPLE: std::ffi::c_uint = 0x0B42;
const GL_EDGE_FLAG: std::ffi::c_uint = 0x0B43;
const GL_CULL_FACE: std::ffi::c_uint = 0x0B44;
const GL_CULL_FACE_MODE: std::ffi::c_uint = 0x0B45;
const GL_FRONT_FACE: std::ffi::c_uint = 0x0B46;
const GL_POLYGON_OFFSET_FACTOR: std::ffi::c_uint = 0x8038;
const GL_POLYGON_OFFSET_UNITS: std::ffi::c_uint = 0x2A00;
const GL_POLYGON_OFFSET_POINT: std::ffi::c_uint = 0x2A01;
const GL_POLYGON_OFFSET_LINE: std::ffi::c_uint = 0x2A02;
const GL_POLYGON_OFFSET_FILL: std::ffi::c_uint = 0x8037;
/* Display Lists */
const GL_COMPILE: std::ffi::c_uint = 0x1300;
const GL_COMPILE_AND_EXECUTE: std::ffi::c_uint = 0x1301;
const GL_LIST_BASE: std::ffi::c_uint = 0x0B32;
const GL_LIST_INDEX: std::ffi::c_uint = 0x0B33;
const GL_LIST_MODE: std::ffi::c_uint = 0x0B30;
/* Depth buffer */
const GL_NEVER: std::ffi::c_uint = 0x0200;
const GL_LESS: std::ffi::c_uint = 0x0201;
const GL_EQUAL: std::ffi::c_uint = 0x0202;
const GL_LEQUAL: std::ffi::c_uint = 0x0203;
const GL_GREATER: std::ffi::c_uint = 0x0204;
const GL_NOTEQUAL: std::ffi::c_uint = 0x0205;
const GL_GEQUAL: std::ffi::c_uint = 0x0206;
const GL_ALWAYS: std::ffi::c_uint = 0x0207;
const GL_DEPTH_TEST: std::ffi::c_uint = 0x0B71;
const GL_DEPTH_BITS: std::ffi::c_uint = 0x0D56;
const GL_DEPTH_CLEAR_VALUE: std::ffi::c_uint = 0x0B73;
const GL_DEPTH_FUNC: std::ffi::c_uint = 0x0B74;
const GL_DEPTH_RANGE: std::ffi::c_uint = 0x0B70;
const GL_DEPTH_WRITEMASK: std::ffi::c_uint = 0x0B72;
const GL_DEPTH_COMPONENT: std::ffi::c_uint = 0x1902;
/* Lighting */
const GL_LIGHTING: std::ffi::c_uint = 0x0B50;
const GL_LIGHT0: std::ffi::c_uint = 0x4000;
const GL_LIGHT1: std::ffi::c_uint = 0x4001;
const GL_LIGHT2: std::ffi::c_uint = 0x4002;
const GL_LIGHT3: std::ffi::c_uint = 0x4003;
const GL_LIGHT4: std::ffi::c_uint = 0x4004;
const GL_LIGHT5: std::ffi::c_uint = 0x4005;
const GL_LIGHT6: std::ffi::c_uint = 0x4006;
const GL_LIGHT7: std::ffi::c_uint = 0x4007;
const GL_SPOT_EXPONENT: std::ffi::c_uint = 0x1205;
const GL_SPOT_CUTOFF: std::ffi::c_uint = 0x1206;
const GL_CONSTANT_ATTENUATION: std::ffi::c_uint = 0x1207;
const GL_LINEAR_ATTENUATION: std::ffi::c_uint = 0x1208;
const GL_QUADRATIC_ATTENUATION: std::ffi::c_uint = 0x1209;
const GL_AMBIENT: std::ffi::c_uint = 0x1200;
const GL_DIFFUSE: std::ffi::c_uint = 0x1201;
const GL_SPECULAR: std::ffi::c_uint = 0x1202;
const GL_SHININESS: std::ffi::c_uint = 0x1601;
const GL_EMISSION: std::ffi::c_uint = 0x1600;
const GL_POSITION: std::ffi::c_uint = 0x1203;
const GL_SPOT_DIRECTION: std::ffi::c_uint = 0x1204;
const GL_AMBIENT_AND_DIFFUSE: std::ffi::c_uint = 0x1602;
const GL_COLOR_INDEXES: std::ffi::c_uint = 0x1603;
const GL_LIGHT_MODEL_TWO_SIDE: std::ffi::c_uint = 0x0B52;
const GL_LIGHT_MODEL_LOCAL_VIEWER: std::ffi::c_uint = 0x0B51;
const GL_LIGHT_MODEL_AMBIENT: std::ffi::c_uint = 0x0B53;
const GL_FRONT_AND_BACK: std::ffi::c_uint = 0x0408;
const GL_SHADE_MODEL: std::ffi::c_uint = 0x0B54;
const GL_FLAT: std::ffi::c_uint = 0x1D00;
const GL_SMOOTH: std::ffi::c_uint = 0x1D01;
const GL_COLOR_MATERIAL: std::ffi::c_uint = 0x0B57;
const GL_COLOR_MATERIAL_FACE: std::ffi::c_uint = 0x0B55;
const GL_COLOR_MATERIAL_PARAMETER: std::ffi::c_uint = 0x0B56;
const GL_NORMALIZE: std::ffi::c_uint = 0x0BA1;
/* User clipping planes */
const GL_CLIP_PLANE0: std::ffi::c_uint = 0x3000;
const GL_CLIP_PLANE1: std::ffi::c_uint = 0x3001;
const GL_CLIP_PLANE2: std::ffi::c_uint = 0x3002;
const GL_CLIP_PLANE3: std::ffi::c_uint = 0x3003;
const GL_CLIP_PLANE4: std::ffi::c_uint = 0x3004;
const GL_CLIP_PLANE5: std::ffi::c_uint = 0x3005;
/* Accumulation buffer */
const GL_ACCUM_RED_BITS: std::ffi::c_uint = 0x0D58;
const GL_ACCUM_GREEN_BITS: std::ffi::c_uint = 0x0D59;
const GL_ACCUM_BLUE_BITS: std::ffi::c_uint = 0x0D5A;
const GL_ACCUM_ALPHA_BITS: std::ffi::c_uint = 0x0D5B;
const GL_ACCUM_CLEAR_VALUE: std::ffi::c_uint = 0x0B80;
const GL_ACCUM: std::ffi::c_uint = 0x0100;
const GL_ADD: std::ffi::c_uint = 0x0104;
const GL_LOAD: std::ffi::c_uint = 0x0101;
const GL_MULT: std::ffi::c_uint = 0x0103;
const GL_RETURN: std::ffi::c_uint = 0x0102;
/* Alpha testing */
const GL_ALPHA_TEST: std::ffi::c_uint = 0x0BC0;
const GL_ALPHA_TEST_REF: std::ffi::c_uint = 0x0BC2;
const GL_ALPHA_TEST_FUNC: std::ffi::c_uint = 0x0BC1;
/* Blending */
const GL_BLEND: std::ffi::c_uint = 0x0BE2;
const GL_BLEND_SRC: std::ffi::c_uint = 0x0BE1;
const GL_BLEND_DST: std::ffi::c_uint = 0x0BE0;
const GL_ZERO: std::ffi::c_uint = 0;
const GL_ONE: std::ffi::c_uint = 1;
const GL_SRC_COLOR: std::ffi::c_uint = 0x0300;
const GL_ONE_MINUS_SRC_COLOR: std::ffi::c_uint = 0x0301;
const GL_SRC_ALPHA: std::ffi::c_uint = 0x0302;
const GL_ONE_MINUS_SRC_ALPHA: std::ffi::c_uint = 0x0303;
const GL_DST_ALPHA: std::ffi::c_uint = 0x0304;
const GL_ONE_MINUS_DST_ALPHA: std::ffi::c_uint = 0x0305;
const GL_DST_COLOR: std::ffi::c_uint = 0x0306;
const GL_ONE_MINUS_DST_COLOR: std::ffi::c_uint = 0x0307;
const GL_SRC_ALPHA_SATURATE: std::ffi::c_uint = 0x0308;
/* Render Mode */
const GL_FEEDBACK: std::ffi::c_uint = 0x1C01;
const GL_RENDER: std::ffi::c_uint = 0x1C00;
const GL_SELECT: std::ffi::c_uint = 0x1C02;
/* Feedback */
const GL_2D: std::ffi::c_uint = 0x0600;
const GL_3D: std::ffi::c_uint = 0x0601;
const GL_3D_COLOR: std::ffi::c_uint = 0x0602;
const GL_3D_COLOR_TEXTURE: std::ffi::c_uint = 0x0603;
const GL_4D_COLOR_TEXTURE: std::ffi::c_uint = 0x0604;
const GL_POINT_TOKEN: std::ffi::c_uint = 0x0701;
const GL_LINE_TOKEN: std::ffi::c_uint = 0x0702;
const GL_LINE_RESET_TOKEN: std::ffi::c_uint = 0x0707;
const GL_POLYGON_TOKEN: std::ffi::c_uint = 0x0703;
const GL_BITMAP_TOKEN: std::ffi::c_uint = 0x0704;
const GL_DRAW_PIXEL_TOKEN: std::ffi::c_uint = 0x0705;
const GL_COPY_PIXEL_TOKEN: std::ffi::c_uint = 0x0706;
const GL_PASS_THROUGH_TOKEN: std::ffi::c_uint = 0x0700;
const GL_FEEDBACK_BUFFER_POINTER: std::ffi::c_uint = 0x0DF0;
const GL_FEEDBACK_BUFFER_SIZE: std::ffi::c_uint = 0x0DF1;
const GL_FEEDBACK_BUFFER_TYPE: std::ffi::c_uint = 0x0DF2;
/* Selection */
const GL_SELECTION_BUFFER_POINTER: std::ffi::c_uint = 0x0DF3;
const GL_SELECTION_BUFFER_SIZE: std::ffi::c_uint = 0x0DF4;
/* Fog */
const GL_FOG: std::ffi::c_uint = 0x0B60;
const GL_FOG_MODE: std::ffi::c_uint = 0x0B65;
const GL_FOG_DENSITY: std::ffi::c_uint = 0x0B62;
const GL_FOG_COLOR: std::ffi::c_uint = 0x0B66;
const GL_FOG_INDEX: std::ffi::c_uint = 0x0B61;
const GL_FOG_START: std::ffi::c_uint = 0x0B63;
const GL_FOG_END: std::ffi::c_uint = 0x0B64;
const GL_LINEAR: std::ffi::c_uint = 0x2601;
const GL_EXP: std::ffi::c_uint = 0x0800;
const GL_EXP2: std::ffi::c_uint = 0x0801;
/* Logic Ops */
const GL_LOGIC_OP: std::ffi::c_uint = 0x0BF1;
const GL_INDEX_LOGIC_OP: std::ffi::c_uint = 0x0BF1;
const GL_COLOR_LOGIC_OP: std::ffi::c_uint = 0x0BF2;
const GL_LOGIC_OP_MODE: std::ffi::c_uint = 0x0BF0;
const GL_CLEAR: std::ffi::c_uint = 0x1500;
const GL_SET: std::ffi::c_uint = 0x150F;
const GL_COPY: std::ffi::c_uint = 0x1503;
const GL_COPY_INVERTED: std::ffi::c_uint = 0x150C;
const GL_NOOP: std::ffi::c_uint = 0x1505;
const GL_INVERT: std::ffi::c_uint = 0x150A;
const GL_AND: std::ffi::c_uint = 0x1501;
const GL_NAND: std::ffi::c_uint = 0x150E;
const GL_OR: std::ffi::c_uint = 0x1507;
const GL_NOR: std::ffi::c_uint = 0x1508;
const GL_XOR: std::ffi::c_uint = 0x1506;
const GL_EQUIV: std::ffi::c_uint = 0x1509;
const GL_AND_REVERSE: std::ffi::c_uint = 0x1502;
const GL_AND_INVERTED: std::ffi::c_uint = 0x1504;
const GL_OR_REVERSE: std::ffi::c_uint = 0x150B;
const GL_OR_INVERTED: std::ffi::c_uint = 0x150D;
/* Stencil */
const GL_STENCIL_BITS: std::ffi::c_uint = 0x0D57;
const GL_STENCIL_TEST: std::ffi::c_uint = 0x0B90;
const GL_STENCIL_CLEAR_VALUE: std::ffi::c_uint = 0x0B91;
const GL_STENCIL_FUNC: std::ffi::c_uint = 0x0B92;
const GL_STENCIL_VALUE_MASK: std::ffi::c_uint = 0x0B93;
const GL_STENCIL_FAIL: std::ffi::c_uint = 0x0B94;
const GL_STENCIL_PASS_DEPTH_FAIL: std::ffi::c_uint = 0x0B95;
const GL_STENCIL_PASS_DEPTH_PASS: std::ffi::c_uint = 0x0B96;
const GL_STENCIL_REF: std::ffi::c_uint = 0x0B97;
const GL_STENCIL_WRITEMASK: std::ffi::c_uint = 0x0B98;
const GL_STENCIL_INDEX: std::ffi::c_uint = 0x1901;
const GL_KEEP: std::ffi::c_uint = 0x1E00;
const GL_REPLACE: std::ffi::c_uint = 0x1E01;
const GL_INCR: std::ffi::c_uint = 0x1E02;
const GL_DECR: std::ffi::c_uint = 0x1E03;
/* Buffers, Pixel Drawing/Reading */
const GL_NONE: std::ffi::c_uint = 0;
const GL_LEFT: std::ffi::c_uint = 0x0406;
const GL_RIGHT: std::ffi::c_uint = 0x0407;
/*GL_FRONT					0x0404 */
/*GL_BACK					0x0405 */
/*GL_FRONT_AND_BACK				0x0408 */
const GL_FRONT_LEFT: std::ffi::c_uint = 0x0400;
const GL_FRONT_RIGHT: std::ffi::c_uint = 0x0401;
const GL_BACK_LEFT: std::ffi::c_uint = 0x0402;
const GL_BACK_RIGHT: std::ffi::c_uint = 0x0403;
const GL_AUX0: std::ffi::c_uint = 0x0409;
const GL_AUX1: std::ffi::c_uint = 0x040A;
const GL_AUX2: std::ffi::c_uint = 0x040B;
const GL_AUX3: std::ffi::c_uint = 0x040C;
const GL_COLOR_INDEX: std::ffi::c_uint = 0x1900;
const GL_RED: std::ffi::c_uint = 0x1903;
const GL_GREEN: std::ffi::c_uint = 0x1904;
const GL_BLUE: std::ffi::c_uint = 0x1905;
const GL_ALPHA: std::ffi::c_uint = 0x1906;
const GL_LUMINANCE: std::ffi::c_uint = 0x1909;
const GL_LUMINANCE_ALPHA: std::ffi::c_uint = 0x190A;
const GL_ALPHA_BITS: std::ffi::c_uint = 0x0D55;
const GL_RED_BITS: std::ffi::c_uint = 0x0D52;
const GL_GREEN_BITS: std::ffi::c_uint = 0x0D53;
const GL_BLUE_BITS: std::ffi::c_uint = 0x0D54;
const GL_INDEX_BITS: std::ffi::c_uint = 0x0D51;
const GL_SUBPIXEL_BITS: std::ffi::c_uint = 0x0D50;
const GL_AUX_BUFFERS: std::ffi::c_uint = 0x0C00;
const GL_READ_BUFFER: std::ffi::c_uint = 0x0C02;
const GL_DRAW_BUFFER: std::ffi::c_uint = 0x0C01;
const GL_DOUBLEBUFFER: std::ffi::c_uint = 0x0C32;
const GL_STEREO: std::ffi::c_uint = 0x0C33;
const GL_BITMAP: std::ffi::c_uint = 0x1A00;
const GL_COLOR: std::ffi::c_uint = 0x1800;
const GL_DEPTH: std::ffi::c_uint = 0x1801;
const GL_STENCIL: std::ffi::c_uint = 0x1802;
const GL_DITHER: std::ffi::c_uint = 0x0BD0;
const GL_RGB: std::ffi::c_uint = 0x1907;
const GL_RGBA: std::ffi::c_uint = 0x1908;
/* Implementation limits */
const GL_MAX_LIST_NESTING: std::ffi::c_uint = 0x0B31;
const GL_MAX_EVAL_ORDER: std::ffi::c_uint = 0x0D30;
const GL_MAX_LIGHTS: std::ffi::c_uint = 0x0D31;
const GL_MAX_CLIP_PLANES: std::ffi::c_uint = 0x0D32;
const GL_MAX_TEXTURE_SIZE: std::ffi::c_uint = 0x0D33;
const GL_MAX_PIXEL_MAP_TABLE: std::ffi::c_uint = 0x0D34;
const GL_MAX_ATTRIB_STACK_DEPTH: std::ffi::c_uint = 0x0D35;
const GL_MAX_MODELVIEW_STACK_DEPTH: std::ffi::c_uint = 0x0D36;
const GL_MAX_NAME_STACK_DEPTH: std::ffi::c_uint = 0x0D37;
const GL_MAX_PROJECTION_STACK_DEPTH: std::ffi::c_uint = 0x0D38;
const GL_MAX_TEXTURE_STACK_DEPTH: std::ffi::c_uint = 0x0D39;
const GL_MAX_VIEWPORT_DIMS: std::ffi::c_uint = 0x0D3A;
const GL_MAX_CLIENT_ATTRIB_STACK_DEPTH: std::ffi::c_uint = 0x0D3B;
/* Gets */
const GL_ATTRIB_STACK_DEPTH: std::ffi::c_uint = 0x0BB0;
const GL_CLIENT_ATTRIB_STACK_DEPTH: std::ffi::c_uint = 0x0BB1;
const GL_COLOR_CLEAR_VALUE: std::ffi::c_uint = 0x0C22;
const GL_COLOR_WRITEMASK: std::ffi::c_uint = 0x0C23;
const GL_CURRENT_INDEX: std::ffi::c_uint = 0x0B01;
const GL_CURRENT_COLOR: std::ffi::c_uint = 0x0B00;
const GL_CURRENT_NORMAL: std::ffi::c_uint = 0x0B02;
const GL_CURRENT_RASTER_COLOR: std::ffi::c_uint = 0x0B04;
const GL_CURRENT_RASTER_DISTANCE: std::ffi::c_uint = 0x0B09;
const GL_CURRENT_RASTER_INDEX: std::ffi::c_uint = 0x0B05;
const GL_CURRENT_RASTER_POSITION: std::ffi::c_uint = 0x0B07;
const GL_CURRENT_RASTER_TEXTURE_COORDS: std::ffi::c_uint = 0x0B06;
const GL_CURRENT_RASTER_POSITION_VALID: std::ffi::c_uint = 0x0B08;
const GL_CURRENT_TEXTURE_COORDS: std::ffi::c_uint = 0x0B03;
const GL_INDEX_CLEAR_VALUE: std::ffi::c_uint = 0x0C20;
const GL_INDEX_MODE: std::ffi::c_uint = 0x0C30;
const GL_INDEX_WRITEMASK: std::ffi::c_uint = 0x0C21;
const GL_MODELVIEW_MATRIX: std::ffi::c_uint = 0x0BA6;
const GL_MODELVIEW_STACK_DEPTH: std::ffi::c_uint = 0x0BA3;
const GL_NAME_STACK_DEPTH: std::ffi::c_uint = 0x0D70;
const GL_PROJECTION_MATRIX: std::ffi::c_uint = 0x0BA7;
const GL_PROJECTION_STACK_DEPTH: std::ffi::c_uint = 0x0BA4;
const GL_RENDER_MODE: std::ffi::c_uint = 0x0C40;
const GL_RGBA_MODE: std::ffi::c_uint = 0x0C31;
const GL_TEXTURE_MATRIX: std::ffi::c_uint = 0x0BA8;
const GL_TEXTURE_STACK_DEPTH: std::ffi::c_uint = 0x0BA5;
const GL_VIEWPORT: std::ffi::c_uint = 0x0BA2;
/* Evaluators */
const GL_AUTO_NORMAL: std::ffi::c_uint = 0x0D80;
const GL_MAP1_COLOR_4: std::ffi::c_uint = 0x0D90;
const GL_MAP1_INDEX: std::ffi::c_uint = 0x0D91;
const GL_MAP1_NORMAL: std::ffi::c_uint = 0x0D92;
const GL_MAP1_TEXTURE_COORD_1: std::ffi::c_uint = 0x0D93;
const GL_MAP1_TEXTURE_COORD_2: std::ffi::c_uint = 0x0D94;
const GL_MAP1_TEXTURE_COORD_3: std::ffi::c_uint = 0x0D95;
const GL_MAP1_TEXTURE_COORD_4: std::ffi::c_uint = 0x0D96;
const GL_MAP1_VERTEX_3: std::ffi::c_uint = 0x0D97;
const GL_MAP1_VERTEX_4: std::ffi::c_uint = 0x0D98;
const GL_MAP2_COLOR_4: std::ffi::c_uint = 0x0DB0;
const GL_MAP2_INDEX: std::ffi::c_uint = 0x0DB1;
const GL_MAP2_NORMAL: std::ffi::c_uint = 0x0DB2;
const GL_MAP2_TEXTURE_COORD_1: std::ffi::c_uint = 0x0DB3;
const GL_MAP2_TEXTURE_COORD_2: std::ffi::c_uint = 0x0DB4;
const GL_MAP2_TEXTURE_COORD_3: std::ffi::c_uint = 0x0DB5;
const GL_MAP2_TEXTURE_COORD_4: std::ffi::c_uint = 0x0DB6;
const GL_MAP2_VERTEX_3: std::ffi::c_uint = 0x0DB7;
const GL_MAP2_VERTEX_4: std::ffi::c_uint = 0x0DB8;
const GL_MAP1_GRID_DOMAIN: std::ffi::c_uint = 0x0DD0;
const GL_MAP1_GRID_SEGMENTS: std::ffi::c_uint = 0x0DD1;
const GL_MAP2_GRID_DOMAIN: std::ffi::c_uint = 0x0DD2;
const GL_MAP2_GRID_SEGMENTS: std::ffi::c_uint = 0x0DD3;
const GL_COEFF: std::ffi::c_uint = 0x0A00;
const GL_ORDER: std::ffi::c_uint = 0x0A01;
const GL_DOMAIN: std::ffi::c_uint = 0x0A02;
/* Hints */
const GL_PERSPECTIVE_CORRECTION_HINT: std::ffi::c_uint = 0x0C50;
const GL_POINT_SMOOTH_HINT: std::ffi::c_uint = 0x0C51;
const GL_LINE_SMOOTH_HINT: std::ffi::c_uint = 0x0C52;
const GL_POLYGON_SMOOTH_HINT: std::ffi::c_uint = 0x0C53;
const GL_FOG_HINT: std::ffi::c_uint = 0x0C54;
const GL_DONT_CARE: std::ffi::c_uint = 0x1100;
const GL_FASTEST: std::ffi::c_uint = 0x1101;
const GL_NICEST: std::ffi::c_uint = 0x1102;
/* Scissor box */
const GL_SCISSOR_BOX: std::ffi::c_uint = 0x0C10;
const GL_SCISSOR_TEST: std::ffi::c_uint = 0x0C11;
/* Pixel Mode / Transfer */
const GL_MAP_COLOR: std::ffi::c_uint = 0x0D10;
const GL_MAP_STENCIL: std::ffi::c_uint = 0x0D11;
const GL_INDEX_SHIFT: std::ffi::c_uint = 0x0D12;
const GL_INDEX_OFFSET: std::ffi::c_uint = 0x0D13;
const GL_RED_SCALE: std::ffi::c_uint = 0x0D14;
const GL_RED_BIAS: std::ffi::c_uint = 0x0D15;
const GL_GREEN_SCALE: std::ffi::c_uint = 0x0D18;
const GL_GREEN_BIAS: std::ffi::c_uint = 0x0D19;
const GL_BLUE_SCALE: std::ffi::c_uint = 0x0D1A;
const GL_BLUE_BIAS: std::ffi::c_uint = 0x0D1B;
const GL_ALPHA_SCALE: std::ffi::c_uint = 0x0D1C;
const GL_ALPHA_BIAS: std::ffi::c_uint = 0x0D1D;
const GL_DEPTH_SCALE: std::ffi::c_uint = 0x0D1E;
const GL_DEPTH_BIAS: std::ffi::c_uint = 0x0D1F;
const GL_PIXEL_MAP_S_TO_S_SIZE: std::ffi::c_uint = 0x0CB1;
const GL_PIXEL_MAP_I_TO_I_SIZE: std::ffi::c_uint = 0x0CB0;
const GL_PIXEL_MAP_I_TO_R_SIZE: std::ffi::c_uint = 0x0CB2;
const GL_PIXEL_MAP_I_TO_G_SIZE: std::ffi::c_uint = 0x0CB3;
const GL_PIXEL_MAP_I_TO_B_SIZE: std::ffi::c_uint = 0x0CB4;
const GL_PIXEL_MAP_I_TO_A_SIZE: std::ffi::c_uint = 0x0CB5;
const GL_PIXEL_MAP_R_TO_R_SIZE: std::ffi::c_uint = 0x0CB6;
const GL_PIXEL_MAP_G_TO_G_SIZE: std::ffi::c_uint = 0x0CB7;
const GL_PIXEL_MAP_B_TO_B_SIZE: std::ffi::c_uint = 0x0CB8;
const GL_PIXEL_MAP_A_TO_A_SIZE: std::ffi::c_uint = 0x0CB9;
const GL_PIXEL_MAP_S_TO_S: std::ffi::c_uint = 0x0C71;
const GL_PIXEL_MAP_I_TO_I: std::ffi::c_uint = 0x0C70;
const GL_PIXEL_MAP_I_TO_R: std::ffi::c_uint = 0x0C72;
const GL_PIXEL_MAP_I_TO_G: std::ffi::c_uint = 0x0C73;
const GL_PIXEL_MAP_I_TO_B: std::ffi::c_uint = 0x0C74;
const GL_PIXEL_MAP_I_TO_A: std::ffi::c_uint = 0x0C75;
const GL_PIXEL_MAP_R_TO_R: std::ffi::c_uint = 0x0C76;
const GL_PIXEL_MAP_G_TO_G: std::ffi::c_uint = 0x0C77;
const GL_PIXEL_MAP_B_TO_B: std::ffi::c_uint = 0x0C78;
const GL_PIXEL_MAP_A_TO_A: std::ffi::c_uint = 0x0C79;
const GL_PACK_ALIGNMENT: std::ffi::c_uint = 0x0D05;
const GL_PACK_LSB_FIRST: std::ffi::c_uint = 0x0D01;
const GL_PACK_ROW_LENGTH: std::ffi::c_uint = 0x0D02;
const GL_PACK_SKIP_PIXELS: std::ffi::c_uint = 0x0D04;
const GL_PACK_SKIP_ROWS: std::ffi::c_uint = 0x0D03;
const GL_PACK_SWAP_BYTES: std::ffi::c_uint = 0x0D00;
const GL_UNPACK_ALIGNMENT: std::ffi::c_uint = 0x0CF5;
const GL_UNPACK_LSB_FIRST: std::ffi::c_uint = 0x0CF1;
const GL_UNPACK_ROW_LENGTH: std::ffi::c_uint = 0x0CF2;
const GL_UNPACK_SKIP_PIXELS: std::ffi::c_uint = 0x0CF4;
const GL_UNPACK_SKIP_ROWS: std::ffi::c_uint = 0x0CF3;
const GL_UNPACK_SWAP_BYTES: std::ffi::c_uint = 0x0CF0;
const GL_ZOOM_X: std::ffi::c_uint = 0x0D16;
const GL_ZOOM_Y: std::ffi::c_uint = 0x0D17;
/* Texture mapping */
const GL_TEXTURE_ENV: std::ffi::c_uint = 0x2300;
const GL_TEXTURE_ENV_MODE: std::ffi::c_uint = 0x2200;
const GL_TEXTURE_1D: std::ffi::c_uint = 0x0DE0;
const GL_TEXTURE_2D: std::ffi::c_uint = 0x0DE1;
const GL_TEXTURE_WRAP_S: std::ffi::c_uint = 0x2802;
const GL_TEXTURE_WRAP_T: std::ffi::c_uint = 0x2803;
const GL_TEXTURE_MAG_FILTER: std::ffi::c_uint = 0x2800;
const GL_TEXTURE_MIN_FILTER: std::ffi::c_uint = 0x2801;
const GL_TEXTURE_ENV_COLOR: std::ffi::c_uint = 0x2201;
const GL_TEXTURE_GEN_S: std::ffi::c_uint = 0x0C60;
const GL_TEXTURE_GEN_T: std::ffi::c_uint = 0x0C61;
const GL_TEXTURE_GEN_R: std::ffi::c_uint = 0x0C62;
const GL_TEXTURE_GEN_Q: std::ffi::c_uint = 0x0C63;
const GL_TEXTURE_GEN_MODE: std::ffi::c_uint = 0x2500;
const GL_TEXTURE_BORDER_COLOR: std::ffi::c_uint = 0x1004;
const GL_TEXTURE_WIDTH: std::ffi::c_uint = 0x1000;
const GL_TEXTURE_HEIGHT: std::ffi::c_uint = 0x1001;
const GL_TEXTURE_BORDER: std::ffi::c_uint = 0x1005;
const GL_TEXTURE_COMPONENTS: std::ffi::c_uint = 0x1003;
const GL_TEXTURE_RED_SIZE: std::ffi::c_uint = 0x805C;
const GL_TEXTURE_GREEN_SIZE: std::ffi::c_uint = 0x805D;
const GL_TEXTURE_BLUE_SIZE: std::ffi::c_uint = 0x805E;
const GL_TEXTURE_ALPHA_SIZE: std::ffi::c_uint = 0x805F;
const GL_TEXTURE_LUMINANCE_SIZE: std::ffi::c_uint = 0x8060;
const GL_TEXTURE_INTENSITY_SIZE: std::ffi::c_uint = 0x8061;
const GL_NEAREST_MIPMAP_NEAREST: std::ffi::c_uint = 0x2700;
const GL_NEAREST_MIPMAP_LINEAR: std::ffi::c_uint = 0x2702;
const GL_LINEAR_MIPMAP_NEAREST: std::ffi::c_uint = 0x2701;
const GL_LINEAR_MIPMAP_LINEAR: std::ffi::c_uint = 0x2703;
const GL_OBJECT_LINEAR: std::ffi::c_uint = 0x2401;
const GL_OBJECT_PLANE: std::ffi::c_uint = 0x2501;
const GL_EYE_LINEAR: std::ffi::c_uint = 0x2400;
const GL_EYE_PLANE: std::ffi::c_uint = 0x2502;
const GL_SPHERE_MAP: std::ffi::c_uint = 0x2402;
const GL_DECAL: std::ffi::c_uint = 0x2101;
const GL_MODULATE: std::ffi::c_uint = 0x2100;
const GL_NEAREST: std::ffi::c_uint = 0x2600;
const GL_REPEAT: std::ffi::c_uint = 0x2901;
const GL_CLAMP: std::ffi::c_uint = 0x2900;
const GL_S: std::ffi::c_uint = 0x2000;
const GL_T: std::ffi::c_uint = 0x2001;
const GL_R: std::ffi::c_uint = 0x2002;
const GL_Q: std::ffi::c_uint = 0x2003;
/* Utility */
const GL_VENDOR: std::ffi::c_uint = 0x1F00;
const GL_RENDERER: std::ffi::c_uint = 0x1F01;
const GL_VERSION: std::ffi::c_uint = 0x1F02;
const GL_EXTENSIONS: std::ffi::c_uint = 0x1F03;

/* glPush/PopAttrib bits */
const GL_CURRENT_BIT: std::ffi::c_uint = 0x00000001;
const GL_POINT_BIT: std::ffi::c_uint = 0x00000002;
const GL_LINE_BIT: std::ffi::c_uint = 0x00000004;
const GL_POLYGON_BIT: std::ffi::c_uint = 0x00000008;
const GL_POLYGON_STIPPLE_BIT: std::ffi::c_uint = 0x00000010;
const GL_PIXEL_MODE_BIT: std::ffi::c_uint = 0x00000020;
const GL_LIGHTING_BIT: std::ffi::c_uint = 0x00000040;
const GL_FOG_BIT: std::ffi::c_uint = 0x00000080;
const GL_DEPTH_BUFFER_BIT: std::ffi::c_uint = 0x00000100;
const GL_ACCUM_BUFFER_BIT: std::ffi::c_uint = 0x00000200;
const GL_STENCIL_BUFFER_BIT: std::ffi::c_uint = 0x00000400;
const GL_VIEWPORT_BIT: std::ffi::c_uint = 0x00000800;
const GL_TRANSFORM_BIT: std::ffi::c_uint = 0x00001000;
const GL_ENABLE_BIT: std::ffi::c_uint = 0x00002000;
const GL_COLOR_BUFFER_BIT: std::ffi::c_uint = 0x00004000;
const GL_HINT_BIT: std::ffi::c_uint = 0x00008000;
const GL_EVAL_BIT: std::ffi::c_uint = 0x00010000;
const GL_LIST_BIT: std::ffi::c_uint = 0x00020000;
const GL_TEXTURE_BIT: std::ffi::c_uint = 0x00040000;
const GL_SCISSOR_BIT: std::ffi::c_uint = 0x00080000;
const GL_ALL_ATTRIB_BITS: std::ffi::c_uint = u32::MAX;

/* OpenGL 1.1 */
const GL_PROXY_TEXTURE_1D: std::ffi::c_uint = 0x8063;
const GL_PROXY_TEXTURE_2D: std::ffi::c_uint = 0x8064;
const GL_TEXTURE_PRIORITY: std::ffi::c_uint = 0x8066;
const GL_TEXTURE_RESIDENT: std::ffi::c_uint = 0x8067;
const GL_TEXTURE_BINDING_1D: std::ffi::c_uint = 0x8068;
const GL_TEXTURE_BINDING_2D: std::ffi::c_uint = 0x8069;
const GL_TEXTURE_INTERNAL_FORMAT: std::ffi::c_uint = 0x1003;
const GL_ALPHA4: std::ffi::c_uint = 0x803B;
const GL_ALPHA8: std::ffi::c_uint = 0x803C;
const GL_ALPHA12: std::ffi::c_uint = 0x803D;
const GL_ALPHA16: std::ffi::c_uint = 0x803E;
const GL_LUMINANCE4: std::ffi::c_uint = 0x803F;
const GL_LUMINANCE8: std::ffi::c_uint = 0x8040;
const GL_LUMINANCE12: std::ffi::c_uint = 0x8041;
const GL_LUMINANCE16: std::ffi::c_uint = 0x8042;
const GL_LUMINANCE4_ALPHA4: std::ffi::c_uint = 0x8043;
const GL_LUMINANCE6_ALPHA2: std::ffi::c_uint = 0x8044;
const GL_LUMINANCE8_ALPHA8: std::ffi::c_uint = 0x8045;
const GL_LUMINANCE12_ALPHA4: std::ffi::c_uint = 0x8046;
const GL_LUMINANCE12_ALPHA12: std::ffi::c_uint = 0x8047;
const GL_LUMINANCE16_ALPHA16: std::ffi::c_uint = 0x8048;
const GL_INTENSITY: std::ffi::c_uint = 0x8049;
const GL_INTENSITY4: std::ffi::c_uint = 0x804A;
const GL_INTENSITY8: std::ffi::c_uint = 0x804B;
const GL_INTENSITY12: std::ffi::c_uint = 0x804C;
const GL_INTENSITY16: std::ffi::c_uint = 0x804D;
const GL_R3_G3_B2: std::ffi::c_uint = 0x2A10;
const GL_RGB4: std::ffi::c_uint = 0x804F;
const GL_RGB5: std::ffi::c_uint = 0x8050;
const GL_RGB8: std::ffi::c_uint = 0x8051;
const GL_RGB10: std::ffi::c_uint = 0x8052;
const GL_RGB12: std::ffi::c_uint = 0x8053;
const GL_RGB16: std::ffi::c_uint = 0x8054;
const GL_RGBA2: std::ffi::c_uint = 0x8055;
const GL_RGBA4: std::ffi::c_uint = 0x8056;
const GL_RGB5_A1: std::ffi::c_uint = 0x8057;
const GL_RGBA8: std::ffi::c_uint = 0x8058;
const GL_RGB10_A2: std::ffi::c_uint = 0x8059;
const GL_RGBA12: std::ffi::c_uint = 0x805A;
const GL_RGBA16: std::ffi::c_uint = 0x805B;
const GL_CLIENT_PIXEL_STORE_BIT: std::ffi::c_uint = 0x00000001;
const GL_CLIENT_VERTEX_ARRAY_BIT: std::ffi::c_uint = 0x00000002;
const GL_ALL_CLIENT_ATTRIB_BITS: std::ffi::c_uint = u32::MAX;
const GL_CLIENT_ALL_ATTRIB_BITS: std::ffi::c_uint = u32::MAX;

// I assume this is the GL instance?
type GL = std::ffi::c_void;

pub mod sys {
    use crate::gfx::opengl::{ConstGLubytePointer, MutableGLubytePointer};

    use super::{
        GLbitfield, GLboolean, GLbyte, GLclampd, GLclampf, GLdouble, GLenum, GLfloat, GLint,
        GLshort, GLsizei, GLubyte, GLuint, GLushort, GL,
    };

    unsafe extern "system" {
        pub unsafe fn glClearIndex(c: GLfloat) -> ();

        /*
         * Miscellaneous
         */

        pub unsafe fn glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);

        pub unsafe fn glClear(mask: GLbitfield) -> ();

        pub unsafe fn glIndexMask(mask: GLuint) -> ();

        pub unsafe fn glColorMask(
            red: GLboolean,
            green: GLboolean,
            blue: GLboolean,
            alpha: GLboolean,
        );

        pub unsafe fn glAlphaFunc(func: GLenum, r#ref: GLclampf) -> ();

        pub unsafe fn glBlendFunc(sfactor: GLenum, dfactor: GLenum) -> ();

        pub unsafe fn glLogicOp(opcode: GLenum) -> ();

        pub unsafe fn glCullFace(mode: GLenum) -> ();

        pub unsafe fn glFrontFace(mode: GLenum) -> ();

        pub unsafe fn glPointSize(size: GLfloat) -> ();

        pub unsafe fn glLineWidth(width: GLfloat) -> ();

        pub unsafe fn glLineStipple(factor: GLint, pattern: GLushort) -> ();

        pub unsafe fn glPolygonMode(face: GLenum, mode: GLenum) -> ();

        pub unsafe fn glPolygonOffset(factor: GLfloat, units: GLfloat) -> ();

        // original type: *const GLubyte
        pub unsafe fn glPolygonStipple(mask: ConstGLubytePointer);

        pub unsafe fn glGetPolygonStipple(mask: MutableGLubytePointer);

        pub unsafe fn glEdgeFlag(flag: GLboolean) -> ();

        pub unsafe fn glEdgeFlagv(flag: *const GLboolean);

        pub unsafe fn glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei);

        pub unsafe fn glClipPlane(plane: GLenum, equation: *const GLdouble);

        pub unsafe fn glGetClipPlane(plane: GLenum, equation: *mut GLdouble);

        pub unsafe fn glDrawBuffer(mode: GLenum) -> ();

        pub unsafe fn glReadBuffer(mode: GLenum) -> ();

        pub unsafe fn glEnable(cap: GLenum) -> ();

        pub unsafe fn glDisable(cap: GLenum) -> ();

        pub unsafe fn glIsEnabled(cap: GLenum) -> GLboolean;

        pub unsafe fn glEnableClientState(cap: GLenum) -> (); /* 1.1 */

        pub unsafe fn glDisableClientState(cap: GLenum) -> (); /* 1.1 */

        pub unsafe fn glGetBooleanv(pname: GLenum, params: *mut GLboolean);

        pub unsafe fn glGetDoublev(pname: GLenum, params: *mut GLdouble);

        pub unsafe fn glGetFloatv(pname: GLenum, params: *mut GLfloat);

        pub unsafe fn glGetIntegerv(pname: GLenum, params: *mut GLint);

        pub unsafe fn glPushAttrib(mask: GLbitfield) -> ();

        pub unsafe fn glPopAttrib() -> ();

        pub unsafe fn glPushClientAttrib(mask: GLbitfield) -> (); /* 1.1 */

        pub unsafe fn glPopClientAttrib() -> (); /* 1.1 */

        pub unsafe fn glRenderMode(mode: GLenum) -> GLint;

        pub unsafe fn glGetError() -> GLenum;

        pub unsafe fn glGetString(name: GLenum) -> ConstGLubytePointer;

        pub unsafe fn glFinish() -> ();

        pub unsafe fn glFlush() -> ();

        pub unsafe fn glHint(target: GLenum, mode: GLenum) -> ();

        /*
         * Depth Buffer
         */

        pub unsafe fn glClearDepth(depth: GLclampd) -> ();

        pub unsafe fn glDepthFunc(func: GLenum) -> ();

        pub unsafe fn glDepthMask(flag: GLboolean) -> ();

        pub unsafe fn glDepthRange(near_val: GLclampd, far_val: GLclampd);

        /*
         * Accumulation Buffer
         */

        pub unsafe fn glClearAccum(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);

        pub unsafe fn glAccum(op: GLenum, value: GLfloat) -> ();

        /*
         * Transformation
         */

        pub unsafe fn glMatrixMode(mode: GLenum) -> ();

        pub unsafe fn glOrtho(
            left: GLdouble,
            right: GLdouble,
            bottom: GLdouble,
            top: GLdouble,
            near_val: GLdouble,
            far_val: GLdouble,
        );

        pub unsafe fn glFrustum(
            left: GLdouble,
            right: GLdouble,
            bottom: GLdouble,
            top: GLdouble,
            near_val: GLdouble,
            far_val: GLdouble,
        );

        pub unsafe fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);

        pub unsafe fn glPushMatrix() -> ();

        pub unsafe fn glPopMatrix() -> ();

        pub unsafe fn glLoadIdentity() -> ();

        pub unsafe fn glLoadMatrixd(m: *const GLdouble);
        pub unsafe fn glLoadMatrixf(m: *const GLfloat);

        pub unsafe fn glMultMatrixd(m: *const GLdouble);
        pub unsafe fn glMultMatrixf(m: *const GLfloat);

        pub unsafe fn glRotated(angle: GLdouble, x: GLdouble, y: GLdouble, z: GLdouble);
        pub unsafe fn glRotatef(angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

        pub unsafe fn glScaled(x: GLdouble, y: GLdouble, z: GLdouble);
        pub unsafe fn glScalef(x: GLfloat, y: GLfloat, z: GLfloat);

        pub unsafe fn glTranslated(x: GLdouble, y: GLdouble, z: GLdouble);
        pub unsafe fn glTranslatef(x: GLfloat, y: GLfloat, z: GLfloat);

        /*
         * Display Lists
         */

        pub unsafe fn glIsList(list: GLuint) -> GLboolean;

        pub unsafe fn glDeleteLists(list: GLuint, range: GLsizei) -> ();

        pub unsafe fn glGenLists(range: GLsizei) -> GLuint;

        pub unsafe fn glNewList(list: GLuint, mode: GLenum) -> ();

        pub unsafe fn glEndList() -> ();

        pub unsafe fn glCallList(list: GLuint) -> ();

        pub unsafe fn glCallLists(n: GLsizei, r#type: GLenum, lists: *const GL);

        pub unsafe fn glListBase(base: GLuint) -> ();

        /*
         * Drawing Functions
         */

        pub unsafe fn glBegin(mode: GLenum) -> ();

        pub unsafe fn glEnd() -> ();

        pub unsafe fn glVertex2d(x: GLdouble, y: GLdouble);
        pub unsafe fn glVertex2f(x: GLfloat, y: GLfloat);
        pub unsafe fn glVertex2i(x: GLint, y: GLint);
        pub unsafe fn glVertex2s(x: GLshort, y: GLshort);

        pub unsafe fn glVertex3d(x: GLdouble, y: GLdouble, z: GLdouble);
        pub unsafe fn glVertex3f(x: GLfloat, y: GLfloat, z: GLfloat);
        pub unsafe fn glVertex3i(x: GLint, y: GLint, z: GLint);
        pub unsafe fn glVertex3s(x: GLshort, y: GLshort, z: GLshort);

        pub unsafe fn glVertex4d(x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
        pub unsafe fn glVertex4f(x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
        pub unsafe fn glVertex4i(x: GLint, y: GLint, z: GLint, w: GLint);
        pub unsafe fn glVertex4s(x: GLshort, y: GLshort, z: GLshort, w: GLshort);

        pub unsafe fn glVertex2dv(v: *const GLdouble);
        pub unsafe fn glVertex2fv(v: *const GLfloat);
        pub unsafe fn glVertex2iv(v: *const GLint);
        pub unsafe fn glVertex2sv(v: *const GLshort);

        pub unsafe fn glVertex3dv(v: *const GLdouble);
        pub unsafe fn glVertex3fv(v: *const GLfloat);
        pub unsafe fn glVertex3iv(v: *const GLint);
        pub unsafe fn glVertex3sv(v: *const GLshort);

        pub unsafe fn glVertex4dv(v: *const GLdouble);
        pub unsafe fn glVertex4fv(v: *const GLfloat);
        pub unsafe fn glVertex4iv(v: *const GLint);
        pub unsafe fn glVertex4sv(v: *const GLshort);

        pub unsafe fn glNormal3b(nx: GLbyte, ny: GLbyte, nz: GLbyte);
        pub unsafe fn glNormal3d(nx: GLdouble, ny: GLdouble, nz: GLdouble);
        pub unsafe fn glNormal3f(nx: GLfloat, ny: GLfloat, nz: GLfloat);
        pub unsafe fn glNormal3i(nx: GLint, ny: GLint, nz: GLint);
        pub unsafe fn glNormal3s(nx: GLshort, ny: GLshort, nz: GLshort);

        pub unsafe fn glNormal3bv(v: *const GLbyte);
        pub unsafe fn glNormal3dv(v: *const GLdouble);
        pub unsafe fn glNormal3fv(v: *const GLfloat);
        pub unsafe fn glNormal3iv(v: *const GLint);
        pub unsafe fn glNormal3sv(v: *const GLshort);

        pub unsafe fn glIndexd(c: GLdouble) -> ();
        pub unsafe fn glIndexf(c: GLfloat) -> ();
        pub unsafe fn glIndexi(c: GLint) -> ();
        pub unsafe fn glIndexs(c: GLshort) -> ();
        pub unsafe fn glIndexub(c: GLubyte) -> (); /* 1.1 */

        pub unsafe fn glIndexdv(c: *const GLdouble);
        pub unsafe fn glIndexfv(c: *const GLfloat);
        pub unsafe fn glIndexiv(c: *const GLint);
        pub unsafe fn glIndexsv(c: *const GLshort);
        pub unsafe fn glIndexubv(c: *const GLubyte); /* 1.1 */

        pub unsafe fn glColor3b(red: GLbyte, green: GLbyte, blue: GLbyte);
        pub unsafe fn glColor3d(red: GLdouble, green: GLdouble, blue: GLdouble);
        pub unsafe fn glColor3f(red: GLfloat, green: GLfloat, blue: GLfloat);
        pub unsafe fn glColor3i(red: GLint, green: GLint, blue: GLint);
        pub unsafe fn glColor3s(red: GLshort, green: GLshort, blue: GLshort);
        pub unsafe fn glColor3ub(red: GLubyte, green: GLubyte, blue: GLubyte);
        pub unsafe fn glColor3ui(red: GLuint, green: GLuint, blue: GLuint);
        pub unsafe fn glColor3us(red: GLushort, green: GLushort, blue: GLushort);

        pub unsafe fn glColor4b(red: GLbyte, green: GLbyte, blue: GLbyte, alpha: GLbyte);
        pub unsafe fn glColor4d(red: GLdouble, green: GLdouble, blue: GLdouble, alpha: GLdouble);
        pub unsafe fn glColor4f(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
        pub unsafe fn glColor4i(red: GLint, green: GLint, blue: GLint, alpha: GLint);
        pub unsafe fn glColor4s(red: GLshort, green: GLshort, blue: GLshort, alpha: GLshort);
        pub unsafe fn glColor4ub(red: GLubyte, green: GLubyte, blue: GLubyte, alpha: GLubyte);
        pub unsafe fn glColor4ui(red: GLuint, green: GLuint, blue: GLuint, alpha: GLuint);
        pub unsafe fn glColor4us(red: GLushort, green: GLushort, blue: GLushort, alpha: GLushort);

        pub unsafe fn glColor3bv(v: *const GLbyte);
        pub unsafe fn glColor3dv(v: *const GLdouble);
        pub unsafe fn glColor3fv(v: *const GLfloat);
        pub unsafe fn glColor3iv(v: *const GLint);
        pub unsafe fn glColor3sv(v: *const GLshort);
        pub unsafe fn glColor3ubv(v: *const GLubyte);
        pub unsafe fn glColor3uiv(v: *const GLuint);
        pub unsafe fn glColor3usv(v: *const GLushort);

        pub unsafe fn glColor4bv(v: *const GLbyte);
        pub unsafe fn glColor4dv(v: *const GLdouble);
        pub unsafe fn glColor4fv(v: *const GLfloat);
        pub unsafe fn glColor4iv(v: *const GLint);
        pub unsafe fn glColor4sv(v: *const GLshort);
        pub unsafe fn glColor4ubv(v: *const GLubyte);
        pub unsafe fn glColor4uiv(v: *const GLuint);
        pub unsafe fn glColor4usv(v: *const GLushort);

        pub unsafe fn glTexCoord1d(s: GLdouble);
        pub unsafe fn glTexCoord1f(s: GLfloat);
        pub unsafe fn glTexCoord1i(s: GLint);
        pub unsafe fn glTexCoord1s(s: GLshort);

        pub unsafe fn glTexCoord2d(s: GLdouble, t: GLdouble);
        pub unsafe fn glTexCoord2f(s: GLfloat, t: GLfloat);
        pub unsafe fn glTexCoord2i(s: GLint, t: GLint);
        pub unsafe fn glTexCoord2s(s: GLshort, t: GLshort);

        pub unsafe fn glTexCoord3d(s: GLdouble, t: GLdouble, r: GLdouble);
        pub unsafe fn glTexCoord3f(s: GLfloat, t: GLfloat, r: GLfloat);
        pub unsafe fn glTexCoord3i(s: GLint, t: GLint, r: GLint);
        pub unsafe fn glTexCoord3s(s: GLshort, t: GLshort, r: GLshort);

        pub unsafe fn glTexCoord4d(s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble);
        pub unsafe fn glTexCoord4f(s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat);
        pub unsafe fn glTexCoord4i(s: GLint, t: GLint, r: GLint, q: GLint);
        pub unsafe fn glTexCoord4s(s: GLshort, t: GLshort, r: GLshort, q: GLshort);

        pub unsafe fn glTexCoord1dv(v: *const GLdouble);
        pub unsafe fn glTexCoord1fv(v: *const GLfloat);
        pub unsafe fn glTexCoord1iv(v: *const GLint);
        pub unsafe fn glTexCoord1sv(v: *const GLshort);

        pub unsafe fn glTexCoord2dv(v: *const GLdouble);
        pub unsafe fn glTexCoord2fv(v: *const GLfloat);
        pub unsafe fn glTexCoord2iv(v: *const GLint);
        pub unsafe fn glTexCoord2sv(v: *const GLshort);

        pub unsafe fn glTexCoord3dv(v: *const GLdouble);
        pub unsafe fn glTexCoord3fv(v: *const GLfloat);
        pub unsafe fn glTexCoord3iv(v: *const GLint);
        pub unsafe fn glTexCoord3sv(v: *const GLshort);

        pub unsafe fn glTexCoord4dv(v: *const GLdouble);
        pub unsafe fn glTexCoord4fv(v: *const GLfloat);
        pub unsafe fn glTexCoord4iv(v: *const GLint);
        pub unsafe fn glTexCoord4sv(v: *const GLshort);

        pub unsafe fn glRasterPos2d(x: GLdouble, y: GLdouble);
        pub unsafe fn glRasterPos2f(x: GLfloat, y: GLfloat);
        pub unsafe fn glRasterPos2i(x: GLint, y: GLint);
        pub unsafe fn glRasterPos2s(x: GLshort, y: GLshort);

        pub unsafe fn glRasterPos3d(x: GLdouble, y: GLdouble, z: GLdouble);
        pub unsafe fn glRasterPos3f(x: GLfloat, y: GLfloat, z: GLfloat);
        pub unsafe fn glRasterPos3i(x: GLint, y: GLint, z: GLint);
        pub unsafe fn glRasterPos3s(x: GLshort, y: GLshort, z: GLshort);

        pub unsafe fn glRasterPos4d(x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
        pub unsafe fn glRasterPos4f(x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
        pub unsafe fn glRasterPos4i(x: GLint, y: GLint, z: GLint, w: GLint);
        pub unsafe fn glRasterPos4s(x: GLshort, y: GLshort, z: GLshort, w: GLshort);

        pub unsafe fn glRasterPos2dv(v: *const GLdouble);
        pub unsafe fn glRasterPos2fv(v: *const GLfloat);
        pub unsafe fn glRasterPos2iv(v: *const GLint);
        pub unsafe fn glRasterPos2sv(v: *const GLshort);

        pub unsafe fn glRasterPos3dv(v: *const GLdouble);
        pub unsafe fn glRasterPos3fv(v: *const GLfloat);
        pub unsafe fn glRasterPos3iv(v: *const GLint);
        pub unsafe fn glRasterPos3sv(v: *const GLshort);

        pub unsafe fn glRasterPos4dv(v: *const GLdouble);
        pub unsafe fn glRasterPos4fv(v: *const GLfloat);
        pub unsafe fn glRasterPos4iv(v: *const GLint);
        pub unsafe fn glRasterPos4sv(v: *const GLshort);

        pub unsafe fn glRectd(x1: GLdouble, y1: GLdouble, x2: GLdouble, y2: GLdouble);
        pub unsafe fn glRectf(x1: GLfloat, y1: GLfloat, x2: GLfloat, y2: GLfloat);
        pub unsafe fn glRecti(x1: GLint, y1: GLint, x2: GLint, y2: GLint);
        pub unsafe fn glRects(x1: GLshort, y1: GLshort, x2: GLshort, y2: GLshort);

        pub unsafe fn glRectdv(v1: *const GLdouble, v2: *const GLdouble);
        pub unsafe fn glRectfv(v1: *const GLfloat, v2: *const GLfloat);
        pub unsafe fn glRectiv(v1: *const GLint, v2: *const GLint);
        pub unsafe fn glRectsv(v1: *const GLshort, v2: *const GLshort);

        /*
         * Vertex Arrays  (1.1)
         */

        pub unsafe fn glVertexPointer(size: GLint, r#type: GLenum, stride: GLsizei, ptr: *const GL);

        pub unsafe fn glNormalPointer(r#type: GLenum, stride: GLsizei, ptr: *const GL);

        pub unsafe fn glColorPointer(size: GLint, r#type: GLenum, stride: GLsizei, ptr: *const GL);

        pub unsafe fn glIndexPointer(r#type: GLenum, stride: GLsizei, ptr: *const GL);

        pub unsafe fn glTexCoordPointer(
            size: GLint,
            r#type: GLenum,
            stride: GLsizei,
            ptr: *const GL,
        );

        pub unsafe fn glEdgeFlagPointer(stride: GLsizei, ptr: *const GL);

        pub unsafe fn glGetPointerv(pname: GLenum, params: *const *mut std::ffi::c_void);

        pub unsafe fn glArrayElement(i: GLint) -> ();

        pub unsafe fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei);

        pub unsafe fn glDrawElements(
            mode: GLenum,
            count: GLsizei,
            r#type: GLenum,
            indices: *const GL,
        );

        pub unsafe fn glInterleavedArrays(format: GLenum, stride: GLsizei, pointer: *const GL);

        /*
         * Lighting
         */

        pub unsafe fn glShadeModel(mode: GLenum) -> ();

        pub unsafe fn glLightf(light: GLenum, pname: GLenum, param: GLfloat);
        pub unsafe fn glLighti(light: GLenum, pname: GLenum, param: GLint);
        pub unsafe fn glLightfv(light: GLenum, pname: GLenum, params: *const GLfloat);
        pub unsafe fn glLightiv(light: GLenum, pname: GLenum, params: *const GLint);

        pub unsafe fn glGetLightfv(light: GLenum, pname: GLenum, params: *mut GLfloat);
        pub unsafe fn glGetLightiv(light: GLenum, pname: GLenum, params: *mut GLint);

        pub unsafe fn glLightModelf(pname: GLenum, param: GLfloat) -> ();
        pub unsafe fn glLightModeli(pname: GLenum, param: GLint) -> ();
        pub unsafe fn glLightModelfv(pname: GLenum, params: *const GLfloat);
        pub unsafe fn glLightModeliv(pname: GLenum, params: *const GLint);

        pub unsafe fn glMaterialf(face: GLenum, pname: GLenum, param: GLfloat);
        pub unsafe fn glMateriali(face: GLenum, pname: GLenum, param: GLint);
        pub unsafe fn glMaterialfv(face: GLenum, pname: GLenum, params: *const GLfloat);
        pub unsafe fn glMaterialiv(face: GLenum, pname: GLenum, params: *const GLint);

        pub unsafe fn glGetMaterialfv(face: GLenum, pname: GLenum, params: *const GLfloat);
        pub unsafe fn glGetMaterialiv(face: GLenum, pname: GLenum, params: *const GLint);

        pub unsafe fn glColorMaterial(face: GLenum, mode: GLenum) -> ();

        /*
         * Raster functions
         */

        pub unsafe fn glPixelZoom(xfactor: GLfloat, yfactor: GLfloat) -> ();

        pub unsafe fn glPixelStoref(pname: GLenum, param: GLfloat) -> ();
        pub unsafe fn glPixelStorei(pname: GLenum, param: GLint) -> ();

        pub unsafe fn glPixelTransferf(pname: GLenum, param: GLfloat) -> ();
        pub unsafe fn glPixelTransferi(pname: GLenum, param: GLint) -> ();

        pub unsafe fn glPixelMapfv(map: GLenum, mapsize: GLsizei, values: *const GLfloat);
        pub unsafe fn glPixelMapuiv(map: GLenum, mapsize: GLsizei, values: *const GLuint);
        pub unsafe fn glPixelMapusv(map: GLenum, mapsize: GLsizei, values: *const GLushort);

        pub unsafe fn glGetPixelMapfv(map: GLenum, values: *mut GLfloat);
        pub unsafe fn glGetPixelMapuiv(map: GLenum, values: *mut GLuint);
        pub unsafe fn glGetPixelMapusv(map: GLenum, values: *mut GLushort);

        pub unsafe fn glBitmap(
            width: GLsizei,
            height: GLsizei,
            xorig: GLfloat,
            yorig: GLfloat,
            xmove: GLfloat,
            ymove: GLfloat,
            bitmap: *const GLubyte,
        );

        pub unsafe fn glReadPixels(
            x: GLint,
            y: GLint,
            width: GLsizei,
            height: GLsizei,
            format: GLenum,
            r#type: GLenum,
            pixels: *mut GL,
        );

        pub unsafe fn glDrawPixels(
            width: GLsizei,
            height: GLsizei,
            format: GLenum,
            r#type: GLenum,
            pixels: *const GL,
        );

        pub unsafe fn glCopyPixels(
            x: GLint,
            y: GLint,
            width: GLsizei,
            height: GLsizei,
            r#type: GLenum,
        );

        /*
         * Stenciling
         */

        pub unsafe fn glStencilFunc(func: GLenum, r#ref: GLint, mask: GLuint);

        pub unsafe fn glStencilMask(mask: GLuint) -> ();

        pub unsafe fn glStencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum);

        pub unsafe fn glClearStencil(s: GLint) -> ();

        /*
         * Texture mapping
         */

        pub unsafe fn glTexGend(coord: GLenum, pname: GLenum, param: GLdouble);
        pub unsafe fn glTexGenf(coord: GLenum, pname: GLenum, param: GLfloat);
        pub unsafe fn glTexGeni(coord: GLenum, pname: GLenum, param: GLint);

        pub unsafe fn glTexGendv(coord: GLenum, pname: GLenum, params: *const GLdouble);
        pub unsafe fn glTexGenfv(coord: GLenum, pname: GLenum, params: *const GLfloat);
        pub unsafe fn glTexGeniv(coord: GLenum, pname: GLenum, params: *const GLint);

        pub unsafe fn glGetTexGendv(coord: GLenum, pname: GLenum, params: *mut GLdouble);
        pub unsafe fn glGetTexGenfv(coord: GLenum, pname: GLenum, params: *const GLfloat);
        pub unsafe fn glGetTexGeniv(coord: GLenum, pname: GLenum, params: *const GLint);

        pub unsafe fn glTexEnvf(target: GLenum, pname: GLenum, param: GLfloat);
        pub unsafe fn glTexEnvi(target: GLenum, pname: GLenum, param: GLint);

        pub unsafe fn glTexEnvfv(target: GLenum, pname: GLenum, params: *const GLfloat);
        pub unsafe fn glTexEnviv(target: GLenum, pname: GLenum, params: *const GLint);

        pub unsafe fn glGetTexEnvfv(target: GLenum, pname: GLenum, params: *const GLfloat);
        pub unsafe fn glGetTexEnviv(target: GLenum, pname: GLenum, params: *const GLint);

        pub unsafe fn glTexParameterf(target: GLenum, pname: GLenum, param: GLfloat);
        pub unsafe fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint);

        pub unsafe fn glTexParameterfv(target: GLenum, pname: GLenum, params: *const GLfloat);
        pub unsafe fn glTexParameteriv(target: GLenum, pname: GLenum, params: *const GLint);

        pub unsafe fn glGetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat);
        pub unsafe fn glGetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint);

        pub unsafe fn glGetTexLevelParameterfv(
            target: GLenum,
            level: GLint,
            pname: GLenum,
            params: *mut GLfloat,
        );
        pub unsafe fn glGetTexLevelParameteriv(
            target: GLenum,
            level: GLint,
            pname: GLenum,
            params: *mut GLint,
        );

        pub unsafe fn glTexImage1D(
            target: GLenum,
            level: GLint,
            internalFormat: GLint,
            width: GLsizei,
            border: GLint,
            format: GLenum,
            r#type: GLenum,
            pixels: *const GL,
        );

        pub unsafe fn glTexImage2D(
            target: GLenum,
            level: GLint,
            internalFormat: GLint,
            width: GLsizei,
            height: GLsizei,
            border: GLint,
            format: GLenum,
            r#type: GLenum,
            pixels: *const GL,
        );

        pub unsafe fn glGetTexImage(
            target: GLenum,
            level: GLint,
            format: GLenum,
            r#type: GLenum,
            pixels: *mut GL,
        );

        /* 1.1 functions */

        pub unsafe fn glGenTextures(n: GLsizei, textures: *mut GLuint);

        pub unsafe fn glDeleteTextures(n: GLsizei, textures: *const GLuint);

        pub unsafe fn glBindTexture(target: GLenum, texture: GLuint) -> ();

        pub unsafe fn glPrioritizeTextures(
            n: GLsizei,
            textures: *const GLuint,
            priorities: *const GLclampf,
        );

        pub unsafe fn glAreTexturesResident(
            n: GLsizei,
            textures: *const GLuint,
            residences: *mut GLboolean,
        ) -> GLboolean;

        pub unsafe fn glIsTexture(texture: GLuint) -> GLboolean;

        pub unsafe fn glTexSubImage1D(
            target: GLenum,
            level: GLint,
            xoffset: GLint,
            width: GLsizei,
            format: GLenum,
            r#type: GLenum,
            pixels: *const GL,
        );

        pub unsafe fn glTexSubImage2D(
            target: GLenum,
            level: GLint,
            xoffset: GLint,
            yoffset: GLint,
            width: GLsizei,
            height: GLsizei,
            format: GLenum,
            r#type: GLenum,
            pixels: *const GL,
        );

        pub unsafe fn glCopyTexImage1D(
            target: GLenum,
            level: GLint,
            internalformat: GLenum,
            x: GLint,
            y: GLint,
            width: GLsizei,
            border: GLint,
        );

        pub unsafe fn glCopyTexImage2D(
            target: GLenum,
            level: GLint,
            internalformat: GLenum,
            x: GLint,
            y: GLint,
            width: GLsizei,
            height: GLsizei,
            border: GLint,
        );

        pub unsafe fn glCopyTexSubImage1D(
            target: GLenum,
            level: GLint,
            xoffset: GLint,
            x: GLint,
            y: GLint,
            width: GLsizei,
        );

        pub unsafe fn glCopyTexSubImage2D(
            target: GLenum,
            level: GLint,
            xoffset: GLint,
            yoffset: GLint,
            x: GLint,
            y: GLint,
            width: GLsizei,
            height: GLsizei,
        );

        /*
         * Evaluators
         */

        pub unsafe fn glMap1d(
            target: GLenum,
            u1: GLdouble,
            u2: GLdouble,
            stride: GLint,
            order: GLint,
            points: *const GLdouble,
        );
        pub unsafe fn glMap1f(
            target: GLenum,
            u1: GLfloat,
            u2: GLfloat,
            stride: GLint,
            order: GLint,
            points: *const GLfloat,
        );

        pub unsafe fn glMap2d(
            target: GLenum,
            u1: GLdouble,
            u2: GLdouble,
            ustride: GLint,
            uorder: GLint,
            v1: GLdouble,
            v2: GLdouble,
            vstride: GLint,
            vorder: GLint,
            points: *const GLdouble,
        );
        pub unsafe fn glMap2f(
            target: GLenum,
            u1: GLfloat,
            u2: GLfloat,
            ustride: GLint,
            uorder: GLint,
            v1: GLfloat,
            v2: GLfloat,
            vstride: GLint,
            vorder: GLint,
            points: *const GLfloat,
        );

        pub unsafe fn glGetMapdv(target: GLenum, query: GLenum, v: *mut GLdouble);
        pub unsafe fn glGetMapfv(target: GLenum, query: GLenum, v: *mut GLfloat);
        pub unsafe fn glGetMapiv(target: GLenum, query: GLenum, v: *mut GLint);

        pub unsafe fn glEvalCoord1d(u: GLdouble);
        pub unsafe fn glEvalCoord1f(u: GLfloat);

        pub unsafe fn glEvalCoord1dv(u: *const GLdouble);
        pub unsafe fn glEvalCoord1fv(u: *const GLfloat);

        pub unsafe fn glEvalCoord2d(u: GLdouble, v: GLdouble);
        pub unsafe fn glEvalCoord2f(u: GLfloat, v: GLfloat);

        pub unsafe fn glEvalCoord2dv(u: *const GLdouble);
        pub unsafe fn glEvalCoord2fv(u: *const GLfloat);

        pub unsafe fn glMapGrid1d(un: GLint, u1: GLdouble, u2: GLdouble);
        pub unsafe fn glMapGrid1f(un: GLint, u1: GLfloat, u2: GLfloat);

        pub unsafe fn glMapGrid2d(
            un: GLint,
            u1: GLdouble,
            u2: GLdouble,
            vn: GLint,
            v1: GLdouble,
            v2: GLdouble,
        );
        pub unsafe fn glMapGrid2f(
            un: GLint,
            u1: GLfloat,
            u2: GLfloat,
            vn: GLint,
            v1: GLfloat,
            v2: GLfloat,
        );

        pub unsafe fn glEvalPoint1(i: GLint);

        pub unsafe fn glEvalPoint2(i: GLint, j: GLint);

        pub unsafe fn glEvalMesh1(mode: GLenum, i1: GLint, i2: GLint);

        pub unsafe fn glEvalMesh2(mode: GLenum, i1: GLint, i2: GLint, j1: GLint, j2: GLint);

        /*
         * Fog
         */

        pub unsafe fn glFogf(pname: GLenum, param: GLfloat) -> ();

        pub unsafe fn glFogi(pname: GLenum, param: GLint) -> ();

        pub unsafe fn glFogfv(pname: GLenum, params: *const GLfloat);

        pub unsafe fn glFogiv(pname: GLenum, params: *const GLint);

        /*
         * Selection and Feedback
         */

        pub unsafe fn glFeedbackBuffer(size: GLsizei, r#type: GLenum, buffer: *mut GLfloat);

        pub unsafe fn glPassThrough(token: GLfloat) -> ();

        pub unsafe fn glSelectBuffer(size: GLsizei, buffer: *mut GLuint);

        pub unsafe fn glInitNames() -> ();

        pub unsafe fn glLoadName(name: GLuint) -> ();

        pub unsafe fn glPushName(name: GLuint) -> ();

        pub unsafe fn glPopName() -> ();

    }
}

#[cfg(feature = "target_os_linux")]
pub struct LinuxGlContext {
    pub display: *mut x11::xlib::Display,
    pub screen: *mut x11::xlib::Screen,
    pub visualinfo: *mut x11::xlib::XVisualInfo,
    // setwindowattributes:
    pub glcontext: x11::glx::GLXContext,
    pub window: x11::xlib::Window,
}
#[cfg(feature = "target_os_linux")]
impl Drop for LinuxGlContext {
    fn drop(&mut self) {
        unsafe {
            glx::glXDestroyContext(self.display, self.glcontext);
            x11::xlib::XDestroyWindow(self.display, self.window);
            x11::xlib::XFree(self.visualinfo.cast());
            x11::xlib::XCloseDisplay(self.display);
        };
    }
}

#[cfg(feature = "target_os_linux")]
pub fn create_window_and_init_gl() -> LinuxGlContext {
    let display = unsafe { x11::xlib::XOpenDisplay(std::ptr::null()) };
    if display.is_null() {
        panic!("null display pointer");
    }
    let screen = unsafe { XDefaultScreenOfDisplay(display) };
    // i guess for the display buffer?
    let mut template: x11::xlib::XVisualInfo = unsafe { std::mem::zeroed() };
    let mut nitems = 0;
    let mask = 0;
    let visualinfo =
        unsafe { x11::xlib::XGetVisualInfo(display, mask, &mut template, &mut nitems) };

    use x11::xlib::{AllocNone, CWColormap, CWEventMask, ExposureMask, KeyPressMask};

    let mut swa: x11::xlib::XSetWindowAttributes = unsafe { std::mem::zeroed() };
    swa.colormap = unsafe {
        x11::xlib::XCreateColormap(display, (*screen).root, (*visualinfo).visual, AllocNone)
    };
    swa.event_mask = ExposureMask | KeyPressMask;
    swa.border_pixel = 1;
    let app_window = unsafe {
        use x11::xlib::CWBorderPixel;
        x11::xlib::XCreateWindow(
            display,
            (*screen).root,
            100,
            100,
            500,
            500,
            1,
            (*visualinfo).depth,
            x11::xlib::InputOutput as u32,
            (*visualinfo).visual,
            CWColormap | CWEventMask | CWBorderPixel,
            &mut swa,
        )
    };
    let ctx: x11::glx::GLXContext =
        unsafe { glx::glXCreateContext(display, visualinfo, std::ptr::null_mut(), GL_TRUE as i32) };
    unsafe { glx::glXMakeCurrent(display, app_window, ctx) };
    unsafe { x11::xlib::XMapWindow(display, app_window) };
    unsafe { x11::xlib::XSync(display, x11::xlib::False) };
    LinuxGlContext {
        display: display,
        screen: screen,
        visualinfo,
        glcontext: ctx,
        window: app_window,
    }
}

pub fn gl_get_version() -> String {
    let gl_string_pointer = unsafe { sys::glGetString(GL_VERSION) };
    if gl_string_pointer.is_null() {
        panic!("null string pointer");
    }
    gl_string_pointer.to_string_lossy().unwrap()
}
#[cfg(feature = "target_os_linux")]
#[test]
pub fn test_gl_get_version() {
    // gl must be initialized first before we can get the opengl version

    use crate::gfx::opengl::sys::{glClear, glClearColor};
    let context = create_window_and_init_gl();
    // let default_screen = x11::xlib::default
    let gl_version = gl_get_version();
    println!("{gl_version}");
    unsafe { sys::glMatrixMode(GL_PROJECTION) };
    for _ in 0..600 {
        unsafe {
            glClearColor(GLclampf(0.0), GLclampf(0.0), GLclampf(0.0), GLclampf(0.0));
            glClear(GLbitfield(GL_COLOR_BUFFER_BIT));
            sys::glLoadIdentity();
            sys::glBegin(GL_TRIANGLES);
            sys::glColor3f(GLfloat(1.0), GLfloat(1.0), GLfloat(1.0));
            sys::glVertex2f(GLfloat(0.0), GLfloat(1.0));
            sys::glVertex2f(GLfloat(-1.0), GLfloat(-1.0));
            sys::glVertex2f(GLfloat(1.0), GLfloat(-1.0));
            sys::glEnd();
            sys::glFlush();
            glx::glXSwapBuffers(context.display, context.window);
        }
    }

    // loop {}
}
