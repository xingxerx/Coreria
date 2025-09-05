#version 330 core
out vec4 FragColor;
in vec2 TexCoord;

uniform sampler2D uDepth;
uniform sampler2D uNormals;
uniform vec2 uPixel;
uniform float uEdgeScale; // line width scaling
uniform float uTimeOfDay; // for dynamic edge styling
uniform vec3 uEdgeColor;  // edge line color

void main() {
    // Sobel edge detection on depth
    float d00 = texture(uDepth, TexCoord + vec2(-uPixel.x, -uPixel.y)).r;
    float d10 = texture(uDepth, TexCoord + vec2( uPixel.x, -uPixel.y)).r;
    float d01 = texture(uDepth, TexCoord + vec2(-uPixel.x,  uPixel.y)).r;
    float d11 = texture(uDepth, TexCoord + vec2( uPixel.x,  uPixel.y)).r;

    float gx = d10 - d00;
    float gy = d11 - d01;
    float depthEdge = sqrt(gx * gx + gy * gy);

    // Sobel edge detection on normals
    vec3 n00 = texture(uNormals, TexCoord + vec2(-uPixel.x, -uPixel.y)).rgb;
    vec3 n10 = texture(uNormals, TexCoord + vec2( uPixel.x, -uPixel.y)).rgb;
    vec3 n01 = texture(uNormals, TexCoord + vec2(-uPixel.x,  uPixel.y)).rgb;
    vec3 n11 = texture(uNormals, TexCoord + vec2( uPixel.x,  uPixel.y)).rgb;

    vec3 ngx = n10 - n00;
    vec3 ngy = n11 - n01;
    float normalEdge = length(ngx) + length(ngy);

    // Combine edge detection methods
    float edge = max(depthEdge, normalEdge * 0.5);
    
    // Dynamic edge threshold based on time of day
    float threshold = mix(0.015, 0.025, uTimeOfDay); // Thinner lines at night
    float edgeStrength = smoothstep(threshold, threshold + 0.01, edge);

    // Dynamic edge color based on time of day
    vec3 finalEdgeColor = mix(
        vec3(0.0, 0.0, 0.0),        // Black edges during day
        uEdgeColor,                  // Colored edges at night
        1.0 - uTimeOfDay
    );

    // Scale edge width
    edgeStrength *= uEdgeScale;

    FragColor = vec4(finalEdgeColor, edgeStrength);
}
