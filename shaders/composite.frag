#version 330 core
out vec4 FragColor;
in vec2 TexCoord;

uniform sampler2D uScene;
uniform sampler2D uBloom;
uniform sampler2D uOutlines;
uniform float uTimeOfDay;
uniform float uWeatherIntensity;
uniform float uExposure;
uniform float uGamma;

// Color grading parameters
uniform float uLift;
uniform float uGamma2;
uniform float uGain;
uniform float uSaturation;

// Epoch of Elria color grading
vec3 colorGrade(vec3 color) {
    // Lift, Gamma, Gain
    color = pow(color + uLift, vec3(1.0 / uGamma2)) * uGain;
    
    // Saturation
    float luminance = dot(color, vec3(0.2126, 0.7152, 0.0722));
    color = mix(vec3(luminance), color, uSaturation);
    
    // Deep blue bias for Epoch of Elria aesthetic
    float blueBias = mix(0.1, 0.3, 1.0 - uTimeOfDay);
    color.b += blueBias * (1.0 - color.b);
    
    // Weather influence on color grading
    if (uWeatherIntensity > 0.5) {
        // Storm: desaturate and darken
        color = mix(color, vec3(dot(color, vec3(0.299, 0.587, 0.114))), uWeatherIntensity * 0.3);
        color *= (1.0 - uWeatherIntensity * 0.2);
    }
    
    return color;
}

// Tone mapping
vec3 toneMap(vec3 color) {
    // Exposure
    color *= uExposure;
    
    // Reinhard tone mapping with slight modification for neon preservation
    vec3 mapped = color / (color + vec3(1.0));
    
    // Preserve neon highlights
    float neonFactor = max(
        smoothstep(0.8, 1.0, color.r * color.g * 0.5), // Orange preservation
        smoothstep(0.8, 1.0, color.g * color.b * 0.5)  // Blue preservation
    );
    mapped = mix(mapped, color * 0.8, neonFactor * 0.3);
    
    // Gamma correction
    mapped = pow(mapped, vec3(1.0 / uGamma));
    
    return mapped;
}

void main() {
    // Sample all render targets
    vec3 scene = texture(uScene, TexCoord).rgb;
    vec3 bloom = texture(uBloom, TexCoord).rgb;
    vec4 outline = texture(uOutlines, TexCoord);
    
    // Combine scene and bloom
    vec3 color = scene + bloom;
    
    // Apply color grading
    color = colorGrade(color);
    
    // Tone mapping
    color = toneMap(color);
    
    // Apply outlines
    color = mix(color, outline.rgb, outline.a);
    
    // Final atmospheric adjustments
    float atmosphericIntensity = mix(0.95, 1.05, sin(uTimeOfDay * 3.14159));
    color *= atmosphericIntensity;
    
    // Subtle vignette effect
    vec2 center = TexCoord - 0.5;
    float vignette = 1.0 - dot(center, center) * 0.3;
    color *= vignette;
    
    FragColor = vec4(color, 1.0);
}
