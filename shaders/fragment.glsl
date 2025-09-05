#version 330 core
in vec3 vNormal;
in vec3 vPos;

out vec4 FragColor;

uniform vec3 uLightDir;   // normalized
uniform vec3 uBaseColor;  // e.g., vec3(0.18, 0.25, 0.33) deep blue
uniform vec3 uAccent1;    // vec3(1.0, 0.70, 0.07) neon orange
uniform vec3 uAccent2;    // vec3(0.0, 0.75, 1.0) neon blue

// tweakables
uniform float uStepCount; // 2–4 tonal steps
uniform float uBlueBias;  // 0–1 pushes hue
uniform float uGlow;      // bloom threshold

// Epoch of Elria atmospheric parameters
uniform float uTimeOfDay; // 0.0 = midnight, 0.5 = noon, 1.0 = midnight
uniform float uWeatherIntensity; // 0.0 = clear, 1.0 = storm
uniform vec3 uFogColor;   // atmospheric fog color
uniform float uFogDensity; // fog density

void main() {
    // lighting
    vec3 N = normalize(vNormal);
    float ndl = max(dot(N, normalize(uLightDir)), 0.0);

    // quantize to steps for cel-shading
    float stepped = floor(ndl * uStepCount) / (uStepCount - 1.0);

    // base cel color with atmospheric influence
    vec3 baseColor = mix(uBaseColor, uFogColor, uFogDensity * 0.3);
    vec3 color = mix(baseColor, uAccent2, stepped + uBlueBias);

    // time of day influence
    float dayIntensity = sin(uTimeOfDay * 3.14159);
    if (dayIntensity > 0.0) {
        // Day: warmer tones
        color = mix(color, uAccent1, dayIntensity * 0.2);
    } else {
        // Night: cooler, more blue
        color = mix(color, uAccent2, abs(dayIntensity) * 0.4);
    }

    // weather effects
    if (uWeatherIntensity > 0.5) {
        // Storm: darker, more dramatic
        color *= (1.0 - uWeatherIntensity * 0.3);
        color = mix(color, vec3(0.1, 0.15, 0.25), uWeatherIntensity * 0.2);
    }

    // accent: rim glow with atmospheric enhancement
    float rim = 1.0 - max(dot(N, normalize(-vPos)), 0.0);
    if (rim > 0.75) {
        vec3 rimColor = mix(uAccent1, uAccent2, uTimeOfDay);
        color = mix(color, rimColor, rim * uGlow * (1.0 + uWeatherIntensity));
    }

    // atmospheric fog
    float distance = length(vPos);
    float fogFactor = exp(-distance * uFogDensity);
    color = mix(uFogColor, color, fogFactor);

    FragColor = vec4(color, 1.0);
}
