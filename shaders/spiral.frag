#version 330 core
out vec4 FragColor;
in vec2 TexCoord;

uniform sampler2D uScene;
uniform float uTime;
uniform float uSpiralIntensity; // 0.0 = no effect, 1.0 = full spiral
uniform vec2 uSpiralCenter;     // center point of spiral effect
uniform float uSpiralRadius;    // radius of spiral effect
uniform float uTimeOfDay;       // for time-based spiral variations

void main() {
    vec2 uv = TexCoord;
    
    // Calculate distance from spiral center
    vec2 center = uSpiralCenter;
    vec2 delta = uv - center;
    float distance = length(delta);
    
    // Only apply spiral effect within radius
    if (distance < uSpiralRadius && uSpiralIntensity > 0.0) {
        // Spiral distortion calculation
        float angle = atan(delta.y, delta.x);
        float spiral = uSpiralIntensity * (1.0 - distance / uSpiralRadius);
        
        // Time-based spiral animation
        float timeSpiral = uTime * 0.5 + distance * 8.0;
        
        // Different spiral behavior for day/night
        float spiralSpeed = mix(1.0, 2.0, 1.0 - uTimeOfDay); // Faster at night
        angle += spiral * sin(timeSpiral * spiralSpeed) * 0.3;
        
        // Apply spiral distortion
        float newDistance = distance * (1.0 + spiral * 0.1 * sin(timeSpiral));
        vec2 spiralUV = center + vec2(cos(angle), sin(angle)) * newDistance;
        
        // Blend between original and spiral-distorted UV
        uv = mix(uv, spiralUV, spiral);
    }
    
    // Sample the scene with potentially distorted coordinates
    vec3 color = texture(uScene, uv).rgb;
    
    // Add subtle spiral energy glow
    if (distance < uSpiralRadius * 0.8 && uSpiralIntensity > 0.3) {
        float glow = (1.0 - distance / (uSpiralRadius * 0.8)) * uSpiralIntensity;
        vec3 spiralGlow = mix(
            vec3(0.0, 0.75, 1.0),  // Neon blue
            vec3(1.0, 0.70, 0.07), // Neon orange
            uTimeOfDay
        );
        color += spiralGlow * glow * 0.1 * sin(uTime * 3.0 + distance * 10.0);
    }
    
    FragColor = vec4(color, 1.0);
}
