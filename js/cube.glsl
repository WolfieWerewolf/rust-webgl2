//VERTEX
#version 300 es
layout(location = 0) in vec3 aPosition;
layout(location = 1) in vec3 aColor;
uniform mat4 uMVMatrix;
uniform mat4 uPMatrix;
out vec4 vColor;
void main() {
    gl_Position = uPMatrix * uMVMatrix * vec4(aPosition, 1.0);
    vColor = vec4(aColor, 1.0);
}

//FRAGMENT
#version 300 es
precision mediump float;
in vec4 vColor;
out vec4 oFragColor;
void main() {
    oFragColor = vColor;
}