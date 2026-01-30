# phosphorust

I've always been interested in computer graphics, so this is a project where I implement the theoretical software renderers outlined in Gabriel Gambetta's *Computer Graphics From Scratch* (2021). In Rust! I'm still new to the language, so this will be an enlightening experience in more than one dimension.

This project will eventually develop into the renderer for my experimental ECS-based game engine in Rust, coming soon.

## Where We Are 

Right now we're just a software raytracer with one kind of primitive (spheres), diffuse and specular lighting, and reflections, which I'm very excited about. Triangles coming soon, with a different engine, a rasterizer.

<img src="img/example1.png" alt="An image of the software raytracer in action" width="300" height="300">

## Following Along

### Using Nix 

To run it yourself, I recommend Nix, because that's what I use. Simply clone the repository:

```bash
git clone https://github.com/TotallyNotBiased/phosphorust.github
cd phosphorust
```

and enter the environment.

```bash
nix develop
```

Then you can just run it:

```bash
cargo run --bin raytracer
```

### Without Nix

Without Nix, you'll need to make sure you have the right stuff installed. Have the [Rust toolchain](https://rustup.rs/) and `winit` dependencies such as `libx11-dev`, `libwayland-dev`, `libxkbcommon-dev` (on WSL2).

Then you can
```bash
cargo run
```

### Compatibility

Note that I had to disable wayland due to compatibility issues with my WSL2 environment. You can use wayland in native Linux, probably. There are comments in `main.rs` for that.
