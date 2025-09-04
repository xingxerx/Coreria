# Makefile for Epoch of Elria Game Engine

CXX = g++
CXXFLAGS = -std=c++17 -Wall -Wextra -O2 -pthread
LDFLAGS = -lsfml-graphics -lsfml-window -lsfml-system -pthread

# Directories
OBJDIR = Build
BINDIR = Bin
EXTERNAL = .

# Source files
LEGACY_SOURCES = \
    main.cpp \
    $(EXTERNAL)/GameObject.cpp \
    $(EXTERNAL)/GameEnvironment.cpp \
    $(EXTERNAL)/Vector2D.cpp \
    $(EXTERNAL)/graphics.cpp

DREAM_WEAVER_SOURCES = \
    main_dream_weaver_complete.cpp \
    $(EXTERNAL)/GameObject3D.cpp \
    $(EXTERNAL)/GameObject.cpp \
    $(EXTERNAL)/GameEnvironment.cpp \
    $(EXTERNAL)/Vector2D.cpp

RPG_CHARACTERS_SOURCES = \
    main_rpg_characters.cpp \
    $(EXTERNAL)/GameObject3D.cpp \
    $(EXTERNAL)/GameObject.cpp \
    $(EXTERNAL)/GameEnvironment.cpp \
    $(EXTERNAL)/Vector2D.cpp

OPEN_WORLD_3D_SOURCES = \
    main_3d_openworld.cpp \
    $(EXTERNAL)/GameObject3D.cpp \
    $(EXTERNAL)/GameObject.cpp \
    $(EXTERNAL)/GameEnvironment.cpp \
    $(EXTERNAL)/Vector2D.cpp

MEMORY_SYSTEM_SOURCES = \
    main_memory_system.cpp \
    $(EXTERNAL)/GameObject3D.cpp \
    $(EXTERNAL)/GameObject.cpp \
    $(EXTERNAL)/GameEnvironment.cpp \
    $(EXTERNAL)/Vector2D.cpp

# Object files
LEGACY_OBJECTS = $(LEGACY_SOURCES:%.cpp=$(OBJDIR)/%.o)
DREAM_WEAVER_OBJECTS = $(DREAM_WEAVER_SOURCES:%.cpp=$(OBJDIR)/%.o)
RPG_CHARACTERS_OBJECTS = $(RPG_CHARACTERS_SOURCES:%.cpp=$(OBJDIR)/%.o)
OPEN_WORLD_3D_OBJECTS = $(OPEN_WORLD_3D_SOURCES:%.cpp=$(OBJDIR)/%.o)
MEMORY_SYSTEM_OBJECTS = $(MEMORY_SYSTEM_SOURCES:%.cpp=$(OBJDIR)/%.o)

# Targets
TARGETS = \
    $(BINDIR)/game_engine_parallel_demo \
    $(BINDIR)/dream_weaver_complete \
    $(BINDIR)/rpg_characters \
    $(BINDIR)/game_3d_openworld \
    $(BINDIR)/memory_system

# Default target
all: directories $(TARGETS)

# Create necessary directories
directories:
	@mkdir -p $(OBJDIR) $(BINDIR)

# Main game engine (parallel demo)
$(BINDIR)/game_engine_parallel_demo: $(LEGACY_OBJECTS)
	$(CXX) $(CXXFLAGS) -o $@ $^ $(LDFLAGS)

# Complete dream weaver game
$(BINDIR)/dream_weaver_complete: $(DREAM_WEAVER_OBJECTS)
	$(CXX) $(CXXFLAGS) -o $@ $^ $(LDFLAGS)

# RPG characters demo
$(BINDIR)/rpg_characters: $(RPG_CHARACTERS_OBJECTS)
	$(CXX) $(CXXFLAGS) -o $@ $^ $(LDFLAGS)

# 3D open world demo
$(BINDIR)/game_3d_openworld: $(OPEN_WORLD_3D_OBJECTS)
	$(CXX) $(CXXFLAGS) -o $@ $^ $(LDFLAGS)

# Memory system demo
$(BINDIR)/memory_system: $(MEMORY_SYSTEM_OBJECTS)
	$(CXX) $(CXXFLAGS) -o $@ $^ $(LDFLAGS)

# Object file compilation rules
$(OBJDIR)/%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

$(OBJDIR)/%.o: $(EXTERNAL)/%.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

# Run targets
run-parallel: $(BINDIR)/game_engine_parallel_demo
	./$(BINDIR)/game_engine_parallel_demo

run-complete: $(BINDIR)/dream_weaver_complete
	./$(BINDIR)/dream_weaver_complete

run-rpg: $(BINDIR)/rpg_characters
	./$(BINDIR)/rpg_characters

run-3d: $(BINDIR)/game_3d_openworld
	./$(BINDIR)/game_3d_openworld

run-memory: $(BINDIR)/memory_system
	./$(BINDIR)/memory_system

# Clean targets
clean:
	rm -rf $(OBJDIR)

distclean: clean
	rm -rf $(BINDIR)

.PHONY: all directories clean distclean run-parallel run-complete run-rpg run-3d run-memory
