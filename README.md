# Kumpel Engine

This is a simple game engine written in rust and using WGPU as graphics API. 
It is basically a learning project for me as well as a good opportunity
to create a game totally from scratch (well, maybe not totally 'totally').
Generative AI will be used only for research purposes. My goal is to have
fun programming the oldschool way. I will update this as I go.


## Roadmap

I plan to implement the engine along the following roadmap:

### Foundation

- [x] Windowing using `winit`
- [x] Input management using `winit`
- [ ] Implement the game loop
  - [ ] Consume events (mouse, keyboard, window events)
  - [ ] Update game logic with delta time (`dt`)
  - [ ] Render the current game state to screen

### Renderer (WebGPU)

- [x] Initialising with `Instance`, `Adapter`, `Device` and `Queue`
- [x] Ressource management (e.g. textures, buffers for indices and vertices, shader)
- [x] Render pipelines
- [x] Render pass (i.e. drawing frames)
- [x] Implement vector maths using `glam`
- [x] Implement MVP matrices (model, view, projection)
- [x] Create uniform buffers and bind groups
- [x] Add depth texture and update pipeline

### Game World

- [x] Implement an Entity Component System (ECS) with simple IDs
- [x] Components like raw data without logic (e.g. position, velocity, mesh)
- [ ] Systems with only logic (e.g. movement system)

### Asset Management

- [x] Build a central asset manager
- [ ] Asynchronous asset loading to prevent freezing
- [x] Reference management via handle IDs

### Editor

- [ ] Implement an editor based on Constructive Solid Geometry (CSG)


## Milestones

### Milestone 1: Time and engine architecture refactoring

Before I implement complex physics or editor controls, I need stable time measurements.
Currently updates are pumped out as fast as the computer can go.

- [ ] Delta Time (dt): Timespan between two frames. Updates should be decoupled from monitor-framerate.
- [ ] ECS refactoring: Currently everything is done in winits `RedrawRequested`-Event. Implement clean functions like `system_update_camera(world, dt)` and `system_update_lights(world, dt)`. Call them before drawing.

### Milestone 2: Loading 3D models

- [ ] Use crates like `tobj` or `gltf` to load 3d models
- [ ] Mesh Asset Manager: Load `.obj` files to RAM, build vertex and index buffers and return a handle to ECS.

### Milestone 3: UI and editor preparation

- [ ] `egui` integration, because it has bindings for WGPU
- [ ] Raycasting / mouse picking: click on screen and select an object

### Milestone 4: Advanced rendering

- [ ] Depth and Shadows (shadow mapping)
- [ ] Specular mapping / PBR


## The Game

The game I want to create will be grid based dungeon crawler with pixel art 
style. Enemies will be simple animated sprites and fights will be turn based.
Ideas come from games like *Orcs and Elves* and *Kingsfield* and similar
old school games. The game will be set in a dark fantasy environment.
I will try to do everything from scratch or by myself as good as possible.
The phase of developing the game will obviously start once the engine is
done.

