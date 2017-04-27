# first-rs-engine
A small stupid engine written in rust using the gfx-rs crate. It is not meant to be a good engine. Its purpose of the engine is that I learn Rust and gfx-rs.


## Installation 

```
git clone https://github.com/SiebenCorgie/first-rs-engine.git
cd first-rs-engine
cargo run
```
This should download, compile and run the project. At the moment you'll get some cubes and a fps-camera. 


## Goals

- modular engine system of different mangers like "model_manager", "light_manager", "camera_manager"
- different game objects prefixed with `g_` like cameras, objects etc.

- opengl based rendering. Maybe Vulkan at a later state
- forward renderer with a fixed size of max lights per scene (have to investigate the performance there :D)
- in distant future a forward+ renderer, maybe for another second engine where I correct the failures I do with this one

- ability to load models
- maybe a gltf 2.0 loader

- material system based on a master shader
- ability to load custom textures per material

- Automatic documentation
