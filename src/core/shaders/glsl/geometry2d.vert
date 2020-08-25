
#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec2 inPosition;
layout(location = 0) out vec4 fragColor;
layout(push_constant) uniform pushConstants {
    mat3 transform;
    vec4 color;
} u_pushConstants;

void main() {
    // vec3 position2d = u_pushConstants.transform * vec3(inPosition, 1.0);
    // gl_Position = vec4(position2d.x, position2d.y, 0.0, 1.0);
    // fragColor = u_pushConstants.color;
    gl_Position = vec4(inPosition.x, inPosition.y, 0., 1.);
    fragColor = vec4(1., 1., 1., 1.);
}