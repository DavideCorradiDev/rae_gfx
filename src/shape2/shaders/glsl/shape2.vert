#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec2 inPosition;
layout(location = 0) out vec4 fragColor;
layout(push_constant) uniform PushConstant {
    mat4 transform;
    vec4 color;
} pushConstant;

void main() {
    gl_Position = pushConstant.transform * vec4(inPosition.x, inPosition.y, 0., 1.);
    fragColor = pushConstant.color;
}