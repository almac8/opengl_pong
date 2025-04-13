#version 330 core

layout(location = 0) in vec2 pos;
layout(location = 1) in vec2 texturePos;

out vec2 textureCoords;

void main() {
  gl_Position = vec4(pos, 0.0, 1.0);
  textureCoords = texturePos;
}