#version 330 core
out vec4 FragColor;
in vec2 TexCoord;

uniform sampler2D uScene;
uniform vec2 uPixel;
uniform float uBloomThreshold;
uniform float uBloomIntensity;
uniform float uTimeOfDay;
uniform int uPass; // 0 = bright pass, 1 = blur horizontal, 2 = blur vertical

// Gaussian blur weights
const float weights[5] = float[](0.227027, 0.1945946, 0.1216216, 0.054054, 0.016216);

void main() {
    vec3 color = texture(uScene, TexCoord).rgb;
    
    if (uPass == 0) {
        // Bright pass extraction with neon bias
        float brightness = dot(color, vec3(0.2126, 0.7152, 0.0722));
        
        // Bias towards neon colors (orange and blue)
        float neonBias = max(
            dot(color, vec3(1.0, 0.7, 0.0)),  // Orange bias
            dot(color, vec3(0.0, 0.7, 1.0))   // Blue bias
        );
        
        brightness = max(brightness, neonBias * 0.8);
        
        // Dynamic threshold based on time of day
        float threshold = mix(uBloomThreshold * 1.2, uBloomThreshold * 0.8, uTimeOfDay);
        
        if (brightness > threshold) {
            float bloomFactor = (brightness - threshold) / (1.0 - threshold);
            FragColor = vec4(color * bloomFactor, 1.0);
        } else {
            FragColor = vec4(0.0, 0.0, 0.0, 1.0);
        }
    }
    else if (uPass == 1) {
        // Horizontal blur
        vec3 result = texture(uScene, TexCoord).rgb * weights[0];
        for (int i = 1; i < 5; ++i) {
            result += texture(uScene, TexCoord + vec2(uPixel.x * i, 0.0)).rgb * weights[i];
            result += texture(uScene, TexCoord - vec2(uPixel.x * i, 0.0)).rgb * weights[i];
        }
        FragColor = vec4(result, 1.0);
    }
    else if (uPass == 2) {
        // Vertical blur
        vec3 result = texture(uScene, TexCoord).rgb * weights[0];
        for (int i = 1; i < 5; ++i) {
            result += texture(uScene, TexCoord + vec2(0.0, uPixel.y * i)).rgb * weights[i];
            result += texture(uScene, TexCoord - vec2(0.0, uPixel.y * i)).rgb * weights[i];
        }
        
        // Apply time-based bloom intensity
        float intensity = mix(uBloomIntensity * 0.7, uBloomIntensity * 1.3, 1.0 - uTimeOfDay);
        FragColor = vec4(result * intensity, 1.0);
    }
}
