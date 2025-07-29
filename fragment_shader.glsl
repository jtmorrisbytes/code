#version 300 es
precision mediump float;
// in vec3 fragPos;
out vec4 fragColor;

void main()
{
            // float x_color = clamp(abs(fragPos.x),0.25,1.0);
            fragColor = vec4(1.0, 1.0, 1.0, 1.0); // Red color
}