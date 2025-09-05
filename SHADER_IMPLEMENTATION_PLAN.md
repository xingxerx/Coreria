# Epoch of Elria - Advanced Shader Implementation Plan

## Current Status: Kiss3D Limitations

Kiss3D is a simple 3D graphics engine designed for ease of use, but it has significant limitations for advanced rendering:

### Kiss3D Constraints:
- **Limited Shader Support**: No built-in custom shader pipeline
- **Basic Lighting**: Simple fixed-function lighting only
- **No Post-Processing**: No framebuffer effects or multi-pass rendering
- **Minimal Material System**: Basic color/texture materials only
- **No Compute Shaders**: No advanced GPU compute capabilities

## Recommended Engine Migration Path

To implement the full Cel/Neon/Spiral shader stack, we need to migrate to a more capable engine:

### Option 1: Bevy Engine (Recommended)
**Pros:**
- Full custom shader support (WGSL)
- Advanced post-processing pipeline
- ECS architecture for complex systems
- Active development and community
- Built-in bloom, HDR, and custom materials

**Implementation Timeline:** 2-3 weeks
**Complexity:** Medium-High

### Option 2: Custom OpenGL/wgpu Implementation
**Pros:**
- Complete control over rendering pipeline
- Direct implementation of provided GLSL shaders
- Maximum performance optimization
- Exact match to shader specifications

**Implementation Timeline:** 4-6 weeks
**Complexity:** High

### Option 3: Luminance-rs Framework
**Pros:**
- Rust-native graphics framework
- Direct OpenGL control
- Shader-focused design
- Lower-level than Bevy but more structured than raw OpenGL

**Implementation Timeline:** 3-4 weeks
**Complexity:** Medium-High

## Immediate Enhancements (Current Kiss3D)

While planning the migration, we can implement these improvements:

### 1. Enhanced Color Palette System ✅ COMPLETED
- Deep blue, neon orange, neon blue color scheme
- Dynamic day/night atmospheric lighting
- Color interpolation based on time cycles

### 2. Pseudo-Cel Shading Effects
- Quantized lighting calculations
- Stepped color transitions
- Enhanced contrast ratios

### 3. Atmospheric Post-Processing Simulation
- Background color manipulation for atmosphere
- Dynamic fog/ambient color changes
- Time-based color cycling

## Full Shader Stack Implementation (Post-Migration)

### Phase 1: Core Rendering Pipeline
1. **G-Buffer Setup**
   - Color (RGBA16F)
   - Depth (DEPTH24)
   - Normals (RGB16F)

2. **Cel-Shading Pass**
   - Quantized diffuse lighting
   - Stepped specular highlights
   - Palette-based color grading

### Phase 2: Edge Detection & Outline
1. **Sobel Edge Detection**
   - Depth-based edge detection
   - Normal-based edge detection
   - Configurable edge thickness

2. **Ink Overlay**
   - Black outline rendering
   - Distance-based line width
   - Artistic edge styling

### Phase 3: Advanced Effects
1. **Spiral Warp Effect**
   - UV coordinate distortion
   - Time-based animation
   - Center-point configuration

2. **Bloom Pipeline**
   - Bright pass extraction
   - Gaussian blur (separable)
   - Neon-biased bloom intensity

### Phase 4: Final Composition
1. **Color Grading**
   - Lift/Gamma/Gain adjustments
   - Deep blue bias application
   - Neon accent enhancement

2. **Atmospheric Integration**
   - Day/night cycle integration
   - Dynamic parameter adjustment
   - Smooth transitions

## Migration Recommendation

**Immediate Action:** Begin Bevy Engine migration
- Port current game logic to Bevy ECS
- Implement basic 3D scene in Bevy
- Add custom shader materials
- Implement post-processing pipeline

**Timeline:**
- Week 1: Basic Bevy setup and scene porting
- Week 2: Custom shader implementation
- Week 3: Post-processing and effects
- Week 4: Polish and optimization

## Current Implementation Status

✅ **Completed:**
- Day/Night cycle system (15-minute cycles)
- Real-time clock with progress visualization
- ASCII minimap system
- Epoch of Elria color palette
- Atmospheric lighting simulation
- Enhanced UI and feedback systems

🔄 **In Progress:**
- Advanced shader research and planning
- Engine migration preparation

⏳ **Planned:**
- Bevy engine migration
- Full shader stack implementation
- Advanced visual effects
- Performance optimization

## Next Steps

1. **Prepare Migration Branch**
   - Create new Bevy-based project structure
   - Port existing game logic
   - Maintain feature parity

2. **Implement Shader Pipeline**
   - Convert GLSL shaders to WGSL
   - Set up multi-pass rendering
   - Add post-processing effects

3. **Polish and Optimize**
   - Fine-tune visual parameters
   - Optimize performance
   - Add advanced features

The current Kiss3D implementation provides an excellent foundation with working day/night cycles, atmospheric lighting, and the core game mechanics. The next phase will unlock the full visual potential of the Epoch of Elria aesthetic.
