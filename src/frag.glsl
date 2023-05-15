#version 460 compat
out vec4 FragColor;
in vec3 ourColor;

void main(){
   gl_FragColor = vec4(ourColor, 1.0);
}
