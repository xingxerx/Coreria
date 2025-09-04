# Contributing to Epoch of Elria

Thank you for your interest in contributing to Epoch of Elria! This document provides guidelines and information for contributors.

## 🤝 How to Contribute

### Reporting Issues
- Use the GitHub issue tracker
- Provide detailed reproduction steps
- Include system information (OS, Rust version, graphics drivers)
- Attach relevant logs or screenshots

### Suggesting Features
- Open a GitHub issue with the "enhancement" label
- Describe the feature and its benefits
- Consider implementation complexity and engine architecture
- Discuss with maintainers before starting large features

### Code Contributions

#### 1. Fork and Clone
```bash
git clone https://github.com/yourusername/Epoch-of-Elria.git
cd Epoch-of-Elria
```

#### 2. Create a Branch
```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-description
```

#### 3. Development Setup
```bash
# Install dependencies
cargo build

# Run tests
cargo test

# Check code quality
cargo clippy
cargo fmt
```

#### 4. Make Changes
- Follow Rust coding conventions
- Add tests for new functionality
- Update documentation as needed
- Ensure all tests pass

#### 5. Commit and Push
```bash
git add .
git commit -m "feat: add amazing new feature"
git push origin feature/your-feature-name
```

#### 6. Create Pull Request
- Use descriptive title and description
- Reference related issues
- Include screenshots for UI changes
- Ensure CI passes

## 📋 Development Guidelines

### Code Style
- Use `cargo fmt` for formatting
- Follow Rust naming conventions
- Add documentation comments for public APIs
- Use meaningful variable and function names

### Testing
- Write unit tests for new functions
- Add integration tests for major features
- Test memory management functionality
- Verify performance doesn't regress

### Memory Management
- Use the engine's garbage collector for managed objects
- Implement proper cleanup in Drop traits
- Test for memory leaks with the built-in leak detector
- Follow RAII principles

### Performance
- Profile code changes with benchmarks
- Maintain 60+ FPS target
- Use object pools for frequently allocated objects
- Minimize allocations in hot paths

## 🏗️ Architecture Overview

### Core Systems
- **Engine Core** (`src/lib.rs`): Main engine coordination
- **Memory Management** (`src/memory_*.rs`): GC and resource management
- **Rendering** (`src/rendering.rs`): 3D graphics and UI
- **Physics** (`src/physics.rs`): Collision and dynamics
- **Audio** (`src/audio.rs`): Sound and music
- **Game Objects** (`src/game_objects.rs`): Entity management

### Design Principles
- **Memory Safety**: Leverage Rust's ownership system
- **Performance**: 60+ FPS with efficient resource usage
- **Modularity**: Clean separation of concerns
- **Extensibility**: Plugin-friendly architecture

## 🧪 Testing

### Running Tests
```bash
# All tests
cargo test

# Specific test
cargo test test_memory_management

# With output
cargo test -- --nocapture

# Memory tests
cargo test memory --features memory-testing
```

### Test Categories
- **Unit Tests**: Individual function testing
- **Integration Tests**: System interaction testing
- **Memory Tests**: GC and leak detection testing
- **Performance Tests**: Benchmark validation

## 📚 Documentation

### Code Documentation
- Use `///` for public API documentation
- Include examples in doc comments
- Document safety requirements for unsafe code
- Explain complex algorithms and data structures

### User Documentation
- Update README.md for user-facing changes
- Add examples to documentation
- Update build instructions if needed
- Create tutorials for new features

## 🐛 Debugging

### Common Issues
- **Graphics Initialization**: Ensure proper display server setup
- **Memory Leaks**: Use built-in leak detector
- **Performance**: Profile with `cargo bench`
- **Audio**: Check system audio dependencies

### Debug Tools
```bash
# Debug build with logging
RUST_LOG=debug cargo run

# Memory debugging
cargo run --features memory-debug

# Performance profiling
cargo bench
```

## 🔄 Release Process

### Version Numbering
- Follow Semantic Versioning (SemVer)
- Major: Breaking changes
- Minor: New features, backward compatible
- Patch: Bug fixes

### Release Checklist
- [ ] All tests pass
- [ ] Documentation updated
- [ ] Performance benchmarks stable
- [ ] Memory leaks checked
- [ ] Changelog updated
- [ ] Version bumped in Cargo.toml

## 🏷️ Labels and Issues

### Issue Labels
- `bug`: Something isn't working
- `enhancement`: New feature or request
- `documentation`: Improvements or additions to docs
- `good first issue`: Good for newcomers
- `help wanted`: Extra attention is needed
- `memory`: Memory management related
- `performance`: Performance optimization
- `graphics`: Rendering and visual issues

### Priority Labels
- `priority: high`: Critical issues
- `priority: medium`: Important improvements
- `priority: low`: Nice to have features

## 💬 Communication

### Channels
- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General questions and ideas
- **Pull Requests**: Code review and discussion

### Code Review Process
1. Automated CI checks must pass
2. At least one maintainer review required
3. Address feedback and update PR
4. Maintainer merges when approved

## 🎯 Areas for Contribution

### High Priority
- Memory management optimizations
- Graphics performance improvements
- Cross-platform compatibility
- Documentation and examples

### Medium Priority
- New game features
- Audio system enhancements
- UI/UX improvements
- Testing infrastructure

### Good First Issues
- Documentation improvements
- Code cleanup and refactoring
- Simple bug fixes
- Adding examples

## 📄 License

By contributing to Epoch of Elria, you agree that your contributions will be licensed under the MIT License.

## 🙏 Recognition

Contributors will be recognized in:
- README.md acknowledgments
- Release notes
- Contributors page
- Special thanks in documentation

Thank you for helping make Epoch of Elria better! 🌟
