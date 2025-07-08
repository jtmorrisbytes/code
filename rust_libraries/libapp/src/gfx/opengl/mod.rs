#![allow(non_camel_case_types)]

use std::collections::HashMap;

// OPENGL 1.1
pub type GLenum = std::os::raw::c_uint;
pub type GLboolean = std::os::raw::c_uchar;
pub type GLbitfield = std::os::raw::c_uint;
pub type GLvoid = std::os::raw::c_void;
pub type GLbyte = std::os::raw::c_schar;
pub type GLshort = std::os::raw::c_short;
pub type GLint = std::os::raw::c_int;
pub type GLubyte = std::os::raw::c_uchar;
pub type GLushort = std::os::raw::c_ushort;
pub type GLuint = std::os::raw::c_uint;
pub type GLsizei = std::os::raw::c_int;
pub type GLfloat = std::os::raw::c_float;
pub type GLclampf = std::os::raw::c_float;
pub type GLdouble = std::os::raw::c_double;
pub type GLclampd = std::os::raw::c_double;

pub const GL_BOOLEAN_GL_FALSE: GLboolean = 0;
pub const GL_BOOLEAN_GL_TRUE: GLboolean = 1;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub enum GLType {
    GLByte = 0x1400,
    GLUnsignedByte = 0x1401,
    GLShort = 0x1402,
    GLUnsignedShort = 0x1403,
    GLInt = 0x1404,
    GLUnsignedInt = 0x1405,
    GLFloat = 0x1406,
    GL2Bytes = 0x1407,
    GL3Bytes = 0x1408,
    GL4Bytes = 0x1409,
    GLDouble = 0x140A,
}
pub const GL_BYTE: GLType = GLType::GLByte;
pub const GL_UNSIGNED_BYTE: GLType = GLType::GLUnsignedByte;
pub const GL_SHORT: GLType = GLType::GLShort;
pub const GL_UNSIGNED_SHORT: GLType = GLType::GLUnsignedByte;
pub const GL_INT: GLType = GLType::GLInt;
pub const GL_UNSIGNED_INT: GLType = GLType::GLUnsignedInt;
pub const GL_FLOAT: GLType = GLType::GLFloat;
pub const GL_2_BYTES: GLType = GLType::GL2Bytes;
pub const GL_3_BYTES: GLType = GLType::GL3Bytes;
pub const GL_4_BYTES: GLType = GLType::GL4Bytes;
pub const GL_DOUBLE: GLType = GLType::GLDouble;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub enum GLPrimitive {
    /* Primitives */
    GlPoints = 0x0000,
    GLLines = 0x0001,
    GLLineLoop = 0x0002,
    GLLineStrip = 0x0003,
    GLTriangles = 0x0004,
    GLTriangleStrip = 0x0005,
    GLTriangleFan = 0x0006,
    GLQuads = 0x0007,
    GLQuadStrip = 0x0008,
    GLPolygon = 0x0009,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub enum GLVertexArrays {
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
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub enum GLMatrixMode {
    GL_MATRIX_MODE = 0x0BA0,
    GL_MODELVIEW = 0x1700,
    GL_PROJECTION = 0x1701,
    GL_TEXTURE = 0x1702,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub enum GLPoints {
    GL_POINT_SMOOTH = 0x0B10,
    GL_POINT_SIZE = 0x0B11,
    GL_POINT_SIZE_GRANULARITY = 0x0B13,
    GL_POINT_SIZE_RANGE = 0x0B12,
}

pub enum GLLines {
    GL_LINE_SMOOTH = 0x0B20,
    GL_LINE_STIPPLE = 0x0B24,
    GL_LINE_STIPPLE_PATTERN = 0x0B25,
    GL_LINE_STIPPLE_REPEAT = 0x0B26,
    GL_LINE_WIDTH = 0x0B21,
    GL_LINE_WIDTH_GRANULARITY = 0x0B23,
    GL_LINE_WIDTH_RANGE = 0x0B22,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub enum GLPolygons {
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
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub enum GLDisplayLists {
    GL_COMPILE = 0x1300,
    GL_COMPILE_AND_EXECUTE = 0x1301,
    GL_LIST_BASE = 0x0B32,
    GL_LIST_INDEX = 0x0B33,
    GL_LIST_MODE = 0x0B30,
}

pub enum GLDepthBuffer {
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


/* User clipping planes */
pub const GL_CLIP_PLANE0: std::os::raw::c_int = 0x3000;
pub const GL_CLIP_PLANE1: std::os::raw::c_int = 0x3001;
pub const GL_CLIP_PLANE2: std::os::raw::c_int = 0x3002;
pub const GL_CLIP_PLANE3: std::os::raw::c_int = 0x3003;
pub const GL_CLIP_PLANE4: std::os::raw::c_int = 0x3004;
pub const GL_CLIP_PLANE5: std::os::raw::c_int = 0x3005;

/* Accumulation buffer */
pub const GL_ACCUM_RED_BITS: std::os::raw::c_int = 0x0D58;
pub const GL_ACCUM_GREEN_BITS: std::os::raw::c_int = 0x0D59;
pub const GL_ACCUM_BLUE_BITS: std::os::raw::c_int = 0x0D5A;
pub const GL_ACCUM_ALPHA_BITS: std::os::raw::c_int = 0x0D5B;
pub const GL_ACCUM_CLEAR_VALUE: std::os::raw::c_int = 0x0B80;
pub const GL_ACCUM: std::os::raw::c_int = 0x0100;
pub const GL_ADD: std::os::raw::c_int = 0x0104;
pub const GL_LOAD: std::os::raw::c_int = 0x0101;
pub const GL_MULT: std::os::raw::c_int = 0x0103;
pub const GL_RETURN: std::os::raw::c_int = 0x0102;

/* Alpha testing */
pub const GL_ALPHA_TEST: std::os::raw::c_int = 0x0BC0;
pub const GL_ALPHA_TEST_REF: std::os::raw::c_int = 0x0BC2;
pub const GL_ALPHA_TEST_FUNC: std::os::raw::c_int = 0x0BC1;

/* Blending */
pub const GL_BLEND: std::os::raw::c_int = 0x0BE2;
pub const GL_BLEND_SRC: std::os::raw::c_int = 0x0BE1;
pub const GL_BLEND_DST: std::os::raw::c_int = 0x0BE0;
pub const GL_ZERO: std::os::raw::c_int = 0;
pub const GL_ONE: std::os::raw::c_int = 1;
pub const GL_SRC_COLOR: std::os::raw::c_int = 0x0300;
pub const GL_ONE_MINUS_SRC_COLOR: std::os::raw::c_int = 0x0301;
pub const GL_SRC_ALPHA: std::os::raw::c_int = 0x0302;
pub const GL_ONE_MINUS_SRC_ALPHA: std::os::raw::c_int = 0x0303;
pub const GL_DST_ALPHA: std::os::raw::c_int = 0x0304;
pub const GL_ONE_MINUS_DST_ALPHA: std::os::raw::c_int = 0x0305;
pub const GL_DST_COLOR: std::os::raw::c_int = 0x0306;
pub const GL_ONE_MINUS_DST_COLOR: std::os::raw::c_int = 0x0307;
pub const GL_SRC_ALPHA_SATURATE: std::os::raw::c_int = 0x0308;

/* Render Mode */
pub const GL_FEEDBACK: std::os::raw::c_int = 0x1C01;
pub const GL_RENDER: std::os::raw::c_int = 0x1C00;
pub const GL_SELECT: std::os::raw::c_int = 0x1C02;

/* Feedback */
pub const GL_2D: std::os::raw::c_int = 0x0600;
pub const GL_3D: std::os::raw::c_int = 0x0601;
pub const GL_3D_COLOR: std::os::raw::c_int = 0x0602;
pub const GL_3D_COLOR_TEXTURE: std::os::raw::c_int = 0x0603;
pub const GL_4D_COLOR_TEXTURE: std::os::raw::c_int = 0x0604;
pub const GL_POINT_TOKEN: std::os::raw::c_int = 0x0701;
pub const GL_LINE_TOKEN: std::os::raw::c_int = 0x0702;
pub const GL_LINE_RESET_TOKEN: std::os::raw::c_int = 0x0707;
pub const GL_POLYGON_TOKEN: std::os::raw::c_int = 0x0703;
pub const GL_BITMAP_TOKEN: std::os::raw::c_int = 0x0704;
pub const GL_DRAW_PIXEL_TOKEN: std::os::raw::c_int = 0x0705;
pub const GL_COPY_PIXEL_TOKEN: std::os::raw::c_int = 0x0706;
pub const GL_PASS_THROUGH_TOKEN: std::os::raw::c_int = 0x0700;
pub const GL_FEEDBACK_BUFFER_POINTER: std::os::raw::c_int = 0x0DF0;
pub const GL_FEEDBACK_BUFFER_SIZE: std::os::raw::c_int = 0x0DF1;
pub const GL_FEEDBACK_BUFFER_TYPE: std::os::raw::c_int = 0x0DF2;

/* Selection */
pub const GL_SELECTION_BUFFER_POINTER: std::os::raw::c_int = 0x0DF3;
pub const GL_SELECTION_BUFFER_SIZE: std::os::raw::c_int = 0x0DF4;

/* Fog */
pub const GL_FOG: std::os::raw::c_int = 0x0B60;
pub const GL_FOG_MODE: std::os::raw::c_int = 0x0B65;
pub const GL_FOG_DENSITY: std::os::raw::c_int = 0x0B62;
pub const GL_FOG_COLOR: std::os::raw::c_int = 0x0B66;
pub const GL_FOG_INDEX: std::os::raw::c_int = 0x0B61;
pub const GL_FOG_START: std::os::raw::c_int = 0x0B63;
pub const GL_FOG_END: std::os::raw::c_int = 0x0B64;
pub const GL_LINEAR: std::os::raw::c_int = 0x2601;
pub const GL_EXP: std::os::raw::c_int = 0x0800;
pub const GL_EXP2: std::os::raw::c_int = 0x0801;

/* Logic Ops */
pub const GL_LOGIC_OP: std::os::raw::c_int = 0x0BF1;
pub const GL_INDEX_LOGIC_OP: std::os::raw::c_int = 0x0BF1;
pub const GL_COLOR_LOGIC_OP: std::os::raw::c_int = 0x0BF2;
pub const GL_LOGIC_OP_MODE: std::os::raw::c_int = 0x0BF0;
pub const GL_CLEAR: std::os::raw::c_int = 0x1500;
pub const GL_SET: std::os::raw::c_int = 0x150F;
pub const GL_COPY: std::os::raw::c_int = 0x1503;
pub const GL_COPY_INVERTED: std::os::raw::c_int = 0x150C;
pub const GL_NOOP: std::os::raw::c_int = 0x1505;
pub const GL_INVERT: std::os::raw::c_int = 0x150A;
pub const GL_AND: std::os::raw::c_int = 0x1501;
pub const GL_NAND: std::os::raw::c_int = 0x150E;
pub const GL_OR: std::os::raw::c_int = 0x1507;
pub const GL_NOR: std::os::raw::c_int = 0x1508;
pub const GL_XOR: std::os::raw::c_int = 0x1506;
pub const GL_EQUIV: std::os::raw::c_int = 0x1509;
pub const GL_AND_REVERSE: std::os::raw::c_int = 0x1502;
pub const GL_AND_INVERTED: std::os::raw::c_int = 0x1504;
pub const GL_OR_REVERSE: std::os::raw::c_int = 0x150B;
pub const GL_OR_INVERTED: std::os::raw::c_int = 0x150D;

/* Stencil */
pub const GL_STENCIL_BITS: std::os::raw::c_int = 0x0D57;
pub const GL_STENCIL_TEST: std::os::raw::c_int = 0x0B90;
pub const GL_STENCIL_CLEAR_VALUE: std::os::raw::c_int = 0x0B91;
pub const GL_STENCIL_FUNC: std::os::raw::c_int = 0x0B92;
pub const GL_STENCIL_VALUE_MASK: std::os::raw::c_int = 0x0B93;
pub const GL_STENCIL_FAIL: std::os::raw::c_int = 0x0B94;
pub const GL_STENCIL_PASS_DEPTH_FAIL: std::os::raw::c_int = 0x0B95;
pub const GL_STENCIL_PASS_DEPTH_PASS: std::os::raw::c_int = 0x0B96;
pub const GL_STENCIL_REF: std::os::raw::c_int = 0x0B97;
pub const GL_STENCIL_WRITEMASK: std::os::raw::c_int = 0x0B98;
pub const GL_STENCIL_INDEX: std::os::raw::c_int = 0x1901;
pub const GL_KEEP: std::os::raw::c_int = 0x1E00;
pub const GL_REPLACE: std::os::raw::c_int = 0x1E01;
pub const GL_INCR: std::os::raw::c_int = 0x1E02;
pub const GL_DECR: std::os::raw::c_int = 0x1E03;

/* Buffers, Pixel Drawing/Reading */
pub const GL_NONE: std::os::raw::c_int = 0;
pub const GL_LEFT: std::os::raw::c_int = 0x0406;
pub const GL_RIGHT: std::os::raw::c_int = 0x0407;
/*GL_FRONT					0x0404 */
/*GL_BACK					0x0405 */
/*GL_FRONT_AND_BACK				0x0408 */
pub const GL_FRONT_LEFT: std::os::raw::c_int = 0x0400;
pub const GL_FRONT_RIGHT: std::os::raw::c_int = 0x0401;
pub const GL_BACK_LEFT: std::os::raw::c_int = 0x0402;
pub const GL_BACK_RIGHT: std::os::raw::c_int = 0x0403;
pub const GL_AUX0: std::os::raw::c_int = 0x0409;
pub const GL_AUX1: std::os::raw::c_int = 0x040A;
pub const GL_AUX2: std::os::raw::c_int = 0x040B;
pub const GL_AUX3: std::os::raw::c_int = 0x040C;
pub const GL_COLOR_INDEX: std::os::raw::c_int = 0x1900;
pub const GL_RED: std::os::raw::c_int = 0x1903;
pub const GL_GREEN: std::os::raw::c_int = 0x1904;
pub const GL_BLUE: std::os::raw::c_int = 0x1905;
pub const GL_ALPHA: std::os::raw::c_int = 0x1906;
pub const GL_LUMINANCE: std::os::raw::c_int = 0x1909;
pub const GL_LUMINANCE_ALPHA: std::os::raw::c_int = 0x190A;
pub const GL_ALPHA_BITS: std::os::raw::c_int = 0x0D55;
pub const GL_RED_BITS: std::os::raw::c_int = 0x0D52;
pub const GL_GREEN_BITS: std::os::raw::c_int = 0x0D53;
pub const GL_BLUE_BITS: std::os::raw::c_int = 0x0D54;
pub const GL_INDEX_BITS: std::os::raw::c_int = 0x0D51;
pub const GL_SUBPIXEL_BITS: std::os::raw::c_int = 0x0D50;
pub const GL_AUX_BUFFERS: std::os::raw::c_int = 0x0C00;
pub const GL_READ_BUFFER: std::os::raw::c_int = 0x0C02;
pub const GL_DRAW_BUFFER: std::os::raw::c_int = 0x0C01;
pub const GL_DOUBLEBUFFER: std::os::raw::c_int = 0x0C32;
pub const GL_STEREO: std::os::raw::c_int = 0x0C33;
pub const GL_BITMAP: std::os::raw::c_int = 0x1A00;
pub const GL_COLOR: std::os::raw::c_int = 0x1800;
pub const GL_DEPTH: std::os::raw::c_int = 0x1801;
pub const GL_STENCIL: std::os::raw::c_int = 0x1802;
pub const GL_DITHER: std::os::raw::c_int = 0x0BD0;
pub const GL_RGB: std::os::raw::c_int = 0x1907;
pub const GL_RGBA: std::os::raw::c_int = 0x1908;

/* Implementation limits */
pub const GL_MAX_LIST_NESTING: std::os::raw::c_int = 0x0B31;
pub const GL_MAX_EVAL_ORDER: std::os::raw::c_int = 0x0D30;
pub const GL_MAX_LIGHTS: std::os::raw::c_int = 0x0D31;
pub const GL_MAX_CLIP_PLANES: std::os::raw::c_int = 0x0D32;
pub const GL_MAX_TEXTURE_SIZE: std::os::raw::c_int = 0x0D33;
pub const GL_MAX_PIXEL_MAP_TABLE: std::os::raw::c_int = 0x0D34;
pub const GL_MAX_ATTRIB_STACK_DEPTH: std::os::raw::c_int = 0x0D35;
pub const GL_MAX_MODELVIEW_STACK_DEPTH: std::os::raw::c_int = 0x0D36;
pub const GL_MAX_NAME_STACK_DEPTH: std::os::raw::c_int = 0x0D37;
pub const GL_MAX_PROJECTION_STACK_DEPTH: std::os::raw::c_int = 0x0D38;
pub const GL_MAX_TEXTURE_STACK_DEPTH: std::os::raw::c_int = 0x0D39;
pub const GL_MAX_VIEWPORT_DIMS: std::os::raw::c_int = 0x0D3A;
pub const GL_MAX_CLIENT_ATTRIB_STACK_DEPTH: std::os::raw::c_int = 0x0D3B;

/* Gets */
pub const GL_ATTRIB_STACK_DEPTH: std::os::raw::c_int = 0x0BB0;
pub const GL_CLIENT_ATTRIB_STACK_DEPTH: std::os::raw::c_int = 0x0BB1;
pub const GL_COLOR_CLEAR_VALUE: std::os::raw::c_int = 0x0C22;
pub const GL_COLOR_WRITEMASK: std::os::raw::c_int = 0x0C23;
pub const GL_CURRENT_INDEX: std::os::raw::c_int = 0x0B01;
pub const GL_CURRENT_COLOR: std::os::raw::c_int = 0x0B00;
pub const GL_CURRENT_NORMAL: std::os::raw::c_int = 0x0B02;
pub const GL_CURRENT_RASTER_COLOR: std::os::raw::c_int = 0x0B04;
pub const GL_CURRENT_RASTER_DISTANCE: std::os::raw::c_int = 0x0B09;
pub const GL_CURRENT_RASTER_INDEX: std::os::raw::c_int = 0x0B05;
pub const GL_CURRENT_RASTER_POSITION: std::os::raw::c_int = 0x0B07;
pub const GL_CURRENT_RASTER_TEXTURE_COORDS: std::os::raw::c_int = 0x0B06;
pub const GL_CURRENT_RASTER_POSITION_VALID: std::os::raw::c_int = 0x0B08;
pub const GL_CURRENT_TEXTURE_COORDS: std::os::raw::c_int = 0x0B03;
pub const GL_INDEX_CLEAR_VALUE: std::os::raw::c_int = 0x0C20;
pub const GL_INDEX_MODE: std::os::raw::c_int = 0x0C30;
pub const GL_INDEX_WRITEMASK: std::os::raw::c_int = 0x0C21;
pub const GL_MODELVIEW_MATRIX: std::os::raw::c_int = 0x0BA6;
pub const GL_MODELVIEW_STACK_DEPTH: std::os::raw::c_int = 0x0BA3;
pub const GL_NAME_STACK_DEPTH: std::os::raw::c_int = 0x0D70;
pub const GL_PROJECTION_MATRIX: std::os::raw::c_int = 0x0BA7;
pub const GL_PROJECTION_STACK_DEPTH: std::os::raw::c_int = 0x0BA4;
pub const GL_RENDER_MODE: std::os::raw::c_int = 0x0C40;
pub const GL_RGBA_MODE: std::os::raw::c_int = 0x0C31;
pub const GL_TEXTURE_MATRIX: std::os::raw::c_int = 0x0BA8;
pub const GL_TEXTURE_STACK_DEPTH: std::os::raw::c_int = 0x0BA5;
pub const GL_VIEWPORT: std::os::raw::c_int = 0x0BA2;

/* Evaluators */
pub const GL_AUTO_NORMAL: std::os::raw::c_int = 0x0D80;
pub const GL_MAP1_COLOR_4: std::os::raw::c_int = 0x0D90;
pub const GL_MAP1_INDEX: std::os::raw::c_int = 0x0D91;
pub const GL_MAP1_NORMAL: std::os::raw::c_int = 0x0D92;
pub const GL_MAP1_TEXTURE_COORD_1: std::os::raw::c_int = 0x0D93;
pub const GL_MAP1_TEXTURE_COORD_2: std::os::raw::c_int = 0x0D94;
pub const GL_MAP1_TEXTURE_COORD_3: std::os::raw::c_int = 0x0D95;
pub const GL_MAP1_TEXTURE_COORD_4: std::os::raw::c_int = 0x0D96;
pub const GL_MAP1_VERTEX_3: std::os::raw::c_int = 0x0D97;
pub const GL_MAP1_VERTEX_4: std::os::raw::c_int = 0x0D98;
pub const GL_MAP2_COLOR_4: std::os::raw::c_int = 0x0DB0;
pub const GL_MAP2_INDEX: std::os::raw::c_int = 0x0DB1;
pub const GL_MAP2_NORMAL: std::os::raw::c_int = 0x0DB2;
pub const GL_MAP2_TEXTURE_COORD_1: std::os::raw::c_int = 0x0DB3;
pub const GL_MAP2_TEXTURE_COORD_2: std::os::raw::c_int = 0x0DB4;
pub const GL_MAP2_TEXTURE_COORD_3: std::os::raw::c_int = 0x0DB5;
pub const GL_MAP2_TEXTURE_COORD_4: std::os::raw::c_int = 0x0DB6;
pub const GL_MAP2_VERTEX_3: std::os::raw::c_int = 0x0DB7;
pub const GL_MAP2_VERTEX_4: std::os::raw::c_int = 0x0DB8;
pub const GL_MAP1_GRID_DOMAIN: std::os::raw::c_int = 0x0DD0;
pub const GL_MAP1_GRID_SEGMENTS: std::os::raw::c_int = 0x0DD1;
pub const GL_MAP2_GRID_DOMAIN: std::os::raw::c_int = 0x0DD2;
pub const GL_MAP2_GRID_SEGMENTS: std::os::raw::c_int = 0x0DD3;
pub const GL_COEFF: std::os::raw::c_int = 0x0A00;
pub const GL_ORDER: std::os::raw::c_int = 0x0A01;
pub const GL_DOMAIN: std::os::raw::c_int = 0x0A02;

/* Hints */
pub const GL_PERSPECTIVE_CORRECTION_HINT: std::os::raw::c_int = 0x0C50;
pub const GL_POINT_SMOOTH_HINT: std::os::raw::c_int = 0x0C51;
pub const GL_LINE_SMOOTH_HINT: std::os::raw::c_int = 0x0C52;
pub const GL_POLYGON_SMOOTH_HINT: std::os::raw::c_int = 0x0C53;
pub const GL_FOG_HINT: std::os::raw::c_int = 0x0C54;
pub const GL_DONT_CARE: std::os::raw::c_int = 0x1100;
pub const GL_FASTEST: std::os::raw::c_int = 0x1101;
pub const GL_NICEST: std::os::raw::c_int = 0x1102;

/* Scissor box */
pub const GL_SCISSOR_BOX: std::os::raw::c_int = 0x0C10;
pub const GL_SCISSOR_TEST: std::os::raw::c_int = 0x0C11;

/* Pixel Mode / Transfer */
pub const GL_MAP_COLOR: std::os::raw::c_int = 0x0D10;
pub const GL_MAP_STENCIL: std::os::raw::c_int = 0x0D11;
pub const GL_INDEX_SHIFT: std::os::raw::c_int = 0x0D12;
pub const GL_INDEX_OFFSET: std::os::raw::c_int = 0x0D13;
pub const GL_RED_SCALE: std::os::raw::c_int = 0x0D14;
pub const GL_RED_BIAS: std::os::raw::c_int = 0x0D15;
pub const GL_GREEN_SCALE: std::os::raw::c_int = 0x0D18;
pub const GL_GREEN_BIAS: std::os::raw::c_int = 0x0D19;
pub const GL_BLUE_SCALE: std::os::raw::c_int = 0x0D1A;
pub const GL_BLUE_BIAS: std::os::raw::c_int = 0x0D1B;
pub const GL_ALPHA_SCALE: std::os::raw::c_int = 0x0D1C;
pub const GL_ALPHA_BIAS: std::os::raw::c_int = 0x0D1D;
pub const GL_DEPTH_SCALE: std::os::raw::c_int = 0x0D1E;
pub const GL_DEPTH_BIAS: std::os::raw::c_int = 0x0D1F;
pub const GL_PIXEL_MAP_S_TO_S_SIZE: std::os::raw::c_int = 0x0CB1;
pub const GL_PIXEL_MAP_I_TO_I_SIZE: std::os::raw::c_int = 0x0CB0;
pub const GL_PIXEL_MAP_I_TO_R_SIZE: std::os::raw::c_int = 0x0CB2;
pub const GL_PIXEL_MAP_I_TO_G_SIZE: std::os::raw::c_int = 0x0CB3;
pub const GL_PIXEL_MAP_I_TO_B_SIZE: std::os::raw::c_int = 0x0CB4;
pub const GL_PIXEL_MAP_I_TO_A_SIZE: std::os::raw::c_int = 0x0CB5;
pub const GL_PIXEL_MAP_R_TO_R_SIZE: std::os::raw::c_int = 0x0CB6;
pub const GL_PIXEL_MAP_G_TO_G_SIZE: std::os::raw::c_int = 0x0CB7;
pub const GL_PIXEL_MAP_B_TO_B_SIZE: std::os::raw::c_int = 0x0CB8;
pub const GL_PIXEL_MAP_A_TO_A_SIZE: std::os::raw::c_int = 0x0CB9;
pub const GL_PIXEL_MAP_S_TO_S: std::os::raw::c_int = 0x0C71;
pub const GL_PIXEL_MAP_I_TO_I: std::os::raw::c_int = 0x0C70;
pub const GL_PIXEL_MAP_I_TO_R: std::os::raw::c_int = 0x0C72;
pub const GL_PIXEL_MAP_I_TO_G: std::os::raw::c_int = 0x0C73;
pub const GL_PIXEL_MAP_I_TO_B: std::os::raw::c_int = 0x0C74;
pub const GL_PIXEL_MAP_I_TO_A: std::os::raw::c_int = 0x0C75;
pub const GL_PIXEL_MAP_R_TO_R: std::os::raw::c_int = 0x0C76;
pub const GL_PIXEL_MAP_G_TO_G: std::os::raw::c_int = 0x0C77;
pub const GL_PIXEL_MAP_B_TO_B: std::os::raw::c_int = 0x0C78;
pub const GL_PIXEL_MAP_A_TO_A: std::os::raw::c_int = 0x0C79;
pub const GL_PACK_ALIGNMENT: std::os::raw::c_int = 0x0D05;
pub const GL_PACK_LSB_FIRST: std::os::raw::c_int = 0x0D01;
pub const GL_PACK_ROW_LENGTH: std::os::raw::c_int = 0x0D02;
pub const GL_PACK_SKIP_PIXELS: std::os::raw::c_int = 0x0D04;
pub const GL_PACK_SKIP_ROWS: std::os::raw::c_int = 0x0D03;
pub const GL_PACK_SWAP_BYTES: std::os::raw::c_int = 0x0D00;
pub const GL_UNPACK_ALIGNMENT: std::os::raw::c_int = 0x0CF5;
pub const GL_UNPACK_LSB_FIRST: std::os::raw::c_int = 0x0CF1;
pub const GL_UNPACK_ROW_LENGTH: std::os::raw::c_int = 0x0CF2;
pub const GL_UNPACK_SKIP_PIXELS: std::os::raw::c_int = 0x0CF4;
pub const GL_UNPACK_SKIP_ROWS: std::os::raw::c_int = 0x0CF3;
pub const GL_UNPACK_SWAP_BYTES: std::os::raw::c_int = 0x0CF0;
pub const GL_ZOOM_X: std::os::raw::c_int = 0x0D16;
pub const GL_ZOOM_Y: std::os::raw::c_int = 0x0D17;

/* Texture mapping */
pub const GL_TEXTURE_ENV: std::os::raw::c_int = 0x2300;
pub const GL_TEXTURE_ENV_MODE: std::os::raw::c_int = 0x2200;
pub const GL_TEXTURE_1D: std::os::raw::c_int = 0x0DE0;
pub const GL_TEXTURE_2D: std::os::raw::c_int = 0x0DE1;
pub const GL_TEXTURE_WRAP_S: std::os::raw::c_int = 0x2802;
pub const GL_TEXTURE_WRAP_T: std::os::raw::c_int = 0x2803;
pub const GL_TEXTURE_MAG_FILTER: std::os::raw::c_int = 0x2800;
pub const GL_TEXTURE_MIN_FILTER: std::os::raw::c_int = 0x2801;
pub const GL_TEXTURE_ENV_COLOR: std::os::raw::c_int = 0x2201;
pub const GL_TEXTURE_GEN_S: std::os::raw::c_int = 0x0C60;
pub const GL_TEXTURE_GEN_T: std::os::raw::c_int = 0x0C61;
pub const GL_TEXTURE_GEN_R: std::os::raw::c_int = 0x0C62;
pub const GL_TEXTURE_GEN_Q: std::os::raw::c_int = 0x0C63;
pub const GL_TEXTURE_GEN_MODE: std::os::raw::c_int = 0x2500;
pub const GL_TEXTURE_BORDER_COLOR: std::os::raw::c_int = 0x1004;
pub const GL_TEXTURE_WIDTH: std::os::raw::c_int = 0x1000;
pub const GL_TEXTURE_HEIGHT: std::os::raw::c_int = 0x1001;
pub const GL_TEXTURE_BORDER: std::os::raw::c_int = 0x1005;
pub const GL_TEXTURE_COMPONENTS: std::os::raw::c_int = 0x1003;
pub const GL_TEXTURE_RED_SIZE: std::os::raw::c_int = 0x805C;
pub const GL_TEXTURE_GREEN_SIZE: std::os::raw::c_int = 0x805D;
pub const GL_TEXTURE_BLUE_SIZE: std::os::raw::c_int = 0x805E;
pub const GL_TEXTURE_ALPHA_SIZE: std::os::raw::c_int = 0x805F;
pub const GL_TEXTURE_LUMINANCE_SIZE: std::os::raw::c_int = 0x8060;
pub const GL_TEXTURE_INTENSITY_SIZE: std::os::raw::c_int = 0x8061;
pub const GL_NEAREST_MIPMAP_NEAREST: std::os::raw::c_int = 0x2700;
pub const GL_NEAREST_MIPMAP_LINEAR: std::os::raw::c_int = 0x2702;
pub const GL_LINEAR_MIPMAP_NEAREST: std::os::raw::c_int = 0x2701;
pub const GL_LINEAR_MIPMAP_LINEAR: std::os::raw::c_int = 0x2703;
pub const GL_OBJECT_LINEAR: std::os::raw::c_int = 0x2401;
pub const GL_OBJECT_PLANE: std::os::raw::c_int = 0x2501;
pub const GL_EYE_LINEAR: std::os::raw::c_int = 0x2400;
pub const GL_EYE_PLANE: std::os::raw::c_int = 0x2502;
pub const GL_SPHERE_MAP: std::os::raw::c_int = 0x2402;
pub const GL_DECAL: std::os::raw::c_int = 0x2101;
pub const GL_MODULATE: std::os::raw::c_int = 0x2100;
pub const GL_NEAREST: std::os::raw::c_int = 0x2600;
pub const GL_REPEAT: std::os::raw::c_int = 0x2901;
pub const GL_CLAMP: std::os::raw::c_int = 0x2900;
pub const GL_S: std::os::raw::c_int = 0x2000;
pub const GL_T: std::os::raw::c_int = 0x2001;
pub const GL_R: std::os::raw::c_int = 0x2002;
pub const GL_Q: std::os::raw::c_int = 0x2003;

/* Utility */
pub const GL_VENDOR: std::os::raw::c_int = 0x1F00;
pub const GL_RENDERER: std::os::raw::c_int = 0x1F01;
pub const GL_VERSION: std::os::raw::c_int = 0x1F02;
pub const GL_EXTENSIONS: std::os::raw::c_int = 0x1F03;

/* Errors */
pub const GL_NO_ERROR: std::os::raw::c_uint = 0;
pub const GL_INVALID_ENUM: std::os::raw::c_uint = 0x0500;
pub const GL_INVALID_VALUE: std::os::raw::c_uint = 0x0501;
pub const GL_INVALID_OPERATION: std::os::raw::c_uint = 0x0502;
pub const GL_STACK_OVERFLOW: std::os::raw::c_uint = 0x0503;
pub const GL_STACK_UNDERFLOW: std::os::raw::c_uint = 0x0504;
pub const GL_OUT_OF_MEMORY: std::os::raw::c_uint = 0x0505;
pub const GL_INVALID_FRAMEBUFFER_OPERATION: std::os::raw::c_uint = 0x0506;
pub const GL_TABLE_TOO_LARGE: std::os::raw::c_uint = 0x0507;


/* glPush/PopAttrib bits */
pub const GL_CURRENT_BIT: std::os::raw::c_int = 0x00000001;
pub const GL_POINT_BIT: std::os::raw::c_int = 0x00000002;
pub const GL_LINE_BIT: std::os::raw::c_int = 0x00000004;
pub const GL_POLYGON_BIT: std::os::raw::c_int = 0x00000008;
pub const GL_POLYGON_STIPPLE_BIT: std::os::raw::c_int = 0x00000010;
pub const GL_PIXEL_MODE_BIT: std::os::raw::c_int = 0x00000020;
pub const GL_LIGHTING_BIT: std::os::raw::c_int = 0x00000040;
pub const GL_FOG_BIT: std::os::raw::c_int = 0x00000080;
pub const GL_DEPTH_BUFFER_BIT: std::os::raw::c_int = 0x00000100;
pub const GL_ACCUM_BUFFER_BIT: std::os::raw::c_int = 0x00000200;
pub const GL_STENCIL_BUFFER_BIT: std::os::raw::c_int = 0x00000400;
pub const GL_VIEWPORT_BIT: std::os::raw::c_int = 0x00000800;
pub const GL_TRANSFORM_BIT: std::os::raw::c_int = 0x00001000;
pub const GL_ENABLE_BIT: std::os::raw::c_int = 0x00002000;
pub const GL_COLOR_BUFFER_BIT: std::os::raw::c_int = 0x00004000;
pub const GL_HINT_BIT: std::os::raw::c_int = 0x00008000;
pub const GL_EVAL_BIT: std::os::raw::c_int = 0x00010000;
pub const GL_LIST_BIT: std::os::raw::c_int = 0x00020000;
pub const GL_TEXTURE_BIT: std::os::raw::c_int = 0x00040000;
pub const GL_SCISSOR_BIT: std::os::raw::c_int = 0x00080000;
pub const GL_ALL_ATTRIB_BITS: std::os::raw::c_int = -1;


/* OpenGL 1.1 */
pub const GL_PROXY_TEXTURE_1D: std::os::raw::c_int = 0x8063;
pub const GL_PROXY_TEXTURE_2D: std::os::raw::c_int = 0x8064;
pub const GL_TEXTURE_PRIORITY: std::os::raw::c_int = 0x8066;
pub const GL_TEXTURE_RESIDENT: std::os::raw::c_int = 0x8067;
pub const GL_TEXTURE_BINDING_1D: std::os::raw::c_int = 0x8068;
pub const GL_TEXTURE_BINDING_2D: std::os::raw::c_int = 0x8069;
pub const GL_TEXTURE_INTERNAL_FORMAT: std::os::raw::c_int = 0x1003;
pub const GL_ALPHA4: std::os::raw::c_int = 0x803B;
pub const GL_ALPHA8: std::os::raw::c_int = 0x803C;
pub const GL_ALPHA12: std::os::raw::c_int = 0x803D;
pub const GL_ALPHA16: std::os::raw::c_int = 0x803E;
pub const GL_LUMINANCE4: std::os::raw::c_int = 0x803F;
pub const GL_LUMINANCE8: std::os::raw::c_int = 0x8040;
pub const GL_LUMINANCE12: std::os::raw::c_int = 0x8041;
pub const GL_LUMINANCE16: std::os::raw::c_int = 0x8042;
pub const GL_LUMINANCE4_ALPHA4: std::os::raw::c_int = 0x8043;
pub const GL_LUMINANCE6_ALPHA2: std::os::raw::c_int = 0x8044;
pub const GL_LUMINANCE8_ALPHA8: std::os::raw::c_int = 0x8045;
pub const GL_LUMINANCE12_ALPHA4: std::os::raw::c_int = 0x8046;
pub const GL_LUMINANCE12_ALPHA12: std::os::raw::c_int = 0x8047;
pub const GL_LUMINANCE16_ALPHA16: std::os::raw::c_int = 0x8048;
pub const GL_INTENSITY: std::os::raw::c_int = 0x8049;
pub const GL_INTENSITY4: std::os::raw::c_int = 0x804A;
pub const GL_INTENSITY8: std::os::raw::c_int = 0x804B;
pub const GL_INTENSITY12: std::os::raw::c_int = 0x804C;
pub const GL_INTENSITY16: std::os::raw::c_int = 0x804D;
pub const GL_R3_G3_B2: std::os::raw::c_int = 0x2A10;
pub const GL_RGB4: std::os::raw::c_int = 0x804F;
pub const GL_RGB5: std::os::raw::c_int = 0x8050;
pub const GL_RGB8: std::os::raw::c_int = 0x8051;
pub const GL_RGB10: std::os::raw::c_int = 0x8052;
pub const GL_RGB12: std::os::raw::c_int = 0x8053;
pub const GL_RGB16: std::os::raw::c_int = 0x8054;
pub const GL_RGBA2: std::os::raw::c_int = 0x8055;
pub const GL_RGBA4: std::os::raw::c_int = 0x8056;
pub const GL_RGB5_A1: std::os::raw::c_int = 0x8057;
pub const GL_RGBA8: std::os::raw::c_int = 0x8058;
pub const GL_RGB10_A2: std::os::raw::c_int = 0x8059;
pub const GL_RGBA12: std::os::raw::c_int = 0x805A;
pub const GL_RGBA16: std::os::raw::c_int = 0x805B;
pub const GL_CLIENT_PIXEL_STORE_BIT: std::os::raw::c_int = 0x00000001;
pub const GL_CLIENT_VERTEX_ARRAY_BIT: std::os::raw::c_int = 0x00000002;
pub const GL_ALL_CLIENT_ATTRIB_BITS: std::os::raw::c_uint = 0xFFFFFFFF;
pub const GL_CLIENT_ALL_ATTRIB_BITS: std::os::raw::c_uint = 0xFFFFFFFF;






pub struct GLContext{
    #[cfg(unix)]
    context: x11::glx::GLXContext,
    #[cfg(target_os="windows")]
    context: windows::Win32::Graphics::OpenGL::HGLRC,
    #[cfg(target_os="windows")]
    device_context: Option<windows::Win32::Graphics::Gdi::HDC>,
    #[cfg(target_os="macos")]
    context: (),
    #[cfg(target_os="ios")]
    context: (),
    #[cfg(target_os="android")]
    context: ()

}
impl  GLContext {
    #[cfg(unix)]
    pub fn new(context: x11::glx::GLXContext) -> Self {
        Self{context}
    }
    #[cfg(windows)]
    pub fn new(context: windows::Win32::Graphics::OpenGL::HGLRC,device_context:Option<windows::Win32::Graphics::Gdi::HDC> ) -> Self {
        Self{context,device_context}
    }
}



#[cfg(windows)]
pub mod wgl {
   

    pub fn get_last_error() -> Option<String> {
        let maybe_error = windows::core::Error::from_win32();
        if maybe_error.code().is_ok() {
            None
        }
        else {
            Some(maybe_error.to_string())
        }
    }
    pub fn wgl_get_current_context() -> Option<windows::Win32::Graphics::OpenGL::HGLRC> {
        let ctx = unsafe {windows::Win32::Graphics::OpenGL::wglGetCurrentContext()};
        if ctx.is_invalid() {
            return None;
        }
        else  {
            return Some(ctx);
        }
    }
    // this reqiures a HDC (handle to device context)
    pub fn wgl_swap_buffers(hdc: windows::Win32::Graphics::Gdi::HDC) ->  Result<(),String> {
        if hdc.is_invalid() {
            return Err("The device handle provided to wgl_swap_buffers is invalid.".to_string())
        }
        unsafe {windows::Win32::Graphics::OpenGL::SwapBuffers(hdc)}.map_err(|e|e.to_string())
    }
    pub fn wgl_get_current_device_context() -> Result<windows::Win32::Graphics::Gdi::HDC,String> {
        // dispose of the last error before calling this function
        let _ = windows::core::Error::from_win32();
        // get the current context
        let dc = unsafe {windows::Win32::Graphics::OpenGL::wglGetCurrentDC()};
        if dc.is_invalid() {
            return Err("Either there is no active Win32 Device Context, or the current Device Context is invalid".to_string());
        }
        // attempt to check for an error
        let err = windows::core::Error::from_win32();
        if err.code().is_err() {
            return Err(err.to_string())
        }
        // othrwise return the dc
        Ok(dc)
    }
}
#[cfg(unix)]
pub mod glx {
    use x11::glx::{GLXContext,GLXDrawable, __GLXcontextRec};
    use x11::xlib::{Bool, Display, XVisualInfo};
    
    #[link(name="libGL",kind="dylib")]
    unsafe extern "C" {
        unsafe fn glXChooseVisual(dpy: *mut Display,screen:std::ffi::c_int, attrib_list: *const std::ffi::c_int) -> x11::xlib::XVisualInfo;
        unsafe fn glXCreateContext(dpy: *mut Display,visual_info: XVisualInfo, direct: Bool) -> GLXContext;
        unsafe fn glXMakeCurrent(dpy: *mut Display, drawable: GLXDrawable,ctx: GLXContext);
        unsafe fn glXSwapBuffers(dpy: *mut Display,drawable: GLXDrawable) -> Bool;
        unsafe fn glXGetCurrentContext() -> GLXContext;
    }

    pub fn glx_make_current(dpy: &mut Display, drawable: GLXDrawable, ctx: GLXContext) {
        unsafe {glXMakeCurrent(std::ptr::from_mut(dpy), drawable, ctx);}
    }
    pub fn glx_get_current_context() -> Option<x11::glx::GLXContext> {
        let ctx = unsafe {glXGetCurrentContext()};
        if ctx.is_null() && !ctx.is_aligned() {
            return None;
        }
        else {
            return Some(ctx);
        }
        
    }
}

#[cfg(target_os="macos")]
pub mod agl {

}

pub fn gl_get_current_context() -> Option<GLContext> {
    #[cfg(target_os="macos")] {
        compile_error!("TODO: macos")
    }
    #[cfg(target_os="windows")] {
        let device_context = wgl::wgl_get_current_device_context().ok();
        wgl::wgl_get_current_context().map(|ctx| GLContext::new(ctx,device_context))
    }
    #[cfg(unix)] {
        match glx::glx_get_current_context() {
            Some(ctx) => {Some(GLContext::new(ctx))},
            None=>None
        }

    }
}

pub fn gl_swap_buffers(context: &mut GLContext) -> Result<(),String> {
        #[cfg(target_os="macos")] {
        compile_error!("TODO: macos")
    }
    #[cfg(target_os="windows")] {
        // attempts to swap the buffers using wglGetCurrentDC. if there is no device context or opengl context, this will fail
        let mut dc = match context.device_context {
            Some(dc)=>dc,
            None => wgl::wgl_get_current_device_context()?
        };
        wgl::wgl_swap_buffers(dc)
    }
    #[cfg(unix)] {
        match glx::glx_get_current_context() {
            Some(ctx) => {Some(GLContext{context:ctx})},
            None=>None
        }

    }
}





// core set of opengl functions. required for opengl to work at all. define them as externs

pub mod core {
    // use crate::gfx::opengl::GL_INVALID_ENUM;

    use crate::gfx::opengl::{GL_INVALID_FRAMEBUFFER_OPERATION, GL_OUT_OF_MEMORY, GL_STACK_OVERFLOW};

    use super::{GL_NO_ERROR,GL_INVALID_ENUM,GL_INVALID_VALUE,GL_INVALID_OPERATION};
    use super::GLenum;
   
    #[allow(non_snake_case)]
    #[cfg_attr(target_os="windows", link(name="OpenGL32.lib",kind="static"))]
    #[cfg_attr(unix,link(name="libGL",kind="dylib"))]
    unsafe extern "C" {
        
        unsafe fn glGetError() -> GLenum;
        unsafe fn glBegin(mode: GLenum) -> ();
        unsafe fn glEnd() -> ();
        unsafe fn glClear() -> ();
    }
    pub fn gl_get_error() -> Result<(),String> {
        let r#enum = unsafe {glGetError()};
        match r#enum {
            GL_NO_ERROR => {Ok(())},
            GL_INVALID_ENUM => {Err("GL_INVALID_ENUM".to_string())},
            GL_INVALID_VALUE => {Err("GL_INVALID_VALUE".to_string())},
            GL_INVALID_OPERATION =>{Err("GL_INVALID_OPERATION".to_string())},
            GL_STACK_OVERFLOW=>(Err("GL_STACK_UNDERFLOW".to_string())),
            GL_OUT_OF_MEMORY=>Err("GL_OUT_OF_MEMORY".to_string()),
            GL_INVALID_FRAMEBUFFER_OPERATION=>{Err("".to_string())},
            unknown=>{Err(format!("Opengl returned an unknown error code: {unknown:#x}"))}
        }
    }

    pub fn gl_begin(mode: super::GLPrimitive) -> Result<(),String> {
        unsafe {glBegin(mode as GLenum);}
        gl_get_error()
    }
    pub fn gl_end() -> Result<(),String> {
        unsafe {glEnd();}
        gl_get_error()

    }
}


pub type PFN_glClearIndex = Option<unsafe extern "C" fn( c: GLfloat )->()>;




pub type PFN_glClearColor = Option<unsafe extern "C" fn( red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf )->()>;

pub type PFN_glClear = Option<unsafe extern "C" fn( mask: GLbitfield )->()>;

pub type PFN_glIndexMask = Option<unsafe extern "C" fn( mask: GLuint )->()>;

pub type PFN_glColorMask = Option<unsafe extern "C" fn( red:GLboolean, green:GLboolean, blue: GLboolean, alpha: GLboolean )->()>;

pub type PFN_glAlphaFunc = Option<unsafe extern "C" fn( func: GLenum, r#ref: GLclampf )->()>;

pub type PFN_glBlendFunc = Option<unsafe extern "C" fn( sfactor: GLenum, dfactor: GLenum )->()>;

pub type PFN_glLogicOp = Option<unsafe extern "C" fn( opcode: GLenum )->()>;

pub type PFN_glCullFace = Option<unsafe extern "C" fn( mode: GLenum )->()>;

pub type PFN_glFrontFace = Option<unsafe extern "C" fn( mode: GLenum )->()>;

pub type PFN_glPointSize = Option<unsafe extern "C" fn( size: GLfloat )->()>;

pub type PFN_glLineWidth = Option<unsafe extern "C" fn( width: GLfloat )->()>;

pub type PFN_glLineStipple = Option<unsafe extern "C" fn( factor: GLint, pattern: GLushort )->()>;

pub type PFN_glPolygonMode = Option<unsafe extern "C" fn( face: GLenum, mode: GLenum )->()>;

pub type PFN_glPolygonOffset = Option<unsafe extern "C" fn( factor: GLfloat, units: GLfloat )->()>;

pub type PFN_glPolygonStipple = Option<unsafe extern "C" fn( mask: *const GLubyte )->()>;

pub type PFN_glGetPolygonStipple = Option<unsafe extern "C" fn( mask: *const GLubyte )->()>;

pub type PFN_glEdgeFlag = Option<unsafe extern "C" fn( flag: GLboolean )->()>;

pub type PFN_glEdgeFlagv = Option<unsafe extern "C" fn( flag: *const GLboolean )->()>;

pub type PFN_glScissor = Option<unsafe extern "C" fn( x: GLint, y: GLint, width: GLsizei, height: GLsizei)->()>;

pub type PFN_glClipPlane = Option<unsafe extern "C" fn( plane: GLenum, equation: *const GLdouble )->()>;

pub type PFN_glGetClipPlane = Option<unsafe extern "C" fn( plane: GLenum, equation: *mut GLdouble )->()>;

pub type PFN_glDrawBuffer = Option<unsafe extern "C" fn( mode: GLenum )->()>;

pub type PFN_glReadBuffer = Option<unsafe extern "C" fn( mode: GLenum )->()>;

pub type PFN_glEnable = Option<unsafe extern "C" fn( cap: GLenum )->()>;

pub type PFN_glDisable = Option<unsafe extern "C" fn( cap: GLenum )->()>;

pub type PFN_glIsEnabled = Option<unsafe extern "C" fn( cap: GLenum ) -> GLboolean>;


pub type PFN_glEnableClientState = Option<unsafe extern "C" fn( cap: GLenum )->()>;  /* 1.1 */

pub type PFN_glDisableClientState = Option<unsafe extern "C" fn( cap: GLenum )->()>;  /* 1.1 */


pub type PFN_glGetBooleanv = Option<unsafe extern "C" fn( pname: GLenum, params: *mut GLboolean )->()>;

pub type PFN_glGetDoublev = Option<unsafe extern "C" fn( pname: GLenum, params: *mut GLdouble )->()>;

pub type PFN_glGetFloatv = Option<unsafe extern "C" fn( pname: GLenum, params: *mut GLfloat )->()>;

pub type PFN_glGetIntegerv = Option<unsafe extern "C" fn( pname: GLenum, params: *mut GLint )->()>;


pub type PFN_glPushAttrib = Option<unsafe extern "C" fn( mask: GLbitfield )->()>;

pub type PFN_glPopAttrib = Option<unsafe extern "C" fn()->()>;


pub type PFN_glPushClientAttrib = Option<unsafe extern "C" fn( mask: GLbitfield )->()>;  /* 1.1 */

pub type PFN_glPopClientAttrib = Option<unsafe extern "C" fn()->()>;  /* 1.1 */


pub type PFN_glRenderMode = Option<unsafe extern "C" fn( mode: GLenum ) -> GLint>;



pub type PFN_glGetString = Option<unsafe extern "C" fn(name: GLenum) -> *const GLubyte>;


pub type PFN_glFinish = Option<unsafe extern "C" fn()->()>;

pub type PFN_glFlush = Option<unsafe extern "C" fn()->()>;

pub type PFN_glHint = Option<unsafe extern "C" fn( target: GLenum, mode: GLenum )->()>;


/*
 * Depth Buffer
 */

pub type PFN_glClearDepth = Option<unsafe extern "C" fn( depth: GLclampd  )->()>;

pub type PFN_glDepthFunc = Option<unsafe extern "C" fn( func: GLenum )->()>;

pub type PFN_glDepthMask = Option<unsafe extern "C" fn( flag: GLboolean )->()>;

pub type PFN_glDepthRange = Option<unsafe extern "C" fn( near_val: GLclampd, far_val: GLclampd)->()>;


/*
 * Accumulation Buffer
 */

pub type PFN_glClearAccum = Option<unsafe extern "C" fn( red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat )->()>;

pub type PFN_glAccum = Option<unsafe extern "C" fn( op: GLenum, value: GLfloat )->()>;


/*
 * Transformation
 */

pub type PFN_glMatrixMode = Option<unsafe extern "C" fn( mode: GLenum )->()>;

pub type PFN_glOrtho = Option<unsafe extern "C" fn( left: GLdouble, right: GLdouble,
                                 bottom: GLdouble, top: GLdouble,
                                 near_val: GLdouble, far_val: GLdouble )->()>;

pub type PFN_glFrustum = Option<unsafe extern "C" fn( left: GLdouble, right: GLdouble,
                                   bottom: GLdouble, top: GLdouble,
                                   near_val: GLdouble, far_val: GLdouble )->()>;

pub type PFN_glViewport = Option<unsafe extern "C" fn( x: GLint, y: GLint,
                                    width: GLsizei, height: GLsizei )->()>;

pub type PFN_glPushMatrix = Option<unsafe extern "C" fn()->()>;

pub type PFN_glPopMatrix = Option<unsafe extern "C" fn()->()>;

pub type PFN_glLoadIdentity = Option<unsafe extern "C" fn()->()>;

pub type PFN_glLoadMatrixd = Option<unsafe extern "C" fn( m: *const GLdouble )->()>;
pub type PFN_glLoadMatrixf = Option<unsafe extern "C" fn( m: *const GLfloat )->()>;

pub type PFN_glMultMatrixd = Option<unsafe extern "C" fn( m: *const GLdouble )->()>;
pub type PFN_glMultMatrixf = Option<unsafe extern "C" fn( m: *const GLfloat )->()>;

pub type PFN_glRotated = Option<unsafe extern "C" fn( angle: GLdouble,
                                   x: GLdouble, y: GLdouble, z: GLdouble )->()>;
pub type PFN_glRotatef = Option<unsafe extern "C" fn( angle: GLfloat,
                                   x: GLfloat, y: GLfloat, z: GLfloat )->()>;

pub type PFN_glScaled = Option<unsafe extern "C" fn( x: GLdouble, y: GLdouble, z: GLdouble )->()>;
pub type PFN_glScalef = Option<unsafe extern "C" fn( x: GLfloat, y: GLfloat, z: GLfloat )->()>;

pub type PFN_glTranslated = Option<unsafe extern "C" fn( x: GLdouble, y: GLdouble, z: GLdouble )->()>;
pub type PFN_glTranslatef = Option<unsafe extern "C" fn( x: GLfloat, y: GLfloat, z: GLfloat )->()>;


/*
 * Display Lists
 */

pub type PFN_glIsList = Option<unsafe extern "C" fn( list: GLuint ) -> GLboolean>;

pub type PFN_glDeleteLists = Option<unsafe extern "C" fn( list: GLuint, range: GLsizei )->()>;

pub type PFN_glGenLists = Option<unsafe extern "C" fn( range: GLsizei ) -> GLuint>;

pub type PFN_glNewList = Option<unsafe extern "C" fn( list: GLuint, mode: GLenum )->()>;

pub type PFN_glEndList = Option<unsafe extern "C" fn()->()>;

pub type PFN_glCallList = Option<unsafe extern "C" fn( list: GLuint )->()>;

pub type PFN_glCallLists = Option<unsafe extern "C" fn( n: GLsizei, r#type: GLenum,
                                     lists: *const GLvoid )->()>;

pub type PFN_glListBase = Option<unsafe extern "C" fn( base: GLuint )->()>;


/*
 * Drawing Functions
 */

pub type PFN_glBegin = Option<unsafe extern "C" fn( mode: GLenum )->()>;

pub type PFN_glEnd = Option<unsafe extern "C" fn()->()>;


pub type PFN_glVertex2d = Option<unsafe extern "C" fn( x: GLdouble, y: GLdouble )->()>;
pub type PFN_glVertex2f = Option<unsafe extern "C" fn( x: GLfloat, y: GLfloat )->()>;
pub type PFN_glVertex2i = Option<unsafe extern "C" fn( x: GLint, y: GLint )->()>;
pub type PFN_glVertex2s = Option<unsafe extern "C" fn( x:GLshort, y:GLshort )->()>;

pub type PFN_glVertex3d = Option<unsafe extern "C" fn( x: GLdouble, y: GLdouble, z: GLdouble )->()>;
pub type PFN_glVertex3f = Option<unsafe extern "C" fn( x: GLfloat, y: GLfloat, z: GLfloat )->()>;
pub type PFN_glVertex3i = Option<unsafe extern "C" fn( x: GLint, y: GLint, z: GLint )->()>;
pub type PFN_glVertex3s = Option<unsafe extern "C" fn( x:GLshort, y:GLshort, z:GLshort )->()>;

pub type PFN_glVertex4d = Option<unsafe extern "C" fn( x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble )->()>;
pub type PFN_glVertex4f = Option<unsafe extern "C" fn( x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat )->()>;
pub type PFN_glVertex4i = Option<unsafe extern "C" fn( x: GLint, y: GLint, z: GLint, w: GLint )->()>;
pub type PFN_glVertex4s = Option<unsafe extern "C" fn( x:GLshort, y:GLshort, z:GLshort, w:GLshort )->()>;

pub type PFN_glVertex2dv = Option<unsafe extern "C" fn( v: *const GLdouble )->()>;
pub type PFN_glVertex2fv = Option<unsafe extern "C" fn( v: *const GLfloat )->()>;
pub type PFN_glVertex2iv = Option<unsafe extern "C" fn( v: *const GLint )->()>;
pub type PFN_glVertex2sv = Option<unsafe extern "C" fn( v: *const GLshort )->()>;

pub type PFN_glVertex3dv = Option<unsafe extern "C" fn( v: *const GLdouble )->()>;
pub type PFN_glVertex3fv = Option<unsafe extern "C" fn( v: *const GLfloat )->()>;
pub type PFN_glVertex3iv = Option<unsafe extern "C" fn( v: *const GLint )->()>;
pub type PFN_glVertex3sv = Option<unsafe extern "C" fn( v: *const GLshort )->()>;

pub type PFN_glVertex4dv = Option<unsafe extern "C" fn( v: *const GLdouble )->()>;
pub type PFN_glVertex4fv = Option<unsafe extern "C" fn( v: *const GLfloat )->()>;
pub type PFN_glVertex4iv = Option<unsafe extern "C" fn( v: *const GLint )->()>;
pub type PFN_glVertex4sv = Option<unsafe extern "C" fn( v: *const GLshort )->()>;


pub type PFN_glNormal3b = Option<unsafe extern "C" fn( nx:GLbyte, ny:GLbyte, nz:GLbyte )->()>;
pub type PFN_glNormal3d = Option<unsafe extern "C" fn( nx: GLdouble, ny: GLdouble, nz: GLdouble )->()>;
pub type PFN_glNormal3f = Option<unsafe extern "C" fn( nx: GLfloat, ny: GLfloat, nz: GLfloat )->()>;
pub type PFN_glNormal3i = Option<unsafe extern "C" fn( nx: GLint, ny: GLint, nz: GLint )->()>;
pub type PFN_glNormal3s = Option<unsafe extern "C" fn( nx:GLshort, ny:GLshort, nz:GLshort )->()>;

pub type PFN_glNormal3bv = Option<unsafe extern "C" fn( v: *const GLbyte )->()>;
pub type PFN_glNormal3dv = Option<unsafe extern "C" fn( v: *const GLdouble )->()>;
pub type PFN_glNormal3fv = Option<unsafe extern "C" fn( v: *const GLfloat )->()>;
pub type PFN_glNormal3iv = Option<unsafe extern "C" fn( v: *const GLint )->()>;
pub type PFN_glNormal3sv = Option<unsafe extern "C" fn( v: *const GLshort )->()>;


pub type PFN_glIndexd = Option<unsafe extern "C" fn( c: GLdouble )->()>;
pub type PFN_glIndexf = Option<unsafe extern "C" fn( c: GLfloat )->()>;
pub type PFN_glIndexi = Option<unsafe extern "C" fn( c: GLint )->()>;
pub type PFN_glIndexs = Option<unsafe extern "C" fn( c:GLshort )->()>;
pub type PFN_glIndexub = Option<unsafe extern "C" fn( c:GLubyte )->()>;  /* 1.1 */

pub type PFN_glIndexdv = Option<unsafe extern "C" fn( c: *const GLdouble )->()>;
pub type PFN_glIndexfv = Option<unsafe extern "C" fn( c: *const GLfloat )->()>;
pub type PFN_glIndexiv = Option<unsafe extern "C" fn( c: *const GLint )->()>;
pub type PFN_glIndexsv = Option<unsafe extern "C" fn( c: *const GLshort )->()>;
pub type PFN_glIndexubv = Option<unsafe extern "C" fn( c: *const GLubyte )->()>;  /* 1.1 */

pub type PFN_glColor3b = Option<unsafe extern "C" fn( red:GLbyte, green:GLbyte, blue:GLbyte )->()>;
pub type PFN_glColor3d = Option<unsafe extern "C" fn( red: GLdouble, green: GLdouble, blue: GLdouble )->()>;
pub type PFN_glColor3f = Option<unsafe extern "C" fn( red: GLfloat, green: GLfloat, blue: GLfloat )->()>;
pub type PFN_glColor3i = Option<unsafe extern "C" fn( red: GLint, green: GLint, blue: GLint )->()>;
pub type PFN_glColor3s = Option<unsafe extern "C" fn( red:GLshort, green:GLshort, blue:GLshort )->()>;
pub type PFN_glColor3ub = Option<unsafe extern "C" fn( red:GLubyte, green:GLubyte, blue:GLubyte )->()>;
pub type PFN_glColor3ui = Option<unsafe extern "C" fn( red: GLuint, green: GLuint, blue: GLuint )->()>;
pub type PFN_glColor3us = Option<unsafe extern "C" fn( red: GLushort, green: GLushort, blue: GLushort )->()>;

pub type PFN_glColor4b = Option<unsafe extern "C" fn( red:GLbyte, green:GLbyte,
                                   blue:GLbyte, alpha:GLbyte )->()>;
pub type PFN_glColor4d = Option<unsafe extern "C" fn( red: GLdouble, green: GLdouble,
                                   blue: GLdouble, alpha: GLdouble )->()>;
pub type PFN_glColor4f = Option<unsafe extern "C" fn( red: GLfloat, green: GLfloat,
                                   blue: GLfloat, alpha: GLfloat )->()>;
pub type PFN_glColor4i = Option<unsafe extern "C" fn( red: GLint, green: GLint,
                                   blue: GLint, alpha: GLint )->()>;
pub type PFN_glColor4s = Option<unsafe extern "C" fn( red:GLshort, green:GLshort,
                                   blue:GLshort, alpha:GLshort )->()>;
pub type PFN_glColor4ub = Option<unsafe extern "C" fn( red:GLubyte, green:GLubyte,
                                    blue:GLubyte, alpha:GLubyte )->()>;
pub type PFN_glColor4ui = Option<unsafe extern "C" fn( red: GLuint, green: GLuint,
                                    blue: GLuint, alpha: GLuint )->()>;
pub type PFN_glColor4us = Option<unsafe extern "C" fn( red: GLushort, green: GLushort,
                                    blue: GLushort, alpha: GLushort )->()>;


pub type PFN_glColor3bv = Option<unsafe extern "C" fn( v: *const GLbyte )->()>;
pub type PFN_glColor3dv = Option<unsafe extern "C" fn( v: *const GLdouble )->()>;
pub type PFN_glColor3fv = Option<unsafe extern "C" fn( v: *const GLfloat )->()>;
pub type PFN_glColor3iv = Option<unsafe extern "C" fn( v: *const GLint )->()>;
pub type PFN_glColor3sv = Option<unsafe extern "C" fn( v: *const GLshort )->()>;
pub type PFN_glColor3ubv = Option<unsafe extern "C" fn( v: *const GLubyte )->()>;
pub type PFN_glColor3uiv = Option<unsafe extern "C" fn( v: *const GLuint )->()>;
pub type PFN_glColor3usv = Option<unsafe extern "C" fn( v: *const GLushort )->()>;

pub type PFN_glColor4bv = Option<unsafe extern "C" fn( v: *const GLbyte )->()>;
pub type PFN_glColor4dv = Option<unsafe extern "C" fn( v: *const GLdouble )->()>;
pub type PFN_glColor4fv = Option<unsafe extern "C" fn( v: *const GLfloat )->()>;
pub type PFN_glColor4iv = Option<unsafe extern "C" fn( v: *const GLint )->()>;
pub type PFN_glColor4sv = Option<unsafe extern "C" fn( v: *const GLshort )->()>;
pub type PFN_glColor4ubv = Option<unsafe extern "C" fn( v: *const GLubyte )->()>;
pub type PFN_glColor4uiv = Option<unsafe extern "C" fn( v: *const GLuint )->()>;
pub type PFN_glColor4usv = Option<unsafe extern "C" fn( v: *const GLushort )->()>;


pub type PFN_glTexCoord1d = Option<unsafe extern "C" fn( s: GLdouble )->()>;
pub type PFN_glTexCoord1f = Option<unsafe extern "C" fn( s: GLfloat )->()>;
pub type PFN_glTexCoord1i = Option<unsafe extern "C" fn( s: GLint )->()>;
pub type PFN_glTexCoord1s = Option<unsafe extern "C" fn( s:GLshort )->()>;

pub type PFN_glTexCoord2d = Option<unsafe extern "C" fn( s: GLdouble, t: GLdouble )->()>;
pub type PFN_glTexCoord2f = Option<unsafe extern "C" fn( s: GLfloat, t: GLfloat )->()>;
pub type PFN_glTexCoord2i = Option<unsafe extern "C" fn( s: GLint, t: GLint )->()>;
pub type PFN_glTexCoord2s = Option<unsafe extern "C" fn( s:GLshort, t:GLshort )->()>;

pub type PFN_glTexCoord3d = Option<unsafe extern "C" fn( s: GLdouble, t: GLdouble, r: GLdouble )->()>;
pub type PFN_glTexCoord3f = Option<unsafe extern "C" fn( s: GLfloat, t: GLfloat, r: GLfloat )->()>;
pub type PFN_glTexCoord3i = Option<unsafe extern "C" fn( s: GLint, t: GLint, r: GLint )->()>;
pub type PFN_glTexCoord3s = Option<unsafe extern "C" fn( s:GLshort, t:GLshort, r:GLshort )->()>;

pub type PFN_glTexCoord4d = Option<unsafe extern "C" fn( s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble )->()>;
pub type PFN_glTexCoord4f = Option<unsafe extern "C" fn( s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat )->()>;
pub type PFN_glTexCoord4i = Option<unsafe extern "C" fn( s: GLint, t: GLint, r: GLint, q: GLint )->()>;
pub type PFN_glTexCoord4s = Option<unsafe extern "C" fn( s:GLshort, t:GLshort, r:GLshort, q:GLshort )->()>;

pub type PFN_glTexCoord1dv = Option<unsafe extern "C" fn( v: *const GLdouble )->()>;
pub type PFN_glTexCoord1fv = Option<unsafe extern "C" fn( v: *const GLfloat )->()>;
pub type PFN_glTexCoord1iv = Option<unsafe extern "C" fn( v: *const GLint )->()>;
pub type PFN_glTexCoord1sv = Option<unsafe extern "C" fn( v: *const GLshort )->()>;

pub type PFN_glTexCoord2dv = Option<unsafe extern "C" fn( v: *const GLdouble )->()>;
pub type PFN_glTexCoord2fv = Option<unsafe extern "C" fn( v: *const GLfloat )->()>;
pub type PFN_glTexCoord2iv = Option<unsafe extern "C" fn( v: *const GLint )->()>;
pub type PFN_glTexCoord2sv = Option<unsafe extern "C" fn( v: *const GLshort )->()>;

pub type PFN_glTexCoord3dv = Option<unsafe extern "C" fn( v: *const GLdouble )->()>;
pub type PFN_glTexCoord3fv = Option<unsafe extern "C" fn( v: *const GLfloat )->()>;
pub type PFN_glTexCoord3iv = Option<unsafe extern "C" fn( v: *const GLint )->()>;
pub type PFN_glTexCoord3sv = Option<unsafe extern "C" fn( v: *const GLshort )->()>;

pub type PFN_glTexCoord4dv = Option<unsafe extern "C" fn( v: *const GLdouble )->()>;
pub type PFN_glTexCoord4fv = Option<unsafe extern "C" fn( v: *const GLfloat )->()>;
pub type PFN_glTexCoord4iv = Option<unsafe extern "C" fn( v: *const GLint )->()>;
pub type PFN_glTexCoord4sv = Option<unsafe extern "C" fn( v: *const GLshort )->()>;


pub type PFN_glRasterPos2d = Option<unsafe extern "C" fn( x: GLdouble, y: GLdouble )->()>;
pub type PFN_glRasterPos2f = Option<unsafe extern "C" fn( x: GLfloat, y: GLfloat )->()>;
pub type PFN_glRasterPos2i = Option<unsafe extern "C" fn( x: GLint, y: GLint )->()>;
pub type PFN_glRasterPos2s = Option<unsafe extern "C" fn( x:GLshort, y:GLshort )->()>;

pub type PFN_glRasterPos3d = Option<unsafe extern "C" fn( x: GLdouble, y: GLdouble, z: GLdouble )->()>;
pub type PFN_glRasterPos3f = Option<unsafe extern "C" fn( x: GLfloat, y: GLfloat, z: GLfloat )->()>;
pub type PFN_glRasterPos3i = Option<unsafe extern "C" fn( x: GLint, y: GLint, z: GLint )->()>;
pub type PFN_glRasterPos3s = Option<unsafe extern "C" fn( x:GLshort, y:GLshort, z:GLshort )->()>;

pub type PFN_glRasterPos4d = Option<unsafe extern "C" fn( x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble )->()>;
pub type PFN_glRasterPos4f = Option<unsafe extern "C" fn( x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat )->()>;
pub type PFN_glRasterPos4i = Option<unsafe extern "C" fn( x: GLint, y: GLint, z: GLint, w: GLint )->()>;
pub type PFN_glRasterPos4s = Option<unsafe extern "C" fn( x:GLshort, y:GLshort, z:GLshort, w:GLshort )->()>;

pub type PFN_glRasterPos2dv = Option<unsafe extern "C" fn( v: *const GLdouble )->()>;
pub type PFN_glRasterPos2fv = Option<unsafe extern "C" fn( v: *const GLfloat )->()>;
pub type PFN_glRasterPos2iv = Option<unsafe extern "C" fn( v: *const GLint )->()>;
pub type PFN_glRasterPos2sv = Option<unsafe extern "C" fn( v: *const GLshort )->()>;

pub type PFN_glRasterPos3dv = Option<unsafe extern "C" fn( v: *const GLdouble )->()>;
pub type PFN_glRasterPos3fv = Option<unsafe extern "C" fn( v: *const GLfloat )->()>;
pub type PFN_glRasterPos3iv = Option<unsafe extern "C" fn( v: *const GLint )->()>;
pub type PFN_glRasterPos3sv = Option<unsafe extern "C" fn( v: *const GLshort )->()>;

pub type PFN_glRasterPos4dv = Option<unsafe extern "C" fn( v: *const GLdouble )->()>;
pub type PFN_glRasterPos4fv = Option<unsafe extern "C" fn( v: *const GLfloat )->()>;
pub type PFN_glRasterPos4iv = Option<unsafe extern "C" fn( v: *const GLint )->()>;
pub type PFN_glRasterPos4sv = Option<unsafe extern "C" fn( v: *const GLshort )->()>;


pub type PFN_glRectd = Option<unsafe extern "C" fn( x1: GLdouble, y1: GLdouble, x2: GLdouble, y2: GLdouble )->()>;
pub type PFN_glRectf = Option<unsafe extern "C" fn( x1: GLfloat, y1: GLfloat, x2: GLfloat, y2: GLfloat )->()>;
pub type PFN_glRecti = Option<unsafe extern "C" fn( x1: GLint, y1: GLint, x2: GLint, y2: GLint )->()>;
pub type PFN_glRects = Option<unsafe extern "C" fn( x1:GLshort, y1:GLshort, x2:GLshort, y2:GLshort )->()>;


pub type PFN_glRectdv = Option<unsafe extern "C" fn( v1: *const GLdouble, v2: *const GLdouble )->()>;
pub type PFN_glRectfv = Option<unsafe extern "C" fn( v1: *const GLfloat, v2: *const GLfloat )->()>;
pub type PFN_glRectiv = Option<unsafe extern "C" fn( v1: *const GLint, v2: *const GLint )->()>;
pub type PFN_glRectsv = Option<unsafe extern "C" fn( v1: *const GLshort, v2: *const GLshort )->()>;


/*
 * Vertex Arrays  (1.1)
 */

pub type PFN_glVertexPointer = Option<unsafe extern "C" fn( size: GLint, r#type: GLenum,
                                       stride: GLsizei, ptr: *const GLvoid )->()>;

pub type PFN_glNormalPointer = Option<unsafe extern "C" fn( r#type: GLenum, stride: GLsizei,
                                       ptr: *const GLvoid )->()>;

pub type PFN_glColorPointer = Option<unsafe extern "C" fn( size: GLint, r#type: GLenum,
                                      stride: GLsizei, ptr: *const GLvoid )->()>;

pub type PFN_glIndexPointer = Option<unsafe extern "C" fn( r#type: GLenum, stride: GLsizei,
                                      ptr: *const GLvoid )->()>;

pub type PFN_glTexCoordPointer = Option<unsafe extern "C" fn( size: GLint, r#type: GLenum,
                                         stride: GLsizei, ptr: *const GLvoid )->()>;

pub type PFN_glEdgeFlagPointer = Option<unsafe extern "C" fn( stride: GLsizei, ptr: *const GLvoid )->()>;

pub type PFN_glGetPointerv = Option<unsafe extern "C" fn( pname: GLenum, params: *const *const GLvoid )->()>;

pub type PFN_glArrayElement = Option<unsafe extern "C" fn( i: GLint )->()>;

pub type PFN_glDrawArrays = Option<unsafe extern "C" fn( mode: GLenum, first: GLint, count: GLsizei )->()>;

pub type PFN_glDrawElements = Option<unsafe extern "C" fn( mode: GLenum, count: GLsizei,
                                      r#type: GLenum, indices: *const GLvoid )->()>;

pub type PFN_glInterleavedArrays = Option<unsafe extern "C" fn( format: GLenum, stride: GLsizei,
                                           pointer: *const GLvoid )->()>;

/*
 * Lighting
 */

pub type PFN_glShadeModel = Option<unsafe extern "C" fn( mode: GLenum )->()>;

pub type PFN_glLightf = Option<unsafe extern "C" fn( light: GLenum, pname: GLenum, param: GLfloat )->()>;
pub type PFN_glLighti = Option<unsafe extern "C" fn( light: GLenum, pname: GLenum, param: GLint )->()>;
pub type PFN_glLightfv = Option<unsafe extern "C" fn( light: GLenum, pname: GLenum,
                                 params: *const GLfloat )->()>;
pub type PFN_glLightiv = Option<unsafe extern "C" fn( light: GLenum, pname: GLenum,
                                 params: *const GLint )->()>;

pub type PFN_glGetLightfv = Option<unsafe extern "C" fn( light: GLenum, pname: GLenum,
                                    params: *mut GLfloat )->()>;
pub type PFN_glGetLightiv = Option<unsafe extern "C" fn( light: GLenum, pname: GLenum,
                                    params: *mut GLint )->()>;

pub type PFN_glLightModelf = Option<unsafe extern "C" fn( pname: GLenum, param: GLfloat )->()>;
pub type PFN_glLightModeli = Option<unsafe extern "C" fn( pname: GLenum, param: GLint )->()>;
pub type PFN_glLightModelfv = Option<unsafe extern "C" fn( pname: GLenum, params: *const GLfloat )->()>;
pub type PFN_glLightModeliv = Option<unsafe extern "C" fn( pname: GLenum, params: *const GLint )->()>;

pub type PFN_glMaterialf = Option<unsafe extern "C" fn( face: GLenum, pname: GLenum, param: GLfloat )->()>;
pub type PFN_glMateriali = Option<unsafe extern "C" fn( face: GLenum, pname: GLenum, param: GLint )->()>;
pub type PFN_glMaterialfv = Option<unsafe extern "C" fn( face: GLenum, pname: GLenum, params: *const GLfloat )->()>;
pub type PFN_glMaterialiv = Option<unsafe extern "C" fn( face: GLenum, pname: GLenum, params: *const GLint )->()>;

pub type PFN_glGetMaterialfv = Option<unsafe extern "C" fn( face: GLenum, pname: GLenum, params: *mut GLfloat )->()>;
pub type PFN_glGetMaterialiv = Option<unsafe extern "C" fn( face: GLenum, pname: GLenum, params: *mut GLint )->()>;

pub type PFN_glColorMaterial = Option<unsafe extern "C" fn( face: GLenum, mode: GLenum )->()>;


/*
 * Raster functions
 */

pub type PFN_glPixelZoom = Option<unsafe extern "C" fn( xfactor: GLfloat, yfactor: GLfloat )->()>;

pub type PFN_glPixelStoref = Option<unsafe extern "C" fn( pname: GLenum, param: GLfloat )->()>;
pub type PFN_glPixelStorei = Option<unsafe extern "C" fn( pname: GLenum, param: GLint )->()>;

pub type PFN_glPixelTransferf = Option<unsafe extern "C" fn( pname: GLenum, param: GLfloat )->()>;
pub type PFN_glPixelTransferi = Option<unsafe extern "C" fn( pname: GLenum, param: GLint )->()>;

pub type PFN_glPixelMapfv = Option<unsafe extern "C" fn( map: GLenum, mapsize: GLsizei,
                                    values: *const GLfloat )->()>;
pub type PFN_glPixelMapuiv = Option<unsafe extern "C" fn( map: GLenum, mapsize: GLsizei,
                                     values: *const GLuint )->()>;
pub type PFN_glPixelMapusv = Option<unsafe extern "C" fn( map: GLenum, mapsize: GLsizei,
                                     values: *const GLushort )->()>;

pub type PFN_glGetPixelMapfv = Option<unsafe extern "C" fn( map: GLenum, values: *mut GLfloat )->()>;
pub type PFN_glGetPixelMapuiv = Option<unsafe extern "C" fn( map: GLenum, values: *mut GLuint )->()>;
pub type PFN_glGetPixelMapusv = Option<unsafe extern "C" fn( map: GLenum, values: *mut GLushort )->()>;

pub type PFN_glBitmap = Option<unsafe extern "C" fn( width: GLsizei, height: GLsizei,
                                xorig: GLfloat, yorig: GLfloat,
                                xmove: GLfloat, ymove: GLfloat,
                                bitmap: *const GLubyte )->()>;

pub type PFN_glReadPixels = Option<unsafe extern "C" fn( x: GLint, y: GLint,
                                    width: GLsizei, height: GLsizei,
                                    format: GLenum, r#type: GLenum,
                                    pixels: *mut GLvoid )->()>;

pub type PFN_glDrawPixels = Option<unsafe extern "C" fn( width: GLsizei, height: GLsizei,
                                    format: GLenum, r#type: GLenum,
                                    pixels: *const GLvoid )->()>;

pub type PFN_glCopyPixels = Option<unsafe extern "C" fn( x: GLint, y: GLint,
                                    width: GLsizei, height: GLsizei,
                                    r#type: GLenum )->()>;

/*
 * Stenciling
 */

pub type PFN_glStencilFunc = Option<unsafe extern "C" fn( func: GLenum, r#ref: GLint, mask: GLuint )->()>;

pub type PFN_glStencilMask = Option<unsafe extern "C" fn( mask: GLuint )->()>;

pub type PFN_glStencilOp = Option<unsafe extern "C" fn( fail: GLenum, zfail: GLenum, zpass: GLenum )->()>;

pub type PFN_glClearStencil = Option<unsafe extern "C" fn( s: GLint )->()>;



/*
 * Texture mapping
 */

pub type PFN_glTexGend = Option<unsafe extern "C" fn( coord: GLenum, pname: GLenum, param: GLdouble )->()>;
pub type PFN_glTexGenf = Option<unsafe extern "C" fn( coord: GLenum, pname: GLenum, param: GLfloat )->()>;
pub type PFN_glTexGeni = Option<unsafe extern "C" fn( coord: GLenum, pname: GLenum, param: GLint )->()>;

pub type PFN_glTexGendv = Option<unsafe extern "C" fn( coord: GLenum, pname: GLenum, params: *const GLdouble )->()>;
pub type PFN_glTexGenfv = Option<unsafe extern "C" fn( coord: GLenum, pname: GLenum, params: *const GLfloat )->()>;
pub type PFN_glTexGeniv = Option<unsafe extern "C" fn( coord: GLenum, pname: GLenum, params: *const GLint )->()>;

pub type PFN_glGetTexGendv = Option<unsafe extern "C" fn( coord: GLenum, pname: GLenum, params: *mut GLdouble )->()>;
pub type PFN_glGetTexGenfv = Option<unsafe extern "C" fn( coord: GLenum, pname: GLenum, params: *mut GLfloat )->()>;
pub type PFN_glGetTexGeniv = Option<unsafe extern "C" fn( coord: GLenum, pname: GLenum, params: *mut GLint )->()>;


pub type PFN_glTexEnvf = Option<unsafe extern "C" fn( target: GLenum, pname: GLenum, param: GLfloat )->()>;
pub type PFN_glTexEnvi = Option<unsafe extern "C" fn( target: GLenum, pname: GLenum, param: GLint )->()>;

pub type PFN_glTexEnvfv = Option<unsafe extern "C" fn( target: GLenum, pname: GLenum, params: *const GLfloat )->()>;
pub type PFN_glTexEnviv = Option<unsafe extern "C" fn( target: GLenum, pname: GLenum, params: *const GLint )->()>;

pub type PFN_glGetTexEnvfv = Option<unsafe extern "C" fn( target: GLenum, pname: GLenum, params: *mut GLfloat )->()>;
pub type PFN_glGetTexEnviv = Option<unsafe extern "C" fn( target: GLenum, pname: GLenum, params: *mut GLint )->()>;


pub type PFN_glTexParameterf = Option<unsafe extern "C" fn( target: GLenum, pname: GLenum, param: GLfloat )->()>;
pub type PFN_glTexParameteri = Option<unsafe extern "C" fn( target: GLenum, pname: GLenum, param: GLint )->()>;

pub type PFN_glTexParameterfv = Option<unsafe extern "C" fn( target: GLenum, pname: GLenum,
                                          params: *const GLfloat )->()>;
pub type PFN_glTexParameteriv = Option<unsafe extern "C" fn( target: GLenum, pname: GLenum,
                                          params: *const GLint )->()>;

pub type PFN_glGetTexParameterfv = Option<unsafe extern "C" fn( target: GLenum,
                                           pname: GLenum, params: *mut GLfloat)->()>;
pub type PFN_glGetTexParameteriv = Option<unsafe extern "C" fn( target: GLenum,
                                           pname: GLenum, params: *mut GLint )->()>;

pub type PFN_glGetTexLevelParameterfv = Option<unsafe extern "C" fn( target: GLenum, level: GLint,
                                                pname: GLenum, params: *mut GLfloat )->()>;
pub type PFN_glGetTexLevelParameteriv = Option<unsafe extern "C" fn( target: GLenum, level: GLint,
                                                pname: GLenum, params: *mut GLint )->()>;


pub type PFN_glTexImage1D = Option<unsafe extern "C" fn( target: GLenum, level: GLint,
                                    internalFormat: GLint,
                                    width: GLsizei, border: GLint,
                                    format: GLenum, r#type: GLenum,
                                    pixels: *const GLvoid )->()>;

pub type PFN_glTexImage2D = Option<unsafe extern "C" fn( target: GLenum, level: GLint,
                                    internalFormat: GLint,
                                    width: GLsizei, height: GLsizei,
                                    border: GLint, format: GLenum, r#type: GLenum,
                                    pixels: *const GLvoid )->()>;

pub type PFN_glGetTexImage = Option<unsafe extern "C" fn( target: GLenum, level: GLint,
                                     format: GLenum, r#type: GLenum,
                                     pixels: *mut GLvoid )->()>;


/* 1.1 functions */

pub type PFN_glGenTextures = Option<unsafe extern "C" fn( n: GLsizei, textures: *mut GLuint )->()>;

pub type PFN_glDeleteTextures = Option<unsafe extern "C" fn( n: GLsizei, textures: *const GLuint)->()>;

pub type PFN_glBindTexture = Option<unsafe extern "C" fn( target: GLenum, texture: GLuint )->()>;

pub type PFN_glPrioritizeTextures = Option<unsafe extern "C" fn( n: GLsizei,
                                            textures: *const GLuint,
                                            priorities: *const GLclampf )->()>;

pub type PFN_glAreTexturesResident = Option< unsafe extern "C" fn( n: GLsizei,
                                                  textures: *const GLuint,
                                                  residences: *const GLboolean ) -> GLboolean>;

pub type PFN_glIsTexture = Option<unsafe extern "C" fn( texture: GLuint ) -> GLboolean>;


pub type PFN_glTexSubImage1D = Option<unsafe extern "C" fn( target: GLenum, level: GLint,
                                       xoffset: GLint,
                                       width: GLsizei, format: GLenum,
                                       r#type: GLenum, pixels: *const GLvoid )->()>;


pub type PFN_glTexSubImage2D = Option<unsafe extern "C" fn( target: GLenum, level: GLint,
                                       xoffset: GLint, yoffset: GLint,
                                       width: GLsizei, height: GLsizei,
                                       format: GLenum, r#type: GLenum,
                                       pixels: *const GLvoid )->()>;


pub type PFN_glCopyTexImage1D = Option<unsafe extern "C" fn( target: GLenum, level: GLint,
                                        internalformat: GLenum,
                                        x: GLint, y: GLint,
                                        width: GLsizei, border: GLint )->()>;


pub type PFN_glCopyTexImage2D = Option<unsafe extern "C" fn( target: GLenum, level: GLint,
                                        internalformat: GLenum,
                                        x: GLint, y: GLint,
                                        width: GLsizei, height: GLsizei,
                                        border: GLint )->()>;


pub type PFN_glCopyTexSubImage1D = Option<unsafe extern "C" fn( target: GLenum, level: GLint,
                                           xoffset: GLint, x: GLint, y: GLint,
                                           width: GLsizei )->()>;


pub type PFN_glCopyTexSubImage2D = Option<unsafe extern "C" fn( target: GLenum, level: GLint,
                                           xoffset: GLint, yoffset: GLint,
                                           x: GLint, y: GLint,
                                           width: GLsizei, height: GLsizei )->()>;


/*
 * Evaluators
 */

pub type PFN_glMap1d = Option<unsafe extern "C" fn( target: GLenum, u1: GLdouble, u2: GLdouble,
                               stride: GLint,
                               order: GLint, points: *const GLdouble )->()>;
pub type PFN_glMap1f = Option<unsafe extern "C" fn( target: GLenum, u1: GLfloat, u2: GLfloat,
                               stride: GLint,
                               order: GLint, points: *const GLfloat )->()>;

pub type PFN_glMap2d = Option<unsafe extern "C" fn( target: GLenum,
		     u1: GLdouble, u2: GLdouble, ustride: GLint, uorder: GLint,
		     v1: GLdouble, v2: GLdouble, vstride: GLint, vorder: GLint,
		     points: *const GLdouble )->()>;
pub type PFN_glMap2f = Option<unsafe extern "C" fn( target: GLenum,
		     u1: GLfloat, u2: GLfloat, ustride: GLint, uorder: GLint,
		     v1: GLfloat, v2: GLfloat, vstride: GLint, vorder: GLint,
		     points: *const GLfloat )->()>;

pub type PFN_glGetMapdv = Option<unsafe extern "C" fn( target: GLenum, query: GLenum, v: *mut GLdouble )->()>;
pub type PFN_glGetMapfv = Option<unsafe extern "C" fn( target: GLenum, query: GLenum, v: *mut GLfloat )->()>;
pub type PFN_glGetMapiv = Option<unsafe extern "C" fn( target: GLenum, query: GLenum, v: *mut GLint )->()>;

pub type PFN_glEvalCoord1d = Option<unsafe extern "C" fn( u: GLdouble )->()>;
pub type PFN_glEvalCoord1f = Option<unsafe extern "C" fn( u: GLfloat )->()>;

pub type PFN_glEvalCoord1dv = Option<unsafe extern "C" fn( u: *const GLdouble )->()>;
pub type PFN_glEvalCoord1fv = Option<unsafe extern "C" fn( u: *const GLfloat )->()>;

pub type PFN_glEvalCoord2d = Option<unsafe extern "C" fn( u: GLdouble, v: GLdouble )->()>;
pub type PFN_glEvalCoord2f = Option<unsafe extern "C" fn( u: GLfloat, v: GLfloat )->()>;

pub type PFN_glEvalCoord2dv = Option<unsafe extern "C" fn( u: *const GLdouble )->()>;
pub type PFN_glEvalCoord2fv = Option<unsafe extern "C" fn( u: *const GLfloat )->()>;

pub type PFN_glMapGrid1d = Option<unsafe extern "C" fn( un: GLint, u1: GLdouble, u2: GLdouble )->()>;
pub type PFN_glMapGrid1f = Option<unsafe extern "C" fn( un: GLint, u1: GLfloat, u2: GLfloat )->()>;

pub type PFN_glMapGrid2d = Option<unsafe extern "C" fn( un: GLint, u1: GLdouble, u2: GLdouble,
                                   vn: GLint, v1: GLdouble, v2: GLdouble )->()>;
pub type PFN_glMapGrid2f = Option<unsafe extern "C" fn( un: GLint, u1: GLfloat, u2: GLfloat,
                                   vn: GLint, v1: GLfloat, v2: GLfloat )->()>;

pub type PFN_glEvalPoint1 = Option<unsafe extern "C" fn( i: GLint )->()>;

pub type PFN_glEvalPoint2 = Option<unsafe extern "C" fn( i: GLint, j: GLint )->()>;

pub type PFN_glEvalMesh1 = Option<unsafe extern "C" fn( mode: GLenum, i1: GLint, i2: GLint )->()>;

pub type PFN_glEvalMesh2 = Option<unsafe extern "C" fn( mode: GLenum, i1: GLint, i2: GLint, j1: GLint, j2: GLint )->()>;


/*
 * Fog
 */

pub type PFN_glFogf = Option<unsafe extern "C" fn( pname: GLenum, param: GLfloat )->()>;

pub type PFN_glFogi = Option<unsafe extern "C" fn( pname: GLenum, param: GLint )->()>;

pub type PFN_glFogfv = Option<unsafe extern "C" fn( pname: GLenum, params: *const GLfloat )->()>;

pub type PFN_glFogiv = Option<unsafe extern "C" fn( pname: GLenum, params: *const GLint )->()>;


/*
 * Selection and Feedback
 */

pub type PFN_glFeedbackBuffer = Option<unsafe extern "C" fn( size: GLsizei, r#type: GLenum, buffer: *mut GLfloat )->()>;

pub type PFN_glPassThrough = Option<unsafe extern "C" fn( token: GLfloat )->()>;

pub type PFN_glSelectBuffer = Option<unsafe extern "C" fn( size: GLsizei, buffer: *mut GLuint )->()>;

pub type PFN_glInitNames = Option<unsafe extern "C" fn()->()>;

pub type PFN_glLoadName = Option<unsafe extern "C" fn( name: GLuint )->()>;

pub type PFN_glPushName = Option<unsafe extern "C" fn( name: GLuint )->()>;

pub type PFN_glPopName = Option<unsafe extern "C" fn()->()>;

// OpenGL 1.2



pub const GL_RESCALE_NORMAL: std::os::raw::c_int = 0x803A;
pub const GL_CLAMP_TO_EDGE: std::os::raw::c_int = 0x812F;
pub const GL_MAX_ELEMENTS_VERTICES: std::os::raw::c_int = 0x80E8;
pub const GL_MAX_ELEMENTS_INDICES: std::os::raw::c_int = 0x80E9;
pub const GL_BGR: std::os::raw::c_int = 0x80E0;
pub const GL_BGRA: std::os::raw::c_int = 0x80E1;
pub const GL_UNSIGNED_BYTE_3_3_2: std::os::raw::c_int = 0x8032;
pub const GL_UNSIGNED_BYTE_2_3_3_REV: std::os::raw::c_int = 0x8362;
pub const GL_UNSIGNED_SHORT_5_6_5: std::os::raw::c_int = 0x8363;
pub const GL_UNSIGNED_SHORT_5_6_5_REV: std::os::raw::c_int = 0x8364;
pub const GL_UNSIGNED_SHORT_4_4_4_4: std::os::raw::c_int = 0x8033;
pub const GL_UNSIGNED_SHORT_4_4_4_4_REV: std::os::raw::c_int = 0x8365;
pub const GL_UNSIGNED_SHORT_5_5_5_1: std::os::raw::c_int = 0x8034;
pub const GL_UNSIGNED_SHORT_1_5_5_5_REV: std::os::raw::c_int = 0x8366;
pub const GL_UNSIGNED_INT_8_8_8_8: std::os::raw::c_int = 0x8035;
pub const GL_UNSIGNED_INT_8_8_8_8_REV: std::os::raw::c_int = 0x8367;
pub const GL_UNSIGNED_INT_10_10_10_2: std::os::raw::c_int = 0x8036;
pub const GL_UNSIGNED_INT_2_10_10_10_REV: std::os::raw::c_int = 0x8368;
pub const GL_LIGHT_MODEL_COLOR_CONTROL: std::os::raw::c_int = 0x81F8;
pub const GL_SINGLE_COLOR: std::os::raw::c_int = 0x81F9;
pub const GL_SEPARATE_SPECULAR_COLOR: std::os::raw::c_int = 0x81FA;
pub const GL_TEXTURE_MIN_LOD: std::os::raw::c_int = 0x813A;
pub const GL_TEXTURE_MAX_LOD: std::os::raw::c_int = 0x813B;
pub const GL_TEXTURE_BASE_LEVEL: std::os::raw::c_int = 0x813C;
pub const GL_TEXTURE_MAX_LEVEL: std::os::raw::c_int = 0x813D;
pub const GL_SMOOTH_POINT_SIZE_RANGE: std::os::raw::c_int = 0x0B12;
pub const GL_SMOOTH_POINT_SIZE_GRANULARITY: std::os::raw::c_int = 0x0B13;
pub const GL_SMOOTH_LINE_WIDTH_RANGE: std::os::raw::c_int = 0x0B22;
pub const GL_SMOOTH_LINE_WIDTH_GRANULARITY: std::os::raw::c_int = 0x0B23;
pub const GL_ALIASED_POINT_SIZE_RANGE: std::os::raw::c_int = 0x846D;
pub const GL_ALIASED_LINE_WIDTH_RANGE: std::os::raw::c_int = 0x846E;
pub const GL_PACK_SKIP_IMAGES: std::os::raw::c_int = 0x806B;
pub const GL_PACK_IMAGE_HEIGHT: std::os::raw::c_int = 0x806C;
pub const GL_UNPACK_SKIP_IMAGES: std::os::raw::c_int = 0x806D;
pub const GL_UNPACK_IMAGE_HEIGHT: std::os::raw::c_int = 0x806E;
pub const GL_TEXTURE_3D: std::os::raw::c_int = 0x806F;
pub const GL_PROXY_TEXTURE_3D: std::os::raw::c_int = 0x8070;
pub const GL_TEXTURE_DEPTH: std::os::raw::c_int = 0x8071;
pub const GL_TEXTURE_WRAP_R: std::os::raw::c_int = 0x8072;
pub const GL_MAX_3D_TEXTURE_SIZE: std::os::raw::c_int = 0x8073;
pub const GL_TEXTURE_BINDING_3D: std::os::raw::c_int = 0x806A;

pub type PFN_glDrawRangeElements = Option<unsafe extern "C" fn( mode: GLenum, start: GLuint,
	end: GLuint, count: GLsizei, r#type: GLenum, indices: *const GLvoid )->()>;

pub type PFN_glTexImage3D = Option<unsafe extern "C" fn( target: GLenum, level: GLint,
                                      internalFormat: GLint,
                                      width: GLsizei, height: GLsizei,
                                      depth: GLsizei, border: GLint,
                                      format: GLenum, r#type: GLenum,
                                      pixels: *const GLvoid )->()>;

pub type PFN_glTexSubImage3D = Option<unsafe extern "C" fn( target: GLenum, level: GLint,
                                         xoffset: GLint, yoffset: GLint,
                                         zoffset: GLint, width: GLsizei,
                                         height: GLsizei, depth: GLsizei,
                                         format: GLenum,
                                         r#type: GLenum, pixels: *const GLvoid)->()>;

pub type PFN_glCopyTexSubImage3D = Option<unsafe extern "C" fn( target: GLenum, level: GLint,
                                             xoffset: GLint, yoffset: GLint,
                                             zoffset: GLint, x: GLint,
                                             y: GLint, width: GLsizei,
                                             height: GLsizei )->()>;

// arb extension functions
pub type  PFNGLDRAWRANGEELEMENTSPROC = Option<unsafe extern "C" fn(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, r#type: GLenum, indices: *const GLvoid) -> ()>;
pub type  PFNGLTEXIMAGE3DPROC = Option<unsafe extern "C" fn(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, r#type: GLenum, pixels: *const GLvoid)>;
pub type  PFNGLTEXSUBIMAGE3DPROC = Option<unsafe extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, r#type: GLenum, pixels: *const GLvoid)>;
pub type  PFNGLCOPYTEXSUBIMAGE3DPROC = Option<unsafe extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei)>;


/*
 * GL_ARB_imaging
 */

pub const GL_CONSTANT_COLOR: std::os::raw::c_int = 0x8001;
pub const GL_ONE_MINUS_CONSTANT_COLOR: std::os::raw::c_int = 0x8002;
pub const GL_CONSTANT_ALPHA: std::os::raw::c_int = 0x8003;
pub const GL_ONE_MINUS_CONSTANT_ALPHA: std::os::raw::c_int = 0x8004;
pub const GL_COLOR_TABLE: std::os::raw::c_int = 0x80D0;
pub const GL_POST_CONVOLUTION_COLOR_TABLE: std::os::raw::c_int = 0x80D1;
pub const GL_POST_COLOR_MATRIX_COLOR_TABLE: std::os::raw::c_int = 0x80D2;
pub const GL_PROXY_COLOR_TABLE: std::os::raw::c_int = 0x80D3;
pub const GL_PROXY_POST_CONVOLUTION_COLOR_TABLE: std::os::raw::c_int = 0x80D4;
pub const GL_PROXY_POST_COLOR_MATRIX_COLOR_TABLE: std::os::raw::c_int = 0x80D5;
pub const GL_COLOR_TABLE_SCALE: std::os::raw::c_int = 0x80D6;
pub const GL_COLOR_TABLE_BIAS: std::os::raw::c_int = 0x80D7;
pub const GL_COLOR_TABLE_FORMAT: std::os::raw::c_int = 0x80D8;
pub const GL_COLOR_TABLE_WIDTH: std::os::raw::c_int = 0x80D9;
pub const GL_COLOR_TABLE_RED_SIZE: std::os::raw::c_int = 0x80DA;
pub const GL_COLOR_TABLE_GREEN_SIZE: std::os::raw::c_int = 0x80DB;
pub const GL_COLOR_TABLE_BLUE_SIZE: std::os::raw::c_int = 0x80DC;
pub const GL_COLOR_TABLE_ALPHA_SIZE: std::os::raw::c_int = 0x80DD;
pub const GL_COLOR_TABLE_LUMINANCE_SIZE: std::os::raw::c_int = 0x80DE;
pub const GL_COLOR_TABLE_INTENSITY_SIZE: std::os::raw::c_int = 0x80DF;
pub const GL_CONVOLUTION_1D: std::os::raw::c_int = 0x8010;
pub const GL_CONVOLUTION_2D: std::os::raw::c_int = 0x8011;
pub const GL_SEPARABLE_2D: std::os::raw::c_int = 0x8012;
pub const GL_CONVOLUTION_BORDER_MODE: std::os::raw::c_int = 0x8013;
pub const GL_CONVOLUTION_FILTER_SCALE: std::os::raw::c_int = 0x8014;
pub const GL_CONVOLUTION_FILTER_BIAS: std::os::raw::c_int = 0x8015;
pub const GL_REDUCE: std::os::raw::c_int = 0x8016;
pub const GL_CONVOLUTION_FORMAT: std::os::raw::c_int = 0x8017;
pub const GL_CONVOLUTION_WIDTH: std::os::raw::c_int = 0x8018;
pub const GL_CONVOLUTION_HEIGHT: std::os::raw::c_int = 0x8019;
pub const GL_MAX_CONVOLUTION_WIDTH: std::os::raw::c_int = 0x801A;
pub const GL_MAX_CONVOLUTION_HEIGHT: std::os::raw::c_int = 0x801B;
pub const GL_POST_CONVOLUTION_RED_SCALE: std::os::raw::c_int = 0x801C;
pub const GL_POST_CONVOLUTION_GREEN_SCALE: std::os::raw::c_int = 0x801D;
pub const GL_POST_CONVOLUTION_BLUE_SCALE: std::os::raw::c_int = 0x801E;
pub const GL_POST_CONVOLUTION_ALPHA_SCALE: std::os::raw::c_int = 0x801F;
pub const GL_POST_CONVOLUTION_RED_BIAS: std::os::raw::c_int = 0x8020;
pub const GL_POST_CONVOLUTION_GREEN_BIAS: std::os::raw::c_int = 0x8021;
pub const GL_POST_CONVOLUTION_BLUE_BIAS: std::os::raw::c_int = 0x8022;
pub const GL_POST_CONVOLUTION_ALPHA_BIAS: std::os::raw::c_int = 0x8023;
pub const GL_CONSTANT_BORDER: std::os::raw::c_int = 0x8151;
pub const GL_REPLICATE_BORDER: std::os::raw::c_int = 0x8153;
pub const GL_CONVOLUTION_BORDER_COLOR: std::os::raw::c_int = 0x8154;
pub const GL_COLOR_MATRIX: std::os::raw::c_int = 0x80B1;
pub const GL_COLOR_MATRIX_STACK_DEPTH: std::os::raw::c_int = 0x80B2;
pub const GL_MAX_COLOR_MATRIX_STACK_DEPTH: std::os::raw::c_int = 0x80B3;
pub const GL_POST_COLOR_MATRIX_RED_SCALE: std::os::raw::c_int = 0x80B4;
pub const GL_POST_COLOR_MATRIX_GREEN_SCALE: std::os::raw::c_int = 0x80B5;
pub const GL_POST_COLOR_MATRIX_BLUE_SCALE: std::os::raw::c_int = 0x80B6;
pub const GL_POST_COLOR_MATRIX_ALPHA_SCALE: std::os::raw::c_int = 0x80B7;
pub const GL_POST_COLOR_MATRIX_RED_BIAS: std::os::raw::c_int = 0x80B8;
pub const GL_POST_COLOR_MATRIX_GREEN_BIAS: std::os::raw::c_int = 0x80B9;
pub const GL_POST_COLOR_MATRIX_BLUE_BIAS: std::os::raw::c_int = 0x80BA;
pub const GL_POST_COLOR_MATRIX_ALPHA_BIAS: std::os::raw::c_int = 0x80BB;
pub const GL_HISTOGRAM: std::os::raw::c_int = 0x8024;
pub const GL_PROXY_HISTOGRAM: std::os::raw::c_int = 0x8025;
pub const GL_HISTOGRAM_WIDTH: std::os::raw::c_int = 0x8026;
pub const GL_HISTOGRAM_FORMAT: std::os::raw::c_int = 0x8027;
pub const GL_HISTOGRAM_RED_SIZE: std::os::raw::c_int = 0x8028;
pub const GL_HISTOGRAM_GREEN_SIZE: std::os::raw::c_int = 0x8029;
pub const GL_HISTOGRAM_BLUE_SIZE: std::os::raw::c_int = 0x802A;
pub const GL_HISTOGRAM_ALPHA_SIZE: std::os::raw::c_int = 0x802B;
pub const GL_HISTOGRAM_LUMINANCE_SIZE: std::os::raw::c_int = 0x802C;
pub const GL_HISTOGRAM_SINK: std::os::raw::c_int = 0x802D;
pub const GL_MINMAX: std::os::raw::c_int = 0x802E;
pub const GL_MINMAX_FORMAT: std::os::raw::c_int = 0x802F;
pub const GL_MINMAX_SINK: std::os::raw::c_int = 0x8030;

pub const GL_BLEND_EQUATION: std::os::raw::c_int = 0x8009;
pub const GL_MIN: std::os::raw::c_int = 0x8007;
pub const GL_MAX: std::os::raw::c_int = 0x8008;
pub const GL_FUNC_ADD: std::os::raw::c_int = 0x8006;
pub const GL_FUNC_SUBTRACT: std::os::raw::c_int = 0x800A;
pub const GL_FUNC_REVERSE_SUBTRACT: std::os::raw::c_int = 0x800B;
pub const GL_BLEND_COLOR: std::os::raw::c_int = 0x8005;


pub type PFN_glColorTable = Option<unsafe extern "C" fn( target: GLenum, internalformat: GLenum,
                                    width: GLsizei, format: GLenum,
                                    r#type: GLenum, table: *const GLvoid )->()>;

pub type PFN_glColorSubTable = Option<unsafe extern "C" fn( target: GLenum,
                                       start: GLsizei, count: GLsizei,
                                       format: GLenum, r#type: GLenum,
                                       data: *const GLvoid )->()>;

pub type PFN_glColorTableParameteriv = Option<unsafe extern "C" fn(target: GLenum, pname: GLenum,
                                              params: *const GLint)->()>;

pub type PFN_glColorTableParameterfv = Option<unsafe extern "C" fn(target: GLenum, pname: GLenum,
                                              params: *const GLfloat)->()>;

pub type PFN_glCopyColorSubTable = Option<unsafe extern "C" fn( target: GLenum, start: GLsizei,
                                           x: GLint, y: GLint, width: GLsizei )->()>;

pub type PFN_glCopyColorTable = Option<unsafe extern "C" fn( target: GLenum, internalformat: GLenum,
                                        x: GLint, y: GLint, width: GLsizei )->()>;

pub type PFN_glGetColorTable = Option<unsafe extern "C" fn( target: GLenum, format: GLenum,
                                       r#type: GLenum, table: *mut GLvoid )->()>;

pub type PFN_glGetColorTableParameterfv = Option<unsafe extern "C" fn( target: GLenum, pname: GLenum,
                                                  params: *mut GLfloat )->()>;

pub type PFN_glGetColorTableParameteriv = Option<unsafe extern "C" fn( target: GLenum, pname: GLenum,
                                                  params: *mut GLint )->()>;

pub type PFN_glBlendEquation = Option<unsafe extern "C" fn( mode: GLenum )->()>;

pub type PFN_glBlendColor = Option<unsafe extern "C" fn( red: GLclampf, green: GLclampf,
                                    blue: GLclampf, alpha: GLclampf )->()>;

pub type PFN_glHistogram = Option<unsafe extern "C" fn( target: GLenum, width: GLsizei,
				   internalformat: GLenum, sink:GLboolean )->()>;

pub type PFN_glResetHistogram = Option<unsafe extern "C" fn( target: GLenum )->()>;

pub type PFN_glGetHistogram = Option<unsafe extern "C" fn( target: GLenum, reset:GLboolean,
				      format: GLenum, r#type: GLenum,
				      values: *mut GLvoid )->()>;

pub type PFN_glGetHistogramParameterfv = Option<unsafe extern "C" fn( target: GLenum, pname: GLenum,
						 params: *mut GLfloat )->()>;

pub type PFN_glGetHistogramParameteriv = Option<unsafe extern "C" fn( target: GLenum, pname: GLenum,
						 params: *mut GLint )->()>;

pub type PFN_glMinmax = Option<unsafe extern "C" fn( target: GLenum, internalformat: GLenum,
				sink:GLboolean )->()>;

pub type PFN_glResetMinmax = Option<unsafe extern "C" fn( target: GLenum )->()>;

pub type PFN_glGetMinmax = Option<unsafe extern "C" fn( target: GLenum, reset:GLboolean,
                                   format: GLenum, types: GLenum,
                                   values: *mut GLvoid )->()>;

pub type PFN_glGetMinmaxParameterfv = Option<unsafe extern "C" fn( target: GLenum, pname: GLenum,
					      params: *mut GLfloat )->()>;

pub type PFN_glGetMinmaxParameteriv = Option<unsafe extern "C" fn( target: GLenum, pname: GLenum,
					      params: *mut GLint )->()>;

pub type PFN_glConvolutionFilter1D = Option<unsafe extern "C" fn( target: GLenum,
	internalformat: GLenum, width: GLsizei, format: GLenum, r#type: GLenum,
	image: *const GLvoid )->()>;

pub type PFN_glConvolutionFilter2D = Option<unsafe extern "C" fn( target: GLenum,
	internalformat: GLenum, width: GLsizei, height: GLsizei, format: GLenum,
	r#type: GLenum, image: *const GLvoid )->()>;

pub type PFN_glConvolutionParameterf = Option<unsafe extern "C" fn( target: GLenum, pname: GLenum,
	params: GLfloat )->()>;

pub type PFN_glConvolutionParameterfv = Option<unsafe extern "C" fn( target: GLenum, pname: GLenum,
	params: *const GLfloat )->()>;

pub type PFN_glConvolutionParameteri = Option<unsafe extern "C" fn( target: GLenum, pname: GLenum,
	params: GLint )->()>;

pub type PFN_glConvolutionParameteriv = Option<unsafe extern "C" fn( target: GLenum, pname: GLenum,
	params: *const GLint )->()>;

pub type PFN_glCopyConvolutionFilter1D = Option<unsafe extern "C" fn( target: GLenum,
	internalformat: GLenum, x: GLint, y: GLint, width: GLsizei )->()>;

pub type PFN_glCopyConvolutionFilter2D = Option<unsafe extern "C" fn( target: GLenum,
	internalformat: GLenum, x: GLint, y: GLint, width: GLsizei,
	height: GLsizei)->()>;

pub type PFN_glGetConvolutionFilter = Option<unsafe extern "C" fn( target: GLenum, format: GLenum,
	r#type: GLenum, image: *mut GLvoid )->()>;

pub type PFN_glGetConvolutionParameterfv = Option<unsafe extern "C" fn( target: GLenum, pname: GLenum,
	params: *mut GLfloat )->()>;

pub type PFN_glGetConvolutionParameteriv = Option<unsafe extern "C" fn( target: GLenum, pname: GLenum,
	params: *mut GLint )->()>;

pub type PFN_glSeparableFilter2D = Option<unsafe extern "C" fn( target: GLenum,
	internalformat: GLenum, width: GLsizei, height: GLsizei, format: GLenum,
	r#type: GLenum, row: *const GLvoid, column: *const GLvoid )->()>;

pub type PFN_glGetSeparableFilter = Option<unsafe extern "C" fn( target: GLenum, format: GLenum,
	r#type: GLenum, row: *mut GLvoid, column: *mut GLvoid, span: *mut GLvoid )->()>;





/*
 * GL_ARB_multitexture (ARB extension 1 and OpenGL 1.2.1)
 */


pub const GL_TEXTURE0_ARB:std::os::raw::c_int = 0x84C0;pub const GL_TEXTURE1_ARB:std::os::raw::c_int = 0x84C1;
pub const GL_TEXTURE2_ARB:std::os::raw::c_int = 0x84C2;
pub const GL_TEXTURE3_ARB:std::os::raw::c_int = 0x84C3;
pub const GL_TEXTURE4_ARB:std::os::raw::c_int = 0x84C4;
pub const GL_TEXTURE5_ARB:std::os::raw::c_int = 0x84C5;
pub const GL_TEXTURE6_ARB:std::os::raw::c_int = 0x84C6;
pub const GL_TEXTURE7_ARB:std::os::raw::c_int = 0x84C7;
pub const GL_TEXTURE8_ARB:std::os::raw::c_int = 0x84C8;
pub const GL_TEXTURE9_ARB:std::os::raw::c_int = 0x84C9;
pub const GL_TEXTURE10_ARB:std::os::raw::c_int = 0x84CA;
pub const GL_TEXTURE11_ARB:std::os::raw::c_int = 0x84CB;
pub const GL_TEXTURE12_ARB:std::os::raw::c_int = 0x84CC;
pub const GL_TEXTURE13_ARB:std::os::raw::c_int = 0x84CD;
pub const GL_TEXTURE14_ARB:std::os::raw::c_int = 0x84CE;
pub const GL_TEXTURE15_ARB:std::os::raw::c_int = 0x84CF;
pub const GL_TEXTURE16_ARB:std::os::raw::c_int = 0x84D0;
pub const GL_TEXTURE17_ARB:std::os::raw::c_int = 0x84D1;
pub const GL_TEXTURE18_ARB:std::os::raw::c_int = 0x84D2;
pub const GL_TEXTURE19_ARB:std::os::raw::c_int = 0x84D3;
pub const GL_TEXTURE20_ARB:std::os::raw::c_int = 0x84D4;
pub const GL_TEXTURE21_ARB:std::os::raw::c_int = 0x84D5;
pub const GL_TEXTURE22_ARB:std::os::raw::c_int = 0x84D6;
pub const GL_TEXTURE23_ARB:std::os::raw::c_int = 0x84D7;
pub const GL_TEXTURE24_ARB:std::os::raw::c_int = 0x84D8;
pub const GL_TEXTURE25_ARB:std::os::raw::c_int = 0x84D9;
pub const GL_TEXTURE26_ARB:std::os::raw::c_int = 0x84DA;
pub const GL_TEXTURE27_ARB:std::os::raw::c_int = 0x84DB;
pub const GL_TEXTURE28_ARB:std::os::raw::c_int = 0x84DC;
pub const GL_TEXTURE29_ARB:std::os::raw::c_int = 0x84DD;
pub const GL_TEXTURE30_ARB:std::os::raw::c_int = 0x84DE;
pub const GL_TEXTURE31_ARB:std::os::raw::c_int = 0x84DF;
pub const GL_ACTIVE_TEXTURE_ARB:std::os::raw::c_int = 0x84E0;
pub const GL_CLIENT_ACTIVE_TEXTURE_ARB:std::os::raw::c_int = 0x84E1;
pub const GL_MAX_TEXTURE_UNITS_ARB: std::os::raw::c_int=0x84E2;

pub type PFN_glActiveTextureARB = Option<unsafe extern "C" fn(texture: GLenum)->()>;
pub type PFN_glClientActiveTextureARB = Option<unsafe extern "C" fn(texture: GLenum)->()>;
pub type PFN_glMultiTexCoord1dARB = Option<unsafe extern "C" fn(target: GLenum, s: GLdouble)->()>;
pub type PFN_glMultiTexCoord1dvARB = Option<unsafe extern "C" fn(target: GLenum, v: *const GLdouble)->()>;
pub type PFN_glMultiTexCoord1fARB = Option<unsafe extern "C" fn(target: GLenum, s: GLfloat)->()>;
pub type PFN_glMultiTexCoord1fvARB = Option<unsafe extern "C" fn(target: GLenum, v: *const GLfloat)->()>;
pub type PFN_glMultiTexCoord1iARB = Option<unsafe extern "C" fn(target: GLenum, s: GLint)->()>;
pub type PFN_glMultiTexCoord1ivARB = Option<unsafe extern "C" fn(target: GLenum, v: *const GLint)->()>;
pub type PFN_glMultiTexCoord1sARB = Option<unsafe extern "C" fn(target: GLenum, s:GLshort)->()>;
pub type PFN_glMultiTexCoord1svARB = Option<unsafe extern "C" fn(target: GLenum, v: *const GLshort)->()>;
pub type PFN_glMultiTexCoord2dARB = Option<unsafe extern "C" fn(target: GLenum, s: GLdouble, t: GLdouble)->()>;
pub type PFN_glMultiTexCoord2dvARB = Option<unsafe extern "C" fn(target: GLenum, v: *const GLdouble)->()>;
pub type PFN_glMultiTexCoord2fARB = Option<unsafe extern "C" fn(target: GLenum, s: GLfloat, t: GLfloat)->()>;
pub type PFN_glMultiTexCoord2fvARB = Option<unsafe extern "C" fn(target: GLenum, v: *const GLfloat)->()>;
pub type PFN_glMultiTexCoord2iARB = Option<unsafe extern "C" fn(target: GLenum, s: GLint, t: GLint)->()>;
pub type PFN_glMultiTexCoord2ivARB = Option<unsafe extern "C" fn(target: GLenum, v: *const GLint)->()>;
pub type PFN_glMultiTexCoord2sARB = Option<unsafe extern "C" fn(target: GLenum, s:GLshort, t:GLshort)->()>;
pub type PFN_glMultiTexCoord2svARB = Option<unsafe extern "C" fn(target: GLenum, v: *const GLshort)->()>;
pub type PFN_glMultiTexCoord3dARB = Option<unsafe extern "C" fn(target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble)->()>;
pub type PFN_glMultiTexCoord3dvARB = Option<unsafe extern "C" fn(target: GLenum, v: *const GLdouble)->()>;
pub type PFN_glMultiTexCoord3fARB = Option<unsafe extern "C" fn(target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat)->()>;
pub type PFN_glMultiTexCoord3fvARB = Option<unsafe extern "C" fn(target: GLenum, v: *const GLfloat)->()>;
pub type PFN_glMultiTexCoord3iARB = Option<unsafe extern "C" fn(target: GLenum, s: GLint, t: GLint, r: GLint)->()>;
pub type PFN_glMultiTexCoord3ivARB = Option<unsafe extern "C" fn(target: GLenum, v: *const GLint)->()>;
pub type PFN_glMultiTexCoord3sARB = Option<unsafe extern "C" fn(target: GLenum, s:GLshort, t:GLshort, r:GLshort)->()>;
pub type PFN_glMultiTexCoord3svARB = Option<unsafe extern "C" fn(target: GLenum, v: *const GLshort)->()>;
pub type PFN_glMultiTexCoord4dARB = Option<unsafe extern "C" fn(target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble)->()>;
pub type PFN_glMultiTexCoord4dvARB = Option<unsafe extern "C" fn(target: GLenum, v: *const GLdouble)->()>;
pub type PFN_glMultiTexCoord4fARB = Option<unsafe extern "C" fn(target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat)->()>;
pub type PFN_glMultiTexCoord4fvARB = Option<unsafe extern "C" fn(target: GLenum, v: *const GLfloat)->()>;
pub type PFN_glMultiTexCoord4iARB = Option<unsafe extern "C" fn(target: GLenum, s: GLint, t: GLint, r: GLint, q: GLint)->()>;
pub type PFN_glMultiTexCoord4ivARB = Option<unsafe extern "C" fn(target: GLenum, v: *const GLint)->()>;
pub type PFN_glMultiTexCoord4sARB = Option<unsafe extern "C" fn(target: GLenum, s:GLshort, t:GLshort, r:GLshort, q:GLshort)->()>;
pub type PFN_glMultiTexCoord4svARB = Option<unsafe extern "C" fn(target: GLenum, v: *const GLshort)->()>;

pub type PFN_PFNGLACTIVETEXTUREARBPROC = Option<unsafe extern "C" fn(texture: GLenum)->()>;
pub type PFN_PFNGLCLIENTACTIVETEXTUREARBPROC = Option<unsafe extern "C" fn(texture: GLenum)->()>;
pub type PFN_PFNGLMULTITEXCOORD1DARBPROC = Option<unsafe extern "C" fn(target: GLenum, s: GLdouble)->()>;
pub type PFN_PFNGLMULTITEXCOORD1DVARBPROC = Option<unsafe extern "C" fn(target: GLenum, v: *const GLdouble)->()>;
pub type PFN_PFNGLMULTITEXCOORD1FARBPROC = Option<unsafe extern "C" fn(target: GLenum, s: GLfloat)->()>;
pub type PFN_PFNGLMULTITEXCOORD1FVARBPROC = Option<unsafe extern "C" fn(target: GLenum, v: *const GLfloat)->()>;
pub type PFN_PFNGLMULTITEXCOORD1IARBPROC = Option<unsafe extern "C" fn(target: GLenum, s: GLint)->()>;
pub type PFN_PFNGLMULTITEXCOORD1IVARBPROC = Option<unsafe extern "C" fn(target: GLenum, v: *const GLint)->()>;
pub type PFN_PFNGLMULTITEXCOORD1SARBPROC = Option<unsafe extern "C" fn(target: GLenum, s:GLshort)->()>;
pub type PFN_PFNGLMULTITEXCOORD1SVARBPROC = Option<unsafe extern "C" fn(target: GLenum, v: *const GLshort)->()>;
pub type PFN_PFNGLMULTITEXCOORD2DARBPROC = Option<unsafe extern "C" fn(target: GLenum, s: GLdouble, t: GLdouble)->()>;
pub type PFN_PFNGLMULTITEXCOORD2DVARBPROC = Option<unsafe extern "C" fn(target: GLenum, v: *const GLdouble)->()>;
pub type PFN_PFNGLMULTITEXCOORD2FARBPROC = Option<unsafe extern "C" fn(target: GLenum, s: GLfloat, t: GLfloat)->()>;
pub type PFN_PFNGLMULTITEXCOORD2FVARBPROC = Option<unsafe extern "C" fn(target: GLenum, v: *const GLfloat)->()>;
pub type PFN_PFNGLMULTITEXCOORD2IARBPROC = Option<unsafe extern "C" fn(target: GLenum, s: GLint, t: GLint)->()>;
pub type PFN_PFNGLMULTITEXCOORD2IVARBPROC = Option<unsafe extern "C" fn(target: GLenum, v: *const GLint)->()>;
pub type PFN_PFNGLMULTITEXCOORD2SARBPROC = Option<unsafe extern "C" fn(target: GLenum, s:GLshort, t:GLshort)->()>;
pub type PFN_PFNGLMULTITEXCOORD2SVARBPROC = Option<unsafe extern "C" fn(target: GLenum, v: *const GLshort)->()>;
pub type PFN_PFNGLMULTITEXCOORD3DARBPROC = Option<unsafe extern "C" fn(target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble)->()>;
pub type PFN_PFNGLMULTITEXCOORD3DVARBPROC = Option<unsafe extern "C" fn(target: GLenum, v: *const GLdouble)->()>;
pub type PFN_PFNGLMULTITEXCOORD3FARBPROC = Option<unsafe extern "C" fn(target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat)->()>;
pub type PFN_PFNGLMULTITEXCOORD3FVARBPROC = Option<unsafe extern "C" fn(target: GLenum, v: *const GLfloat)->()>;
pub type PFN_PFNGLMULTITEXCOORD3IARBPROC = Option<unsafe extern "C" fn(target: GLenum, s: GLint, t: GLint, r: GLint)->()>;
pub type PFN_PFNGLMULTITEXCOORD3IVARBPROC = Option<unsafe extern "C" fn(target: GLenum, v: *const GLint)->()>;
pub type PFN_PFNGLMULTITEXCOORD3SARBPROC = Option<unsafe extern "C" fn(target: GLenum, s:GLshort, t:GLshort, r:GLshort)->()>;
pub type PFN_PFNGLMULTITEXCOORD3SVARBPROC = Option<unsafe extern "C" fn(target: GLenum, v: *const GLshort)->()>;
pub type PFN_PFNGLMULTITEXCOORD4DARBPROC = Option<unsafe extern "C" fn(target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble)->()>;
pub type PFN_PFNGLMULTITEXCOORD4DVARBPROC = Option<unsafe extern "C" fn(target: GLenum, v: *const GLdouble)->()>;
pub type PFN_PFNGLMULTITEXCOORD4FARBPROC = Option<unsafe extern "C" fn(target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat)->()>;
pub type PFN_PFNGLMULTITEXCOORD4FVARBPROC = Option<unsafe extern "C" fn(target: GLenum, v: *const GLfloat)->()>;
pub type PFN_PFNGLMULTITEXCOORD4IARBPROC = Option<unsafe extern "C" fn(target: GLenum, s: GLint, t: GLint, r: GLint, q: GLint)->()>;
pub type PFN_PFNGLMULTITEXCOORD4IVARBPROC = Option<unsafe extern "C" fn(target: GLenum, v: *const GLint)->()>;
pub type PFN_PFNGLMULTITEXCOORD4SARBPROC = Option<unsafe extern "C" fn(target: GLenum, s:GLshort, t:GLshort, r:GLshort, q:GLshort)->()>;
pub type PFN_PFNGLMULTITEXCOORD4SVARBPROC = Option<unsafe extern "C" fn(target: GLenum, v: *const GLshort)->()>;





// should I create my own libraries?
pub struct Backend {
    // glfw_instance: glfw::Glfw
}

pub struct CreateOpenGLArgs {
    #[cfg(unix)]
    display: *mut x11::xlib::Display,
    #[cfg(unix)]
    screen: std::os::raw::c_int
}

impl super::Backend for Backend {}
impl super::Initialize for Backend {
    type Args = CreateOpenGLArgs;
    fn try_initialize(
        args: Self::Args,
    ) -> Result<Box<dyn super::Backend>, Box<dyn std::error::Error>> {
        // load functions
        // let glfw = glfw::init_no_callbacks()?;
        let i = self::Backend {};
        Ok(Box::new(i))
    }
}

#[cfg(unix)]
#[test]
pub fn opengl_test_initialize() -> Result<(), Box<dyn std::error::Error>> {
    use x11::xlib::XScreenNumberOfScreen;

    use crate::gfx::Initialize;
    let mut display = unsafe {x11::xlib::XOpenDisplay(std::ptr::null())};
    let root_window = unsafe {x11::xlib::XDefaultRootWindow(display)};

    let mainwindow: x11::xlib::Window = unsafe {x11::xlib::XCreateSimpleWindow(display, root_window, 0, 0, 800, 600, 0, 0, 0x00aade87) };
    let mut attributes: x11::xlib::XWindowAttributes = unsafe {std::mem::zeroed()};
     let _ =  unsafe {x11::xlib::XGetWindowAttributes(display, mainwindow, &mut attributes)};
     let screen_number = unsafe {XScreenNumberOfScreen(attributes.screen)};
    let args = CreateOpenGLArgs {display:display.clone(),screen:screen_number};
    let instance = Backend::try_initialize(args)?;
    Ok(())
}
