#version 100

//uniform vec2 u_resolution;
//uniform vec3 u_color;
uniform float u_red;
uniform float u_green;
uniform float u_blue;
uniform float u_value;

void main() {
  float n_value = u_value / 127.0;
  //gl_FragColor = vec4(n_value * u_color.r, n_value * u_color.g, n_value * u_color.b, 1.0);
  //gl_FragColor = vec4(vec3(n_value), 1.0);
  gl_FragColor = vec4(u_red, u_green, u_blue, 1.0);
}
