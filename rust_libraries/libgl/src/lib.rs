pub mod types {
    use std::ffi::{c_uint,c_uchar,c_int,c_char,c_void,c_ushort,c_float,c_double};
    pub type GLenum = c_uint;
    pub type GLboolean = c_char;
    pub type GLbitfield = c_uint;
    pub type GLvoid = c_void;
    pub type GLint = c_int;
    pub type GLubyte = c_uchar;
    pub type GLushort = c_ushort;
    pub type GLuint = c_uint;
    pub type GLsizei = c_int;
    pub type GLfloat = c_float;
    pub type GLdouble = c_double;
    pub type GLclampd = c_double;

}
// boolean values
const GL_FALSE: types::GLenum = 0;
const GL_TRUE: types::GLenum = 1;

const GL_BYTE: types::GLenum = 0x1400;


const GL_UNSIGNED_BYTE: types::GLenum = 0x1401;
const GL_SHORT: types::GLenum = 0x1402;
const GL_UNSIGNED_SHORT: types::GLenum = 0x1403;
const GL_INT: types::GLenum = 0x1404;
const GL_UNSIGNED_INT: types::GLenum = 0x1405;
const GL_FLOAT: types::GLenum = 0x1406;
const GL_2_BYTES: types::GLenum = 0x1407;
const GL_3_BYTES: types::GLenum = 0x1408;
const GL_4_BYTES: types::GLenum = 0x1409;
const GL_DOUBLE: types::GLenum = 0x140A;

/* Primitives */
const GL_POINTS: types::GLenum = 0x0000;
const GL_LINES: types::GLenum = 0x0001;
const GL_LINE_LOOP: types::GLenum = 0x0002;
const GL_LINE_STRIP: types::GLenum = 0x0003;
const GL_TRIANGLES: types::GLenum = 0x0004;
const GL_TRIANGLE_STRIP: types::GLenum = 0x0005;
const GL_TRIANGLE_FAN: types::GLenum = 0x0006;
const GL_QUADS: types::GLenum = 0x0007;
const GL_QUAD_STRIP: types::GLenum = 0x0008;
const GL_POLYGON: types::GLenum = 0x0009;

/* Vertex Arrays */
const GL_VERTEX_ARRAY: types::GLenum = 0x8074;
const GL_NORMAL_ARRAY: types::GLenum = 0x8075;
const GL_COLOR_ARRAY: types::GLenum = 0x8076;
const GL_INDEX_ARRAY: types::GLenum = 0x8077;
const GL_TEXTURE_COORD_ARRAY: types::GLenum = 0x8078;
const GL_EDGE_FLAG_ARRAY: types::GLenum = 0x8079;
const GL_VERTEX_ARRAY_SIZE: types::GLenum = 0x807A;
const GL_VERTEX_ARRAY_TYPE: types::GLenum = 0x807B;
const GL_VERTEX_ARRAY_STRIDE: types::GLenum = 0x807C;
const GL_NORMAL_ARRAY_TYPE: types::GLenum = 0x807E;
const GL_NORMAL_ARRAY_STRIDE: types::GLenum = 0x807F;
const GL_COLOR_ARRAY_SIZE: types::GLenum = 0x8081;
const GL_COLOR_ARRAY_TYPE: types::GLenum = 0x8082;
const GL_COLOR_ARRAY_STRIDE: types::GLenum = 0x8083;
const GL_INDEX_ARRAY_TYPE: types::GLenum = 0x8085;
const GL_INDEX_ARRAY_STRIDE: types::GLenum = 0x8086;
const GL_TEXTURE_COORD_ARRAY_SIZE: types::GLenum = 0x8088;
const GL_TEXTURE_COORD_ARRAY_TYPE: types::GLenum = 0x8089;
const GL_TEXTURE_COORD_ARRAY_STRIDE: types::GLenum = 0x808A;
const GL_EDGE_FLAG_ARRAY_STRIDE: types::GLenum = 0x808C;
const GL_VERTEX_ARRAY_POINTER: types::GLenum = 0x808E;
const GL_NORMAL_ARRAY_POINTER: types::GLenum = 0x808F;
const GL_COLOR_ARRAY_POINTER: types::GLenum = 0x8090;
const GL_INDEX_ARRAY_POINTER: types::GLenum = 0x8091;
const GL_TEXTURE_COORD_ARRAY_POINTER: types::GLenum = 0x8092;
const GL_EDGE_FLAG_ARRAY_POINTER: types::GLenum = 0x8093;
const GL_V2F: types::GLenum = 0x2A20;
const GL_V3F: types::GLenum = 0x2A21;
const GL_C4UB_V2F: types::GLenum = 0x2A22;
const GL_C4UB_V3F: types::GLenum = 0x2A23;
const GL_C3F_V3F: types::GLenum = 0x2A24;
const GL_N3F_V3F: types::GLenum = 0x2A25;
const GL_C4F_N3F_V3F: types::GLenum = 0x2A26;
const GL_T2F_V3F: types::GLenum = 0x2A27;
const GL_T4F_V4F: types::GLenum = 0x2A28;
const GL_T2F_C4UB_V3F: types::GLenum = 0x2A29;
const GL_T2F_C3F_V3F: types::GLenum = 0x2A2A;
const GL_T2F_N3F_V3F: types::GLenum = 0x2A2B;
const GL_T2F_C4F_N3F_V3F: types::GLenum = 0x2A2C;
const GL_T4F_C4F_N3F_V4F: types::GLenum = 0x2A2D;

/* Matrix Mode */
const GL_MATRIX_MODE: types::GLenum = 0x0BA0;
const GL_MODELVIEW: types::GLenum = 0x1700;
const GL_PROJECTION: types::GLenum = 0x1701;
const GL_TEXTURE: types::GLenum = 0x1702;


/* Points */
const GL_POINT_SMOOTH: types::GLenum = 0x0B10;
const GL_POINT_SIZE: types::GLenum = 0x0B11;
const GL_POINT_SIZE_GRANULARITY: types::GLenum = 0x0B13;
const GL_POINT_SIZE_RANGE: types::GLenum = 0x0B12;
/* Lines */
const GL_LINE_SMOOTH: types::GLenum = 0x0B20;
const GL_LINE_STIPPLE: types::GLenum = 0x0B24;
const GL_LINE_STIPPLE_PATTERN: types::GLenum = 0x0B25;
const GL_LINE_STIPPLE_REPEAT: types::GLenum = 0x0B26;
const GL_LINE_WIDTH: types::GLenum = 0x0B21;
const GL_LINE_WIDTH_GRANULARITY: types::GLenum = 0x0B23;
const GL_LINE_WIDTH_RANGE: types::GLenum = 0x0B22;
/* Polygons */
const GL_POINT: types::GLenum = 0x1B00;
const GL_LINE: types::GLenum = 0x1B01;
const GL_FILL: types::GLenum = 0x1B02;
const GL_CW: types::GLenum = 0x0900;
const GL_CCW: types::GLenum = 0x0901;
const GL_FRONT: types::GLenum = 0x0404;
const GL_BACK: types::GLenum = 0x0405;
const GL_POLYGON_MODE: types::GLenum = 0x0B40;
const GL_POLYGON_SMOOTH: types::GLenum = 0x0B41;
const GL_POLYGON_STIPPLE: types::GLenum = 0x0B42;
const GL_EDGE_FLAG: types::GLenum = 0x0B43;
const GL_CULL_FACE: types::GLenum = 0x0B44;
const GL_CULL_FACE_MODE: types::GLenum = 0x0B45;
const GL_FRONT_FACE: types::GLenum = 0x0B46;
const GL_POLYGON_OFFSET_FACTOR: types::GLenum = 0x8038;
const GL_POLYGON_OFFSET_UNITS: types::GLenum = 0x2A00;
const GL_POLYGON_OFFSET_POINT: types::GLenum = 0x2A01;
const GL_POLYGON_OFFSET_LINE: types::GLenum = 0x2A02;
const GL_POLYGON_OFFSET_FILL: types::GLenum = 0x8037;
/* Display Lists */
const GL_COMPILE: types::GLenum = 0x1300;
const GL_COMPILE_AND_EXECUTE: types::GLenum = 0x1301;
const GL_LIST_BASE: types::GLenum = 0x0B32;
const GL_LIST_INDEX: types::GLenum = 0x0B33;
const GL_LIST_MODE: types::GLenum = 0x0B30;
/* Depth buffer */
const GL_NEVER: types::GLenum = 0x0200;
const GL_LESS: types::GLenum = 0x0201;
const GL_EQUAL: types::GLenum = 0x0202;
const GL_LEQUAL: types::GLenum = 0x0203;
const GL_GREATER: types::GLenum = 0x0204;
const GL_NOTEQUAL: types::GLenum = 0x0205;
const GL_GEQUAL: types::GLenum = 0x0206;
const GL_ALWAYS: types::GLenum = 0x0207;
const GL_DEPTH_TEST: types::GLenum = 0x0B71;
const GL_DEPTH_BITS: types::GLenum = 0x0D56;
const GL_DEPTH_CLEAR_VALUE: types::GLenum = 0x0B73;
const GL_DEPTH_FUNC: types::GLenum = 0x0B74;
const GL_DEPTH_RANGE: types::GLenum = 0x0B70;
const GL_DEPTH_WRITEMASK: types::GLenum = 0x0B72;
const GL_DEPTH_COMPONENT: types::GLenum = 0x1902;
/* Lighting */
const GL_LIGHTING: types::GLenum = 0x0B50;
const GL_LIGHT0: types::GLenum = 0x4000;
const GL_LIGHT1: types::GLenum = 0x4001;
const GL_LIGHT2: types::GLenum = 0x4002;
const GL_LIGHT3: types::GLenum = 0x4003;
const GL_LIGHT4: types::GLenum = 0x4004;
const GL_LIGHT5: types::GLenum = 0x4005;
const GL_LIGHT6: types::GLenum = 0x4006;
const GL_LIGHT7: types::GLenum = 0x4007;
const GL_SPOT_EXPONENT: types::GLenum = 0x1205;
const GL_SPOT_CUTOFF: types::GLenum = 0x1206;
const GL_CONSTANT_ATTENUATION: types::GLenum = 0x1207;
const GL_LINEAR_ATTENUATION: types::GLenum = 0x1208;
const GL_QUADRATIC_ATTENUATION: types::GLenum = 0x1209;
const GL_AMBIENT: types::GLenum = 0x1200;
const GL_DIFFUSE: types::GLenum = 0x1201;
const GL_SPECULAR: types::GLenum = 0x1202;
const GL_SHININESS: types::GLenum = 0x1601;
const GL_EMISSION: types::GLenum = 0x1600;
const GL_POSITION: types::GLenum = 0x1203;
const GL_SPOT_DIRECTION: types::GLenum = 0x1204;
const GL_AMBIENT_AND_DIFFUSE: types::GLenum = 0x1602;
const GL_COLOR_INDEXES: types::GLenum = 0x1603;
const GL_LIGHT_MODEL_TWO_SIDE: types::GLenum = 0x0B52;
const GL_LIGHT_MODEL_LOCAL_VIEWER: types::GLenum = 0x0B51;
const GL_LIGHT_MODEL_AMBIENT: types::GLenum = 0x0B53;
const GL_FRONT_AND_BACK: types::GLenum = 0x0408;
const GL_SHADE_MODEL: types::GLenum = 0x0B54;
const GL_FLAT: types::GLenum = 0x1D00;
const GL_SMOOTH: types::GLenum = 0x1D01;
const GL_COLOR_MATERIAL: types::GLenum = 0x0B57;
const GL_COLOR_MATERIAL_FACE: types::GLenum = 0x0B55;
const GL_COLOR_MATERIAL_PARAMETER: types::GLenum = 0x0B56;
const GL_NORMALIZE: types::GLenum = 0x0BA1;
/* User clipping planes */
const GL_CLIP_PLANE0: types::GLenum = 0x3000;
const GL_CLIP_PLANE1: types::GLenum = 0x3001;
const GL_CLIP_PLANE2: types::GLenum = 0x3002;
const GL_CLIP_PLANE3: types::GLenum = 0x3003;
const GL_CLIP_PLANE4: types::GLenum = 0x3004;
const GL_CLIP_PLANE5: types::GLenum = 0x3005;
/* Accumulation buffer */
const GL_ACCUM_RED_BITS: types::GLenum = 0x0D58;
const GL_ACCUM_GREEN_BITS: types::GLenum = 0x0D59;
const GL_ACCUM_BLUE_BITS: types::GLenum = 0x0D5A;
const GL_ACCUM_ALPHA_BITS: types::GLenum = 0x0D5B;
const GL_ACCUM_CLEAR_VALUE: types::GLenum = 0x0B80;
const GL_ACCUM: types::GLenum = 0x0100;
const GL_ADD: types::GLenum = 0x0104;
const GL_LOAD: types::GLenum = 0x0101;
const GL_MULT: types::GLenum = 0x0103;
const GL_RETURN: types::GLenum = 0x0102;
/* Alpha testing */
const GL_ALPHA_TEST: types::GLenum = 0x0BC0;
const GL_ALPHA_TEST_REF: types::GLenum = 0x0BC2;
const GL_ALPHA_TEST_FUNC: types::GLenum = 0x0BC1;
/* Blending */
const GL_BLEND: types::GLenum = 0x0BE2;
const GL_BLEND_SRC: types::GLenum = 0x0BE1;
const GL_BLEND_DST: types::GLenum = 0x0BE0;
const GL_ZERO: types::GLenum = 0;
const GL_ONE: types::GLenum = 1;
const GL_SRC_COLOR: types::GLenum = 0x0300;
const GL_ONE_MINUS_SRC_COLOR: types::GLenum = 0x0301;
const GL_SRC_ALPHA: types::GLenum = 0x0302;
const GL_ONE_MINUS_SRC_ALPHA: types::GLenum = 0x0303;
const GL_DST_ALPHA: types::GLenum = 0x0304;
const GL_ONE_MINUS_DST_ALPHA: types::GLenum = 0x0305;
const GL_DST_COLOR: types::GLenum = 0x0306;
const GL_ONE_MINUS_DST_COLOR: types::GLenum = 0x0307;
const GL_SRC_ALPHA_SATURATE: types::GLenum = 0x0308;
/* Render Mode */
const GL_FEEDBACK: types::GLenum = 0x1C01;
const GL_RENDER: types::GLenum = 0x1C00;
const GL_SELECT: types::GLenum = 0x1C02;
/* Feedback */
const GL_2D: types::GLenum = 0x0600;
const GL_3D: types::GLenum = 0x0601;
const GL_3D_COLOR: types::GLenum = 0x0602;
const GL_3D_COLOR_TEXTURE: types::GLenum = 0x0603;
const GL_4D_COLOR_TEXTURE: types::GLenum = 0x0604;
const GL_POINT_TOKEN: types::GLenum = 0x0701;
const GL_LINE_TOKEN: types::GLenum = 0x0702;
const GL_LINE_RESET_TOKEN: types::GLenum = 0x0707;
const GL_POLYGON_TOKEN: types::GLenum = 0x0703;
const GL_BITMAP_TOKEN: types::GLenum = 0x0704;
const GL_DRAW_PIXEL_TOKEN: types::GLenum = 0x0705;
const GL_COPY_PIXEL_TOKEN: types::GLenum = 0x0706;
const GL_PASS_THROUGH_TOKEN: types::GLenum = 0x0700;
const GL_FEEDBACK_BUFFER_POINTER: types::GLenum = 0x0DF0;
const GL_FEEDBACK_BUFFER_SIZE: types::GLenum = 0x0DF1;
const GL_FEEDBACK_BUFFER_TYPE: types::GLenum = 0x0DF2;
/* Selection */
const GL_SELECTION_BUFFER_POINTER: types::GLenum = 0x0DF3;
const GL_SELECTION_BUFFER_SIZE: types::GLenum = 0x0DF4;
/* Fog */
const GL_FOG: types::GLenum = 0x0B60;
const GL_FOG_MODE: types::GLenum = 0x0B65;
const GL_FOG_DENSITY: types::GLenum = 0x0B62;
const GL_FOG_COLOR: types::GLenum = 0x0B66;
const GL_FOG_INDEX: types::GLenum = 0x0B61;
const GL_FOG_START: types::GLenum = 0x0B63;
const GL_FOG_END: types::GLenum = 0x0B64;
const GL_LINEAR: types::GLenum = 0x2601;
const GL_EXP: types::GLenum = 0x0800;
const GL_EXP2: types::GLenum = 0x0801;
/* Logic Ops */
const GL_LOGIC_OP: types::GLenum = 0x0BF1;
const GL_INDEX_LOGIC_OP: types::GLenum = 0x0BF1;
const GL_COLOR_LOGIC_OP: types::GLenum = 0x0BF2;
const GL_LOGIC_OP_MODE: types::GLenum = 0x0BF0;
const GL_CLEAR: types::GLenum = 0x1500;
const GL_SET: types::GLenum = 0x150F;
const GL_COPY: types::GLenum = 0x1503;
const GL_COPY_INVERTED: types::GLenum = 0x150C;
const GL_NOOP: types::GLenum = 0x1505;
const GL_INVERT: types::GLenum = 0x150A;
const GL_AND: types::GLenum = 0x1501;
const GL_NAND: types::GLenum = 0x150E;
const GL_OR: types::GLenum = 0x1507;
const GL_NOR: types::GLenum = 0x1508;
const GL_XOR: types::GLenum = 0x1506;
const GL_EQUIV: types::GLenum = 0x1509;
const GL_AND_REVERSE: types::GLenum = 0x1502;
const GL_AND_INVERTED: types::GLenum = 0x1504;
const GL_OR_REVERSE: types::GLenum = 0x150B;
const GL_OR_INVERTED: types::GLenum = 0x150D;
/* Stencil */
const GL_STENCIL_BITS: types::GLenum = 0x0D57;
const GL_STENCIL_TEST: types::GLenum = 0x0B90;
const GL_STENCIL_CLEAR_VALUE: types::GLenum = 0x0B91;
const GL_STENCIL_FUNC: types::GLenum = 0x0B92;
const GL_STENCIL_VALUE_MASK: types::GLenum = 0x0B93;
const GL_STENCIL_FAIL: types::GLenum = 0x0B94;
const GL_STENCIL_PASS_DEPTH_FAIL: types::GLenum = 0x0B95;
const GL_STENCIL_PASS_DEPTH_PASS: types::GLenum = 0x0B96;
const GL_STENCIL_REF: types::GLenum = 0x0B97;
const GL_STENCIL_WRITEMASK: types::GLenum = 0x0B98;
const GL_STENCIL_INDEX: types::GLenum = 0x1901;
const GL_KEEP: types::GLenum = 0x1E00;
const GL_REPLACE: types::GLenum = 0x1E01;
const GL_INCR: types::GLenum = 0x1E02;
const GL_DECR: types::GLenum = 0x1E03;
/* Buffers, Pixel Drawing/Reading */
const GL_NONE: types::GLenum = 0;
const GL_LEFT: types::GLenum = 0x0406;
const GL_RIGHT: types::GLenum = 0x0407;
/*GL_FRONT					0x0404 */
/*GL_BACK					0x0405 */
/*GL_FRONT_AND_BACK				0x0408 */
const GL_FRONT_LEFT: types::GLenum = 0x0400;
const GL_FRONT_RIGHT: types::GLenum = 0x0401;
const GL_BACK_LEFT: types::GLenum = 0x0402;
const GL_BACK_RIGHT: types::GLenum = 0x0403;
const GL_AUX0: types::GLenum = 0x0409;
const GL_AUX1: types::GLenum = 0x040A;
const GL_AUX2: types::GLenum = 0x040B;
const GL_AUX3: types::GLenum = 0x040C;
const GL_COLOR_INDEX: types::GLenum = 0x1900;
const GL_RED: types::GLenum = 0x1903;
const GL_GREEN: types::GLenum = 0x1904;
const GL_BLUE: types::GLenum = 0x1905;
const GL_ALPHA: types::GLenum = 0x1906;
const GL_LUMINANCE: types::GLenum = 0x1909;
const GL_LUMINANCE_ALPHA: types::GLenum = 0x190A;
const GL_ALPHA_BITS: types::GLenum = 0x0D55;
const GL_RED_BITS: types::GLenum = 0x0D52;
const GL_GREEN_BITS: types::GLenum = 0x0D53;
const GL_BLUE_BITS: types::GLenum = 0x0D54;
const GL_INDEX_BITS: types::GLenum = 0x0D51;
const GL_SUBPIXEL_BITS: types::GLenum = 0x0D50;
const GL_AUX_BUFFERS: types::GLenum = 0x0C00;
const GL_READ_BUFFER: types::GLenum = 0x0C02;
const GL_DRAW_BUFFER: types::GLenum = 0x0C01;
const GL_DOUBLEBUFFER: types::GLenum = 0x0C32;
const GL_STEREO: types::GLenum = 0x0C33;
const GL_BITMAP: types::GLenum = 0x1A00;
const GL_COLOR: types::GLenum = 0x1800;
const GL_DEPTH: types::GLenum = 0x1801;
const GL_STENCIL: types::GLenum = 0x1802;
const GL_DITHER: types::GLenum = 0x0BD0;
const GL_RGB: types::GLenum = 0x1907;
const GL_RGBA: types::GLenum = 0x1908;
/* Implementation limits */
const GL_MAX_LIST_NESTING: types::GLenum = 0x0B31;
const GL_MAX_EVAL_ORDER: types::GLenum = 0x0D30;
const GL_MAX_LIGHTS: types::GLenum = 0x0D31;
const GL_MAX_CLIP_PLANES: types::GLenum = 0x0D32;
const GL_MAX_TEXTURE_SIZE: types::GLenum = 0x0D33;
const GL_MAX_PIXEL_MAP_TABLE: types::GLenum = 0x0D34;
const GL_MAX_ATTRIB_STACK_DEPTH: types::GLenum = 0x0D35;
const GL_MAX_MODELVIEW_STACK_DEPTH: types::GLenum = 0x0D36;
const GL_MAX_NAME_STACK_DEPTH: types::GLenum = 0x0D37;
const GL_MAX_PROJECTION_STACK_DEPTH: types::GLenum = 0x0D38;
const GL_MAX_TEXTURE_STACK_DEPTH: types::GLenum = 0x0D39;
const GL_MAX_VIEWPORT_DIMS: types::GLenum = 0x0D3A;
const GL_MAX_CLIENT_ATTRIB_STACK_DEPTH: types::GLenum = 0x0D3B;
/* Gets */
const GL_ATTRIB_STACK_DEPTH: types::GLenum = 0x0BB0;
const GL_CLIENT_ATTRIB_STACK_DEPTH: types::GLenum = 0x0BB1;
const GL_COLOR_CLEAR_VALUE: types::GLenum = 0x0C22;
const GL_COLOR_WRITEMASK: types::GLenum = 0x0C23;
const GL_CURRENT_INDEX: types::GLenum = 0x0B01;
const GL_CURRENT_COLOR: types::GLenum = 0x0B00;
const GL_CURRENT_NORMAL: types::GLenum = 0x0B02;
const GL_CURRENT_RASTER_COLOR: types::GLenum = 0x0B04;
const GL_CURRENT_RASTER_DISTANCE: types::GLenum = 0x0B09;
const GL_CURRENT_RASTER_INDEX: types::GLenum = 0x0B05;
const GL_CURRENT_RASTER_POSITION: types::GLenum = 0x0B07;
const GL_CURRENT_RASTER_TEXTURE_COORDS: types::GLenum = 0x0B06;
const GL_CURRENT_RASTER_POSITION_VALID: types::GLenum = 0x0B08;
const GL_CURRENT_TEXTURE_COORDS: types::GLenum = 0x0B03;
const GL_INDEX_CLEAR_VALUE: types::GLenum = 0x0C20;
const GL_INDEX_MODE: types::GLenum = 0x0C30;
const GL_INDEX_WRITEMASK: types::GLenum = 0x0C21;
const GL_MODELVIEW_MATRIX: types::GLenum = 0x0BA6;
const GL_MODELVIEW_STACK_DEPTH: types::GLenum = 0x0BA3;
const GL_NAME_STACK_DEPTH: types::GLenum = 0x0D70;
const GL_PROJECTION_MATRIX: types::GLenum = 0x0BA7;
const GL_PROJECTION_STACK_DEPTH: types::GLenum = 0x0BA4;
const GL_RENDER_MODE: types::GLenum = 0x0C40;
const GL_RGBA_MODE: types::GLenum = 0x0C31;
const GL_TEXTURE_MATRIX: types::GLenum = 0x0BA8;
const GL_TEXTURE_STACK_DEPTH: types::GLenum = 0x0BA5;
const GL_VIEWPORT: types::GLenum = 0x0BA2;
/* Evaluators */
const GL_AUTO_NORMAL: types::GLenum = 0x0D80;
const GL_MAP1_COLOR_4: types::GLenum = 0x0D90;
const GL_MAP1_INDEX: types::GLenum = 0x0D91;
const GL_MAP1_NORMAL: types::GLenum = 0x0D92;
const GL_MAP1_TEXTURE_COORD_1: types::GLenum = 0x0D93;
const GL_MAP1_TEXTURE_COORD_2: types::GLenum = 0x0D94;
const GL_MAP1_TEXTURE_COORD_3: types::GLenum = 0x0D95;
const GL_MAP1_TEXTURE_COORD_4: types::GLenum = 0x0D96;
const GL_MAP1_VERTEX_3: types::GLenum = 0x0D97;
const GL_MAP1_VERTEX_4: types::GLenum = 0x0D98;
const GL_MAP2_COLOR_4: types::GLenum = 0x0DB0;
const GL_MAP2_INDEX: types::GLenum = 0x0DB1;
const GL_MAP2_NORMAL: types::GLenum = 0x0DB2;
const GL_MAP2_TEXTURE_COORD_1: types::GLenum = 0x0DB3;
const GL_MAP2_TEXTURE_COORD_2: types::GLenum = 0x0DB4;
const GL_MAP2_TEXTURE_COORD_3: types::GLenum = 0x0DB5;
const GL_MAP2_TEXTURE_COORD_4: types::GLenum = 0x0DB6;
const GL_MAP2_VERTEX_3: types::GLenum = 0x0DB7;
const GL_MAP2_VERTEX_4: types::GLenum = 0x0DB8;
const GL_MAP1_GRID_DOMAIN: types::GLenum = 0x0DD0;
const GL_MAP1_GRID_SEGMENTS: types::GLenum = 0x0DD1;
const GL_MAP2_GRID_DOMAIN: types::GLenum = 0x0DD2;
const GL_MAP2_GRID_SEGMENTS: types::GLenum = 0x0DD3;
const GL_COEFF: types::GLenum = 0x0A00;
const GL_ORDER: types::GLenum = 0x0A01;
const GL_DOMAIN: types::GLenum = 0x0A02;
/* Hints */
const GL_PERSPECTIVE_CORRECTION_HINT: types::GLenum = 0x0C50;
const GL_POINT_SMOOTH_HINT: types::GLenum = 0x0C51;
const GL_LINE_SMOOTH_HINT: types::GLenum = 0x0C52;
const GL_POLYGON_SMOOTH_HINT: types::GLenum = 0x0C53;
const GL_FOG_HINT: types::GLenum = 0x0C54;
const GL_DONT_CARE: types::GLenum = 0x1100;
const GL_FASTEST: types::GLenum = 0x1101;
const GL_NICEST: types::GLenum = 0x1102;
/* Scissor box */
const GL_SCISSOR_BOX: types::GLenum = 0x0C10;
const GL_SCISSOR_TEST: types::GLenum = 0x0C11;
/* Pixel Mode / Transfer */
const GL_MAP_COLOR: types::GLenum = 0x0D10;
const GL_MAP_STENCIL: types::GLenum = 0x0D11;
const GL_INDEX_SHIFT: types::GLenum = 0x0D12;
const GL_INDEX_OFFSET: types::GLenum = 0x0D13;
const GL_RED_SCALE: types::GLenum = 0x0D14;
const GL_RED_BIAS: types::GLenum = 0x0D15;
const GL_GREEN_SCALE: types::GLenum = 0x0D18;
const GL_GREEN_BIAS: types::GLenum = 0x0D19;
const GL_BLUE_SCALE: types::GLenum = 0x0D1A;
const GL_BLUE_BIAS: types::GLenum = 0x0D1B;
const GL_ALPHA_SCALE: types::GLenum = 0x0D1C;
const GL_ALPHA_BIAS: types::GLenum = 0x0D1D;
const GL_DEPTH_SCALE: types::GLenum = 0x0D1E;
const GL_DEPTH_BIAS: types::GLenum = 0x0D1F;
const GL_PIXEL_MAP_S_TO_S_SIZE: types::GLenum = 0x0CB1;
const GL_PIXEL_MAP_I_TO_I_SIZE: types::GLenum = 0x0CB0;
const GL_PIXEL_MAP_I_TO_R_SIZE: types::GLenum = 0x0CB2;
const GL_PIXEL_MAP_I_TO_G_SIZE: types::GLenum = 0x0CB3;
const GL_PIXEL_MAP_I_TO_B_SIZE: types::GLenum = 0x0CB4;
const GL_PIXEL_MAP_I_TO_A_SIZE: types::GLenum = 0x0CB5;
const GL_PIXEL_MAP_R_TO_R_SIZE: types::GLenum = 0x0CB6;
const GL_PIXEL_MAP_G_TO_G_SIZE: types::GLenum = 0x0CB7;
const GL_PIXEL_MAP_B_TO_B_SIZE: types::GLenum = 0x0CB8;
const GL_PIXEL_MAP_A_TO_A_SIZE: types::GLenum = 0x0CB9;
const GL_PIXEL_MAP_S_TO_S: types::GLenum = 0x0C71;
const GL_PIXEL_MAP_I_TO_I: types::GLenum = 0x0C70;
const GL_PIXEL_MAP_I_TO_R: types::GLenum = 0x0C72;
const GL_PIXEL_MAP_I_TO_G: types::GLenum = 0x0C73;
const GL_PIXEL_MAP_I_TO_B: types::GLenum = 0x0C74;
const GL_PIXEL_MAP_I_TO_A: types::GLenum = 0x0C75;
const GL_PIXEL_MAP_R_TO_R: types::GLenum = 0x0C76;
const GL_PIXEL_MAP_G_TO_G: types::GLenum = 0x0C77;
const GL_PIXEL_MAP_B_TO_B: types::GLenum = 0x0C78;
const GL_PIXEL_MAP_A_TO_A: types::GLenum = 0x0C79;
const GL_PACK_ALIGNMENT: types::GLenum = 0x0D05;
const GL_PACK_LSB_FIRST: types::GLenum = 0x0D01;
const GL_PACK_ROW_LENGTH: types::GLenum = 0x0D02;
const GL_PACK_SKIP_PIXELS: types::GLenum = 0x0D04;
const GL_PACK_SKIP_ROWS: types::GLenum = 0x0D03;
const GL_PACK_SWAP_BYTES: types::GLenum = 0x0D00;
const GL_UNPACK_ALIGNMENT: types::GLenum = 0x0CF5;
const GL_UNPACK_LSB_FIRST: types::GLenum = 0x0CF1;
const GL_UNPACK_ROW_LENGTH: types::GLenum = 0x0CF2;
const GL_UNPACK_SKIP_PIXELS: types::GLenum = 0x0CF4;
const GL_UNPACK_SKIP_ROWS: types::GLenum = 0x0CF3;
const GL_UNPACK_SWAP_BYTES: types::GLenum = 0x0CF0;
const GL_ZOOM_X: types::GLenum = 0x0D16;
const GL_ZOOM_Y: types::GLenum = 0x0D17;
/* Texture mapping */
const GL_TEXTURE_ENV: types::GLenum = 0x2300;
const GL_TEXTURE_ENV_MODE: types::GLenum = 0x2200;
const GL_TEXTURE_1D: types::GLenum = 0x0DE0;
const GL_TEXTURE_2D: types::GLenum = 0x0DE1;
const GL_TEXTURE_WRAP_S: types::GLenum = 0x2802;
const GL_TEXTURE_WRAP_T: types::GLenum = 0x2803;
const GL_TEXTURE_MAG_FILTER: types::GLenum = 0x2800;
const GL_TEXTURE_MIN_FILTER: types::GLenum = 0x2801;
const GL_TEXTURE_ENV_COLOR: types::GLenum = 0x2201;
const GL_TEXTURE_GEN_S: types::GLenum = 0x0C60;
const GL_TEXTURE_GEN_T: types::GLenum = 0x0C61;
const GL_TEXTURE_GEN_R: types::GLenum = 0x0C62;
const GL_TEXTURE_GEN_Q: types::GLenum = 0x0C63;
const GL_TEXTURE_GEN_MODE: types::GLenum = 0x2500;
const GL_TEXTURE_BORDER_COLOR: types::GLenum = 0x1004;
const GL_TEXTURE_WIDTH: types::GLenum = 0x1000;
const GL_TEXTURE_HEIGHT: types::GLenum = 0x1001;
const GL_TEXTURE_BORDER: types::GLenum = 0x1005;
const GL_TEXTURE_COMPONENTS: types::GLenum = 0x1003;
const GL_TEXTURE_RED_SIZE: types::GLenum = 0x805C;
const GL_TEXTURE_GREEN_SIZE: types::GLenum = 0x805D;
const GL_TEXTURE_BLUE_SIZE: types::GLenum = 0x805E;
const GL_TEXTURE_ALPHA_SIZE: types::GLenum = 0x805F;
const GL_TEXTURE_LUMINANCE_SIZE: types::GLenum = 0x8060;
const GL_TEXTURE_INTENSITY_SIZE: types::GLenum = 0x8061;
const GL_NEAREST_MIPMAP_NEAREST: types::GLenum = 0x2700;
const GL_NEAREST_MIPMAP_LINEAR: types::GLenum = 0x2702;
const GL_LINEAR_MIPMAP_NEAREST: types::GLenum = 0x2701;
const GL_LINEAR_MIPMAP_LINEAR: types::GLenum = 0x2703;
const GL_OBJECT_LINEAR: types::GLenum = 0x2401;
const GL_OBJECT_PLANE: types::GLenum = 0x2501;
const GL_EYE_LINEAR: types::GLenum = 0x2400;
const GL_EYE_PLANE: types::GLenum = 0x2502;
const GL_SPHERE_MAP: types::GLenum = 0x2402;
const GL_DECAL: types::GLenum = 0x2101;
const GL_MODULATE: types::GLenum = 0x2100;
const GL_NEAREST: types::GLenum = 0x2600;
const GL_REPEAT: types::GLenum = 0x2901;
const GL_CLAMP: types::GLenum = 0x2900;
const GL_S: types::GLenum = 0x2000;
const GL_T: types::GLenum = 0x2001;
const GL_R: types::GLenum = 0x2002;
const GL_Q: types::GLenum = 0x2003;
/* Utility */
const GL_VENDOR: types::GLenum = 0x1F00;
const GL_RENDERER: types::GLenum = 0x1F01;
const GL_VERSION: types::GLenum = 0x1F02;
const GL_EXTENSIONS: types::GLenum = 0x1F03;
/* Errors */
const GL_NO_ERROR: types::GLenum = 0;
const GL_INVALID_ENUM: types::GLenum = 0x0500;
const GL_INVALID_VALUE: types::GLenum = 0x0501;
const GL_INVALID_OPERATION: types::GLenum = 0x0502;
const GL_STACK_OVERFLOW: types::GLenum = 0x0503;
const GL_STACK_UNDERFLOW: types::GLenum = 0x0504;
const GL_OUT_OF_MEMORY: types::GLenum = 0x0505;
/* glPush/PopAttrib bits */
const GL_CURRENT_BIT: types::GLenum = 0x00000001;
const GL_POINT_BIT: types::GLenum = 0x00000002;
const GL_LINE_BIT: types::GLenum = 0x00000004;
const GL_POLYGON_BIT: types::GLenum = 0x00000008;
const GL_POLYGON_STIPPLE_BIT: types::GLenum = 0x00000010;
const GL_PIXEL_MODE_BIT: types::GLenum = 0x00000020;
const GL_LIGHTING_BIT: types::GLenum = 0x00000040;
const GL_FOG_BIT: types::GLenum = 0x00000080;
const GL_DEPTH_BUFFER_BIT: types::GLenum = 0x00000100;
const GL_ACCUM_BUFFER_BIT: types::GLenum = 0x00000200;
const GL_STENCIL_BUFFER_BIT: types::GLenum = 0x00000400;
const GL_VIEWPORT_BIT: types::GLenum = 0x00000800;
const GL_TRANSFORM_BIT: types::GLenum = 0x00001000;
const GL_ENABLE_BIT: types::GLenum = 0x00002000;
const GL_COLOR_BUFFER_BIT: types::GLenum = 0x00004000;
const GL_HINT_BIT: types::GLenum = 0x00008000;
const GL_EVAL_BIT: types::GLenum = 0x00010000;
const GL_LIST_BIT: types::GLenum = 0x00020000;
const GL_TEXTURE_BIT: types::GLenum = 0x00040000;
const GL_SCISSOR_BIT: types::GLenum = 0x00080000;
const GL_ALL_ATTRIB_BITS: types::GLenum = 0xFFFFFFFF;



#[cfg(unix)]
#[link(name="libGL")]
pub unsafe extern "C" {
    pub fn glXGetProcAddress(proc_name: *const types::GLubyte) -> *const types::GLvoid;
}


// minimum supported OPENGL 1_1
pub fn load_gl_function(name: &str) -> *mut types:: GLvoid {
    let cstring = std::ffi::CString::new(name).unwrap_or_default();
    #[cfg(target_os="windows")] {
        let cstring_bytes = cstring.as_bytes_with_nul();
        let ptr = windows::core::PCSTR(cstring_bytes.as_ptr());
        let fn_ptr = windows::Win32::Graphics::OpenGL::wglGetProcAddress(ptr);
        std::mem::transmute::<_,*mut types::GLvoid>(fn_ptr)
    }
    #[cfg(unix)] {

    }
}




/*
 * Miscellaneous
 */

GLAPI void GLAPIENTRY glClearIndex( GLfloat c );

GLAPI void GLAPIENTRY glClearColor( GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha );

GLAPI void GLAPIENTRY glClear( GLbitfield mask );

GLAPI void GLAPIENTRY glIndexMask( GLuint mask );

GLAPI void GLAPIENTRY glColorMask( GLboolean red, GLboolean green, GLboolean blue, GLboolean alpha );

GLAPI void GLAPIENTRY glAlphaFunc( GLenum func, GLclampf ref );

GLAPI void GLAPIENTRY glBlendFunc( GLenum sfactor, GLenum dfactor );

GLAPI void GLAPIENTRY glLogicOp( GLenum opcode );

GLAPI void GLAPIENTRY glCullFace( GLenum mode );

GLAPI void GLAPIENTRY glFrontFace( GLenum mode );

GLAPI void GLAPIENTRY glPointSize( GLfloat size );

GLAPI void GLAPIENTRY glLineWidth( GLfloat width );

GLAPI void GLAPIENTRY glLineStipple( GLint factor, GLushort pattern );

GLAPI void GLAPIENTRY glPolygonMode( GLenum face, GLenum mode );

GLAPI void GLAPIENTRY glPolygonOffset( GLfloat factor, GLfloat units );

GLAPI void GLAPIENTRY glPolygonStipple( const GLubyte *mask );

GLAPI void GLAPIENTRY glGetPolygonStipple( GLubyte *mask );

GLAPI void GLAPIENTRY glEdgeFlag( GLboolean flag );

GLAPI void GLAPIENTRY glEdgeFlagv( const GLboolean *flag );

GLAPI void GLAPIENTRY glScissor( GLint x, GLint y, GLsizei width, GLsizei height);

GLAPI void GLAPIENTRY glClipPlane( GLenum plane, const GLdouble *equation );

GLAPI void GLAPIENTRY glGetClipPlane( GLenum plane, GLdouble *equation );

GLAPI void GLAPIENTRY glDrawBuffer( GLenum mode );

GLAPI void GLAPIENTRY glReadBuffer( GLenum mode );

GLAPI void GLAPIENTRY glEnable( GLenum cap );

GLAPI void GLAPIENTRY glDisable( GLenum cap );

GLAPI GLboolean GLAPIENTRY glIsEnabled( GLenum cap );


GLAPI void GLAPIENTRY glEnableClientState( GLenum cap );  /* 1.1 */

GLAPI void GLAPIENTRY glDisableClientState( GLenum cap );  /* 1.1 */


GLAPI void GLAPIENTRY glGetBooleanv( GLenum pname, GLboolean *params );

GLAPI void GLAPIENTRY glGetDoublev( GLenum pname, GLdouble *params );

GLAPI void GLAPIENTRY glGetFloatv( GLenum pname, GLfloat *params );

GLAPI void GLAPIENTRY glGetIntegerv( GLenum pname, GLint *params );


GLAPI void GLAPIENTRY glPushAttrib( GLbitfield mask );

GLAPI void GLAPIENTRY glPopAttrib( void );


GLAPI void GLAPIENTRY glPushClientAttrib( GLbitfield mask );  /* 1.1 */

GLAPI void GLAPIENTRY glPopClientAttrib( void );  /* 1.1 */


GLAPI GLint GLAPIENTRY glRenderMode( GLenum mode );

GLAPI GLenum GLAPIENTRY glGetError( void );

GLAPI const GLubyte * GLAPIENTRY glGetString( GLenum name );

GLAPI void GLAPIENTRY glFinish( void );

GLAPI void GLAPIENTRY glFlush( void );

GLAPI void GLAPIENTRY glHint( GLenum target, GLenum mode );


/*
 * Depth Buffer
 */

GLAPI void GLAPIENTRY glClearDepth( GLclampd depth );

GLAPI void GLAPIENTRY glDepthFunc( GLenum func );

GLAPI void GLAPIENTRY glDepthMask( GLboolean flag );

GLAPI void GLAPIENTRY glDepthRange( GLclampd near_val, GLclampd far_val );


/*
 * Accumulation Buffer
 */

GLAPI void GLAPIENTRY glClearAccum( GLfloat red, GLfloat green, GLfloat blue, GLfloat alpha );

GLAPI void GLAPIENTRY glAccum( GLenum op, GLfloat value );


/*
 * Transformation
 */

GLAPI void GLAPIENTRY glMatrixMode( GLenum mode );

GLAPI void GLAPIENTRY glOrtho( GLdouble left, GLdouble right,
                                 GLdouble bottom, GLdouble top,
                                 GLdouble near_val, GLdouble far_val );

GLAPI void GLAPIENTRY glFrustum( GLdouble left, GLdouble right,
                                   GLdouble bottom, GLdouble top,
                                   GLdouble near_val, GLdouble far_val );

GLAPI void GLAPIENTRY glViewport( GLint x, GLint y,
                                    GLsizei width, GLsizei height );

GLAPI void GLAPIENTRY glPushMatrix( void );

GLAPI void GLAPIENTRY glPopMatrix( void );

GLAPI void GLAPIENTRY glLoadIdentity( void );

GLAPI void GLAPIENTRY glLoadMatrixd( const GLdouble *m );
GLAPI void GLAPIENTRY glLoadMatrixf( const GLfloat *m );

GLAPI void GLAPIENTRY glMultMatrixd( const GLdouble *m );
GLAPI void GLAPIENTRY glMultMatrixf( const GLfloat *m );

GLAPI void GLAPIENTRY glRotated( GLdouble angle,
                                   GLdouble x, GLdouble y, GLdouble z );
GLAPI void GLAPIENTRY glRotatef( GLfloat angle,
                                   GLfloat x, GLfloat y, GLfloat z );

GLAPI void GLAPIENTRY glScaled( GLdouble x, GLdouble y, GLdouble z );
GLAPI void GLAPIENTRY glScalef( GLfloat x, GLfloat y, GLfloat z );

GLAPI void GLAPIENTRY glTranslated( GLdouble x, GLdouble y, GLdouble z );
GLAPI void GLAPIENTRY glTranslatef( GLfloat x, GLfloat y, GLfloat z );


/*
 * Display Lists
 */

GLAPI GLboolean GLAPIENTRY glIsList( GLuint list );

GLAPI void GLAPIENTRY glDeleteLists( GLuint list, GLsizei range );

GLAPI GLuint GLAPIENTRY glGenLists( GLsizei range );

GLAPI void GLAPIENTRY glNewList( GLuint list, GLenum mode );

GLAPI void GLAPIENTRY glEndList( void );

GLAPI void GLAPIENTRY glCallList( GLuint list );

GLAPI void GLAPIENTRY glCallLists( GLsizei n, GLenum type,
                                     const GLvoid *lists );

GLAPI void GLAPIENTRY glListBase( GLuint base );


/*
 * Drawing Functions
 */

GLAPI void GLAPIENTRY glBegin( GLenum mode );

GLAPI void GLAPIENTRY glEnd( void );


GLAPI void GLAPIENTRY glVertex2d( GLdouble x, GLdouble y );
GLAPI void GLAPIENTRY glVertex2f( GLfloat x, GLfloat y );
GLAPI void GLAPIENTRY glVertex2i( GLint x, GLint y );
GLAPI void GLAPIENTRY glVertex2s( GLshort x, GLshort y );

GLAPI void GLAPIENTRY glVertex3d( GLdouble x, GLdouble y, GLdouble z );
GLAPI void GLAPIENTRY glVertex3f( GLfloat x, GLfloat y, GLfloat z );
GLAPI void GLAPIENTRY glVertex3i( GLint x, GLint y, GLint z );
GLAPI void GLAPIENTRY glVertex3s( GLshort x, GLshort y, GLshort z );

GLAPI void GLAPIENTRY glVertex4d( GLdouble x, GLdouble y, GLdouble z, GLdouble w );
GLAPI void GLAPIENTRY glVertex4f( GLfloat x, GLfloat y, GLfloat z, GLfloat w );
GLAPI void GLAPIENTRY glVertex4i( GLint x, GLint y, GLint z, GLint w );
GLAPI void GLAPIENTRY glVertex4s( GLshort x, GLshort y, GLshort z, GLshort w );

GLAPI void GLAPIENTRY glVertex2dv( const GLdouble *v );
GLAPI void GLAPIENTRY glVertex2fv( const GLfloat *v );
GLAPI void GLAPIENTRY glVertex2iv( const GLint *v );
GLAPI void GLAPIENTRY glVertex2sv( const GLshort *v );

GLAPI void GLAPIENTRY glVertex3dv( const GLdouble *v );
GLAPI void GLAPIENTRY glVertex3fv( const GLfloat *v );
GLAPI void GLAPIENTRY glVertex3iv( const GLint *v );
GLAPI void GLAPIENTRY glVertex3sv( const GLshort *v );

GLAPI void GLAPIENTRY glVertex4dv( const GLdouble *v );
GLAPI void GLAPIENTRY glVertex4fv( const GLfloat *v );
GLAPI void GLAPIENTRY glVertex4iv( const GLint *v );
GLAPI void GLAPIENTRY glVertex4sv( const GLshort *v );


GLAPI void GLAPIENTRY glNormal3b( GLbyte nx, GLbyte ny, GLbyte nz );
GLAPI void GLAPIENTRY glNormal3d( GLdouble nx, GLdouble ny, GLdouble nz );
GLAPI void GLAPIENTRY glNormal3f( GLfloat nx, GLfloat ny, GLfloat nz );
GLAPI void GLAPIENTRY glNormal3i( GLint nx, GLint ny, GLint nz );
GLAPI void GLAPIENTRY glNormal3s( GLshort nx, GLshort ny, GLshort nz );

GLAPI void GLAPIENTRY glNormal3bv( const GLbyte *v );
GLAPI void GLAPIENTRY glNormal3dv( const GLdouble *v );
GLAPI void GLAPIENTRY glNormal3fv( const GLfloat *v );
GLAPI void GLAPIENTRY glNormal3iv( const GLint *v );
GLAPI void GLAPIENTRY glNormal3sv( const GLshort *v );


GLAPI void GLAPIENTRY glIndexd( GLdouble c );
GLAPI void GLAPIENTRY glIndexf( GLfloat c );
GLAPI void GLAPIENTRY glIndexi( GLint c );
GLAPI void GLAPIENTRY glIndexs( GLshort c );
GLAPI void GLAPIENTRY glIndexub( GLubyte c );  /* 1.1 */

GLAPI void GLAPIENTRY glIndexdv( const GLdouble *c );
GLAPI void GLAPIENTRY glIndexfv( const GLfloat *c );
GLAPI void GLAPIENTRY glIndexiv( const GLint *c );
GLAPI void GLAPIENTRY glIndexsv( const GLshort *c );
GLAPI void GLAPIENTRY glIndexubv( const GLubyte *c );  /* 1.1 */

GLAPI void GLAPIENTRY glColor3b( GLbyte red, GLbyte green, GLbyte blue );
GLAPI void GLAPIENTRY glColor3d( GLdouble red, GLdouble green, GLdouble blue );
GLAPI void GLAPIENTRY glColor3f( GLfloat red, GLfloat green, GLfloat blue );
GLAPI void GLAPIENTRY glColor3i( GLint red, GLint green, GLint blue );
GLAPI void GLAPIENTRY glColor3s( GLshort red, GLshort green, GLshort blue );
GLAPI void GLAPIENTRY glColor3ub( GLubyte red, GLubyte green, GLubyte blue );
GLAPI void GLAPIENTRY glColor3ui( GLuint red, GLuint green, GLuint blue );
GLAPI void GLAPIENTRY glColor3us( GLushort red, GLushort green, GLushort blue );

GLAPI void GLAPIENTRY glColor4b( GLbyte red, GLbyte green,
                                   GLbyte blue, GLbyte alpha );
GLAPI void GLAPIENTRY glColor4d( GLdouble red, GLdouble green,
                                   GLdouble blue, GLdouble alpha );
GLAPI void GLAPIENTRY glColor4f( GLfloat red, GLfloat green,
                                   GLfloat blue, GLfloat alpha );
GLAPI void GLAPIENTRY glColor4i( GLint red, GLint green,
                                   GLint blue, GLint alpha );
GLAPI void GLAPIENTRY glColor4s( GLshort red, GLshort green,
                                   GLshort blue, GLshort alpha );
GLAPI void GLAPIENTRY glColor4ub( GLubyte red, GLubyte green,
                                    GLubyte blue, GLubyte alpha );
GLAPI void GLAPIENTRY glColor4ui( GLuint red, GLuint green,
                                    GLuint blue, GLuint alpha );
GLAPI void GLAPIENTRY glColor4us( GLushort red, GLushort green,
                                    GLushort blue, GLushort alpha );


GLAPI void GLAPIENTRY glColor3bv( const GLbyte *v );
GLAPI void GLAPIENTRY glColor3dv( const GLdouble *v );
GLAPI void GLAPIENTRY glColor3fv( const GLfloat *v );
GLAPI void GLAPIENTRY glColor3iv( const GLint *v );
GLAPI void GLAPIENTRY glColor3sv( const GLshort *v );
GLAPI void GLAPIENTRY glColor3ubv( const GLubyte *v );
GLAPI void GLAPIENTRY glColor3uiv( const GLuint *v );
GLAPI void GLAPIENTRY glColor3usv( const GLushort *v );

GLAPI void GLAPIENTRY glColor4bv( const GLbyte *v );
GLAPI void GLAPIENTRY glColor4dv( const GLdouble *v );
GLAPI void GLAPIENTRY glColor4fv( const GLfloat *v );
GLAPI void GLAPIENTRY glColor4iv( const GLint *v );
GLAPI void GLAPIENTRY glColor4sv( const GLshort *v );
GLAPI void GLAPIENTRY glColor4ubv( const GLubyte *v );
GLAPI void GLAPIENTRY glColor4uiv( const GLuint *v );
GLAPI void GLAPIENTRY glColor4usv( const GLushort *v );


GLAPI void GLAPIENTRY glTexCoord1d( GLdouble s );
GLAPI void GLAPIENTRY glTexCoord1f( GLfloat s );
GLAPI void GLAPIENTRY glTexCoord1i( GLint s );
GLAPI void GLAPIENTRY glTexCoord1s( GLshort s );

GLAPI void GLAPIENTRY glTexCoord2d( GLdouble s, GLdouble t );
GLAPI void GLAPIENTRY glTexCoord2f( GLfloat s, GLfloat t );
GLAPI void GLAPIENTRY glTexCoord2i( GLint s, GLint t );
GLAPI void GLAPIENTRY glTexCoord2s( GLshort s, GLshort t );

GLAPI void GLAPIENTRY glTexCoord3d( GLdouble s, GLdouble t, GLdouble r );
GLAPI void GLAPIENTRY glTexCoord3f( GLfloat s, GLfloat t, GLfloat r );
GLAPI void GLAPIENTRY glTexCoord3i( GLint s, GLint t, GLint r );
GLAPI void GLAPIENTRY glTexCoord3s( GLshort s, GLshort t, GLshort r );

GLAPI void GLAPIENTRY glTexCoord4d( GLdouble s, GLdouble t, GLdouble r, GLdouble q );
GLAPI void GLAPIENTRY glTexCoord4f( GLfloat s, GLfloat t, GLfloat r, GLfloat q );
GLAPI void GLAPIENTRY glTexCoord4i( GLint s, GLint t, GLint r, GLint q );
GLAPI void GLAPIENTRY glTexCoord4s( GLshort s, GLshort t, GLshort r, GLshort q );

GLAPI void GLAPIENTRY glTexCoord1dv( const GLdouble *v );
GLAPI void GLAPIENTRY glTexCoord1fv( const GLfloat *v );
GLAPI void GLAPIENTRY glTexCoord1iv( const GLint *v );
GLAPI void GLAPIENTRY glTexCoord1sv( const GLshort *v );

GLAPI void GLAPIENTRY glTexCoord2dv( const GLdouble *v );
GLAPI void GLAPIENTRY glTexCoord2fv( const GLfloat *v );
GLAPI void GLAPIENTRY glTexCoord2iv( const GLint *v );
GLAPI void GLAPIENTRY glTexCoord2sv( const GLshort *v );

GLAPI void GLAPIENTRY glTexCoord3dv( const GLdouble *v );
GLAPI void GLAPIENTRY glTexCoord3fv( const GLfloat *v );
GLAPI void GLAPIENTRY glTexCoord3iv( const GLint *v );
GLAPI void GLAPIENTRY glTexCoord3sv( const GLshort *v );

GLAPI void GLAPIENTRY glTexCoord4dv( const GLdouble *v );
GLAPI void GLAPIENTRY glTexCoord4fv( const GLfloat *v );
GLAPI void GLAPIENTRY glTexCoord4iv( const GLint *v );
GLAPI void GLAPIENTRY glTexCoord4sv( const GLshort *v );


GLAPI void GLAPIENTRY glRasterPos2d( GLdouble x, GLdouble y );
GLAPI void GLAPIENTRY glRasterPos2f( GLfloat x, GLfloat y );
GLAPI void GLAPIENTRY glRasterPos2i( GLint x, GLint y );
GLAPI void GLAPIENTRY glRasterPos2s( GLshort x, GLshort y );

GLAPI void GLAPIENTRY glRasterPos3d( GLdouble x, GLdouble y, GLdouble z );
GLAPI void GLAPIENTRY glRasterPos3f( GLfloat x, GLfloat y, GLfloat z );
GLAPI void GLAPIENTRY glRasterPos3i( GLint x, GLint y, GLint z );
GLAPI void GLAPIENTRY glRasterPos3s( GLshort x, GLshort y, GLshort z );

GLAPI void GLAPIENTRY glRasterPos4d( GLdouble x, GLdouble y, GLdouble z, GLdouble w );
GLAPI void GLAPIENTRY glRasterPos4f( GLfloat x, GLfloat y, GLfloat z, GLfloat w );
GLAPI void GLAPIENTRY glRasterPos4i( GLint x, GLint y, GLint z, GLint w );
GLAPI void GLAPIENTRY glRasterPos4s( GLshort x, GLshort y, GLshort z, GLshort w );

GLAPI void GLAPIENTRY glRasterPos2dv( const GLdouble *v );
GLAPI void GLAPIENTRY glRasterPos2fv( const GLfloat *v );
GLAPI void GLAPIENTRY glRasterPos2iv( const GLint *v );
GLAPI void GLAPIENTRY glRasterPos2sv( const GLshort *v );

GLAPI void GLAPIENTRY glRasterPos3dv( const GLdouble *v );
GLAPI void GLAPIENTRY glRasterPos3fv( const GLfloat *v );
GLAPI void GLAPIENTRY glRasterPos3iv( const GLint *v );
GLAPI void GLAPIENTRY glRasterPos3sv( const GLshort *v );

GLAPI void GLAPIENTRY glRasterPos4dv( const GLdouble *v );
GLAPI void GLAPIENTRY glRasterPos4fv( const GLfloat *v );
GLAPI void GLAPIENTRY glRasterPos4iv( const GLint *v );
GLAPI void GLAPIENTRY glRasterPos4sv( const GLshort *v );


GLAPI void GLAPIENTRY glRectd( GLdouble x1, GLdouble y1, GLdouble x2, GLdouble y2 );
GLAPI void GLAPIENTRY glRectf( GLfloat x1, GLfloat y1, GLfloat x2, GLfloat y2 );
GLAPI void GLAPIENTRY glRecti( GLint x1, GLint y1, GLint x2, GLint y2 );
GLAPI void GLAPIENTRY glRects( GLshort x1, GLshort y1, GLshort x2, GLshort y2 );


GLAPI void GLAPIENTRY glRectdv( const GLdouble *v1, const GLdouble *v2 );
GLAPI void GLAPIENTRY glRectfv( const GLfloat *v1, const GLfloat *v2 );
GLAPI void GLAPIENTRY glRectiv( const GLint *v1, const GLint *v2 );
GLAPI void GLAPIENTRY glRectsv( const GLshort *v1, const GLshort *v2 );


/*
 * Vertex Arrays  (1.1)
 */

GLAPI void GLAPIENTRY glVertexPointer( GLint size, GLenum type,
                                       GLsizei stride, const GLvoid *ptr );

GLAPI void GLAPIENTRY glNormalPointer( GLenum type, GLsizei stride,
                                       const GLvoid *ptr );

GLAPI void GLAPIENTRY glColorPointer( GLint size, GLenum type,
                                      GLsizei stride, const GLvoid *ptr );

GLAPI void GLAPIENTRY glIndexPointer( GLenum type, GLsizei stride,
                                      const GLvoid *ptr );

GLAPI void GLAPIENTRY glTexCoordPointer( GLint size, GLenum type,
                                         GLsizei stride, const GLvoid *ptr );

GLAPI void GLAPIENTRY glEdgeFlagPointer( GLsizei stride, const GLvoid *ptr );

GLAPI void GLAPIENTRY glGetPointerv( GLenum pname, GLvoid **params );

GLAPI void GLAPIENTRY glArrayElement( GLint i );

GLAPI void GLAPIENTRY glDrawArrays( GLenum mode, GLint first, GLsizei count );

GLAPI void GLAPIENTRY glDrawElements( GLenum mode, GLsizei count,
                                      GLenum type, const GLvoid *indices );

GLAPI void GLAPIENTRY glInterleavedArrays( GLenum format, GLsizei stride,
                                           const GLvoid *pointer );

/*
 * Lighting
 */

GLAPI void GLAPIENTRY glShadeModel( GLenum mode );

GLAPI void GLAPIENTRY glLightf( GLenum light, GLenum pname, GLfloat param );
GLAPI void GLAPIENTRY glLighti( GLenum light, GLenum pname, GLint param );
GLAPI void GLAPIENTRY glLightfv( GLenum light, GLenum pname,
                                 const GLfloat *params );
GLAPI void GLAPIENTRY glLightiv( GLenum light, GLenum pname,
                                 const GLint *params );

GLAPI void GLAPIENTRY glGetLightfv( GLenum light, GLenum pname,
                                    GLfloat *params );
GLAPI void GLAPIENTRY glGetLightiv( GLenum light, GLenum pname,
                                    GLint *params );

GLAPI void GLAPIENTRY glLightModelf( GLenum pname, GLfloat param );
GLAPI void GLAPIENTRY glLightModeli( GLenum pname, GLint param );
GLAPI void GLAPIENTRY glLightModelfv( GLenum pname, const GLfloat *params );
GLAPI void GLAPIENTRY glLightModeliv( GLenum pname, const GLint *params );

GLAPI void GLAPIENTRY glMaterialf( GLenum face, GLenum pname, GLfloat param );
GLAPI void GLAPIENTRY glMateriali( GLenum face, GLenum pname, GLint param );
GLAPI void GLAPIENTRY glMaterialfv( GLenum face, GLenum pname, const GLfloat *params );
GLAPI void GLAPIENTRY glMaterialiv( GLenum face, GLenum pname, const GLint *params );

GLAPI void GLAPIENTRY glGetMaterialfv( GLenum face, GLenum pname, GLfloat *params );
GLAPI void GLAPIENTRY glGetMaterialiv( GLenum face, GLenum pname, GLint *params );

GLAPI void GLAPIENTRY glColorMaterial( GLenum face, GLenum mode );


/*
 * Raster functions
 */

GLAPI void GLAPIENTRY glPixelZoom( GLfloat xfactor, GLfloat yfactor );

GLAPI void GLAPIENTRY glPixelStoref( GLenum pname, GLfloat param );
GLAPI void GLAPIENTRY glPixelStorei( GLenum pname, GLint param );

GLAPI void GLAPIENTRY glPixelTransferf( GLenum pname, GLfloat param );
GLAPI void GLAPIENTRY glPixelTransferi( GLenum pname, GLint param );

GLAPI void GLAPIENTRY glPixelMapfv( GLenum map, GLsizei mapsize,
                                    const GLfloat *values );
GLAPI void GLAPIENTRY glPixelMapuiv( GLenum map, GLsizei mapsize,
                                     const GLuint *values );
GLAPI void GLAPIENTRY glPixelMapusv( GLenum map, GLsizei mapsize,
                                     const GLushort *values );

GLAPI void GLAPIENTRY glGetPixelMapfv( GLenum map, GLfloat *values );
GLAPI void GLAPIENTRY glGetPixelMapuiv( GLenum map, GLuint *values );
GLAPI void GLAPIENTRY glGetPixelMapusv( GLenum map, GLushort *values );

GLAPI void GLAPIENTRY glBitmap( GLsizei width, GLsizei height,
                                GLfloat xorig, GLfloat yorig,
                                GLfloat xmove, GLfloat ymove,
                                const GLubyte *bitmap );

GLAPI void GLAPIENTRY glReadPixels( GLint x, GLint y,
                                    GLsizei width, GLsizei height,
                                    GLenum format, GLenum type,
                                    GLvoid *pixels );

GLAPI void GLAPIENTRY glDrawPixels( GLsizei width, GLsizei height,
                                    GLenum format, GLenum type,
                                    const GLvoid *pixels );

GLAPI void GLAPIENTRY glCopyPixels( GLint x, GLint y,
                                    GLsizei width, GLsizei height,
                                    GLenum type );

/*
 * Stenciling
 */

GLAPI void GLAPIENTRY glStencilFunc( GLenum func, GLint ref, GLuint mask );

GLAPI void GLAPIENTRY glStencilMask( GLuint mask );

GLAPI void GLAPIENTRY glStencilOp( GLenum fail, GLenum zfail, GLenum zpass );

GLAPI void GLAPIENTRY glClearStencil( GLint s );



/*
 * Texture mapping
 */

GLAPI void GLAPIENTRY glTexGend( GLenum coord, GLenum pname, GLdouble param );
GLAPI void GLAPIENTRY glTexGenf( GLenum coord, GLenum pname, GLfloat param );
GLAPI void GLAPIENTRY glTexGeni( GLenum coord, GLenum pname, GLint param );

GLAPI void GLAPIENTRY glTexGendv( GLenum coord, GLenum pname, const GLdouble *params );
GLAPI void GLAPIENTRY glTexGenfv( GLenum coord, GLenum pname, const GLfloat *params );
GLAPI void GLAPIENTRY glTexGeniv( GLenum coord, GLenum pname, const GLint *params );

GLAPI void GLAPIENTRY glGetTexGendv( GLenum coord, GLenum pname, GLdouble *params );
GLAPI void GLAPIENTRY glGetTexGenfv( GLenum coord, GLenum pname, GLfloat *params );
GLAPI void GLAPIENTRY glGetTexGeniv( GLenum coord, GLenum pname, GLint *params );


GLAPI void GLAPIENTRY glTexEnvf( GLenum target, GLenum pname, GLfloat param );
GLAPI void GLAPIENTRY glTexEnvi( GLenum target, GLenum pname, GLint param );

GLAPI void GLAPIENTRY glTexEnvfv( GLenum target, GLenum pname, const GLfloat *params );
GLAPI void GLAPIENTRY glTexEnviv( GLenum target, GLenum pname, const GLint *params );

GLAPI void GLAPIENTRY glGetTexEnvfv( GLenum target, GLenum pname, GLfloat *params );
GLAPI void GLAPIENTRY glGetTexEnviv( GLenum target, GLenum pname, GLint *params );


GLAPI void GLAPIENTRY glTexParameterf( GLenum target, GLenum pname, GLfloat param );
GLAPI void GLAPIENTRY glTexParameteri( GLenum target, GLenum pname, GLint param );

GLAPI void GLAPIENTRY glTexParameterfv( GLenum target, GLenum pname,
                                          const GLfloat *params );
GLAPI void GLAPIENTRY glTexParameteriv( GLenum target, GLenum pname,
                                          const GLint *params );

GLAPI void GLAPIENTRY glGetTexParameterfv( GLenum target,
                                           GLenum pname, GLfloat *params);
GLAPI void GLAPIENTRY glGetTexParameteriv( GLenum target,
                                           GLenum pname, GLint *params );

GLAPI void GLAPIENTRY glGetTexLevelParameterfv( GLenum target, GLint level,
                                                GLenum pname, GLfloat *params );
GLAPI void GLAPIENTRY glGetTexLevelParameteriv( GLenum target, GLint level,
                                                GLenum pname, GLint *params );


GLAPI void GLAPIENTRY glTexImage1D( GLenum target, GLint level,
                                    GLint internalFormat,
                                    GLsizei width, GLint border,
                                    GLenum format, GLenum type,
                                    const GLvoid *pixels );

GLAPI void GLAPIENTRY glTexImage2D( GLenum target, GLint level,
                                    GLint internalFormat,
                                    GLsizei width, GLsizei height,
                                    GLint border, GLenum format, GLenum type,
                                    const GLvoid *pixels );

GLAPI void GLAPIENTRY glGetTexImage( GLenum target, GLint level,
                                     GLenum format, GLenum type,
                                     GLvoid *pixels );


/* 1.1 functions */

GLAPI void GLAPIENTRY glGenTextures( GLsizei n, GLuint *textures );

GLAPI void GLAPIENTRY glDeleteTextures( GLsizei n, const GLuint *textures);

GLAPI void GLAPIENTRY glBindTexture( GLenum target, GLuint texture );

GLAPI void GLAPIENTRY glPrioritizeTextures( GLsizei n,
                                            const GLuint *textures,
                                            const GLclampf *priorities );

GLAPI GLboolean GLAPIENTRY glAreTexturesResident( GLsizei n,
                                                  const GLuint *textures,
                                                  GLboolean *residences );

GLAPI GLboolean GLAPIENTRY glIsTexture( GLuint texture );


GLAPI void GLAPIENTRY glTexSubImage1D( GLenum target, GLint level,
                                       GLint xoffset,
                                       GLsizei width, GLenum format,
                                       GLenum type, const GLvoid *pixels );


GLAPI void GLAPIENTRY glTexSubImage2D( GLenum target, GLint level,
                                       GLint xoffset, GLint yoffset,
                                       GLsizei width, GLsizei height,
                                       GLenum format, GLenum type,
                                       const GLvoid *pixels );


GLAPI void GLAPIENTRY glCopyTexImage1D( GLenum target, GLint level,
                                        GLenum internalformat,
                                        GLint x, GLint y,
                                        GLsizei width, GLint border );


GLAPI void GLAPIENTRY glCopyTexImage2D( GLenum target, GLint level,
                                        GLenum internalformat,
                                        GLint x, GLint y,
                                        GLsizei width, GLsizei height,
                                        GLint border );


GLAPI void GLAPIENTRY glCopyTexSubImage1D( GLenum target, GLint level,
                                           GLint xoffset, GLint x, GLint y,
                                           GLsizei width );


GLAPI void GLAPIENTRY glCopyTexSubImage2D( GLenum target, GLint level,
                                           GLint xoffset, GLint yoffset,
                                           GLint x, GLint y,
                                           GLsizei width, GLsizei height );


/*
 * Evaluators
 */

GLAPI void GLAPIENTRY glMap1d( GLenum target, GLdouble u1, GLdouble u2,
                               GLint stride,
                               GLint order, const GLdouble *points );
GLAPI void GLAPIENTRY glMap1f( GLenum target, GLfloat u1, GLfloat u2,
                               GLint stride,
                               GLint order, const GLfloat *points );

GLAPI void GLAPIENTRY glMap2d( GLenum target,
		     GLdouble u1, GLdouble u2, GLint ustride, GLint uorder,
		     GLdouble v1, GLdouble v2, GLint vstride, GLint vorder,
		     const GLdouble *points );
GLAPI void GLAPIENTRY glMap2f( GLenum target,
		     GLfloat u1, GLfloat u2, GLint ustride, GLint uorder,
		     GLfloat v1, GLfloat v2, GLint vstride, GLint vorder,
		     const GLfloat *points );

GLAPI void GLAPIENTRY glGetMapdv( GLenum target, GLenum query, GLdouble *v );
GLAPI void GLAPIENTRY glGetMapfv( GLenum target, GLenum query, GLfloat *v );
GLAPI void GLAPIENTRY glGetMapiv( GLenum target, GLenum query, GLint *v );

GLAPI void GLAPIENTRY glEvalCoord1d( GLdouble u );
GLAPI void GLAPIENTRY glEvalCoord1f( GLfloat u );

GLAPI void GLAPIENTRY glEvalCoord1dv( const GLdouble *u );
GLAPI void GLAPIENTRY glEvalCoord1fv( const GLfloat *u );

GLAPI void GLAPIENTRY glEvalCoord2d( GLdouble u, GLdouble v );
GLAPI void GLAPIENTRY glEvalCoord2f( GLfloat u, GLfloat v );

GLAPI void GLAPIENTRY glEvalCoord2dv( const GLdouble *u );
GLAPI void GLAPIENTRY glEvalCoord2fv( const GLfloat *u );

GLAPI void GLAPIENTRY glMapGrid1d( GLint un, GLdouble u1, GLdouble u2 );
GLAPI void GLAPIENTRY glMapGrid1f( GLint un, GLfloat u1, GLfloat u2 );

GLAPI void GLAPIENTRY glMapGrid2d( GLint un, GLdouble u1, GLdouble u2,
                                   GLint vn, GLdouble v1, GLdouble v2 );
GLAPI void GLAPIENTRY glMapGrid2f( GLint un, GLfloat u1, GLfloat u2,
                                   GLint vn, GLfloat v1, GLfloat v2 );

GLAPI void GLAPIENTRY glEvalPoint1( GLint i );

GLAPI void GLAPIENTRY glEvalPoint2( GLint i, GLint j );

GLAPI void GLAPIENTRY glEvalMesh1( GLenum mode, GLint i1, GLint i2 );

GLAPI void GLAPIENTRY glEvalMesh2( GLenum mode, GLint i1, GLint i2, GLint j1, GLint j2 );


/*
 * Fog
 */

GLAPI void GLAPIENTRY glFogf( GLenum pname, GLfloat param );

GLAPI void GLAPIENTRY glFogi( GLenum pname, GLint param );

GLAPI void GLAPIENTRY glFogfv( GLenum pname, const GLfloat *params );

GLAPI void GLAPIENTRY glFogiv( GLenum pname, const GLint *params );


/*
 * Selection and Feedback
 */

GLAPI void GLAPIENTRY glFeedbackBuffer( GLsizei size, GLenum type, GLfloat *buffer );

GLAPI void GLAPIENTRY glPassThrough( GLfloat token );

GLAPI void GLAPIENTRY glSelectBuffer( GLsizei size, GLuint *buffer );

GLAPI void GLAPIENTRY glInitNames( void );

GLAPI void GLAPIENTRY glLoadName( GLuint name );

GLAPI void GLAPIENTRY glPushName( GLuint name );

GLAPI void GLAPIENTRY glPopName( void );
}