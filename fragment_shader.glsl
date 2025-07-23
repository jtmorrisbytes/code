#version 300 es // Specify the GLSL version
precision mediump float;
in vec4 gl_FragCoord;

out vec4 fragColor; // Declare the output variable for the fragment color

void main()
{
    fragColor = vec4(gl_FragCoord.x, gl_FragCoord.y, gl_FragCoord.z, gl_FragCoord.w); // Set the fragment color to red (RGBA)
}