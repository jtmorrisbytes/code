#version 330
layout (location=0) in vec4 aPos;

out vec3 fragPos; 
uniform mat4 model;
uniform mat4 view; // The view matrix
uniform mat4 projection;

uniform vec3 world_space_position;
uniform vec3 local_space_rotation;
uniform vec3 local_space_origin;

mat3 local_space_rotationX_matrix3(float angle) {
    float c = cos(angle);
    float s = sin(angle);
    return mat3(
        1.0,0.0,0.0,
        0.0,  c, -s,
        0.0,  s,  c
    );
}
mat4 local_space_rotationY_matrix4(float angle) {
    float cos = cos(angle);
    float sin = sin(angle);
    return mat4(
        cos,0.0,0.0,0.0,
        0.0,1.0,-sin,0.0,
        -sin,0.0,cos,0.0,
        0.0,0.0,0.0,0.0
    );
}
mat4 local_space_rotationZ_matrix4(float angle) {
    float c = cos(angle);
    float s = sin(angle);
    return mat4(
        1.0,0.0,0.0,0.0,
        0.0,c,-s,0.0,
        0.0,s,c,0.0,
        0.0,0.0,0.0,1.0
    );
}



void main()
{
    // calculate and perform local space transformations
    // normalize the vertex position, assuming the position of the vertex could
    // be other than the 'origin' (0,0,0)
    vec3 vertex_in_local_space = aPos.xyz;    
    /*
      rotate the X coordinate about the axis (0,0,0)
      before applying translations and after applying scaling
    */



    



   // y axis
    vec3 fvy_pos = vertex_in_local_space;
    fvy_pos.x = vertex_in_local_space.x * cos(local_space_rotation.y) + vertex_in_local_space.z * sin(local_space_rotation.y);
    fvy_pos.z = -vertex_in_local_space.x * sin(local_space_rotation.y) + vertex_in_local_space.z * cos(local_space_rotation.y);

    // x axis
    vec3 fvx_pos = fvy_pos;
    fvx_pos.y = fvy_pos.y * cos(local_space_rotation.x) - fvy_pos.z * sin(local_space_rotation.x);
    fvx_pos.z = fvy_pos.y * sin(local_space_rotation.x) + fvy_pos.z * cos(local_space_rotation.x);

    gl_Position = vec4(fvx_pos.x,fvx_pos.y,fvx_pos.z,1.0);
    gl_PointSize = 10.0; // Set point size (e.g., 10 pixels);
    fragPos=gl_Position.xyz;
}