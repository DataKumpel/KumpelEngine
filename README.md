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
- [ ] Input management using `winit`
- [ ] Implement the game loop
  - [ ] Consume events (mouse, keyboard, window events)
  - [ ] Update game logic with delta time (`dt`)
  - [ ] Render the current game state to screen

### Renderer (WebGPU)

- [x] Initialising with `Instance`, `Adapter`, `Device` and `Queue`
- [ ] Ressource management (e.g. textures, buffers for indices and vertices, shader)
- [x] Render pipelines
- [x] Render pass (i.e. drawing frames)
- [ ] Implement vector maths using `glam`
- [ ] Implement MVP matrices (model, view, projection)
- [ ] Create uniform buffers and bind groups
- [ ] Add depth texture and update pipeline

### Game World

- [ ] Implement an Entity Component System (ECS) with simple IDs
- [ ] Components like raw data without logic (e.g. position, velocity, mesh)
- [ ] Systems with only logic (e.g. movement system)

### Asset Management

- [ ] Build a central asset manager
- [ ] Asynchronous asset loading to prevent freezing
- [ ] Reference management via handle IDs

### Editor

- [ ] Implement an editor based on Constructive Solid Geometry (CSG)


## The Game

The game I want to create will be grid based dungeon crawler with pixel art 
style. Enemies will be simple animated sprites and fights will be turn based.
Ideas come from games like *Orcs and Elves* and *Kingsfield* and similar
old school games. The game will be set in a dark fantasy environment.
I will try to do everything from scratch or by myself as good as possible.
The phase of developing the game will obviously start once the engine is
done.

