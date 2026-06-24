# howtovulkan-rs

## Overview

An implementation of the simple renderer described in [howtovulkan](https://howtovulkan.com) but in Rust and GLSL (vs C++ and Slang),
with a number of tweaks made as a result (+ Cel shading!).

### Changes made from howtovulkan

 - GLSL instead of Slang (and thus a UBO is used instead of a push constant for sending extra data to the vertex shader)
 - Cel Shading :3
    - Outlines drawn with a thick wireframe pipeline for backfaces behind a normal polygon-fill pipeline for front faces
    - Quantised Phong lighting

### Usage (keybinds)

 - Left and right arrow to change model selection (indicated by increased brightness)
 - Hold left click and pan up and down to rotate the selected model
 - Zoom in and out by scrolling or using -/= keys (if, say, trackpad scrolling is not registering)
 - Increase and decrease the shininess of the model material with 9/0 keys
 - Quit the program by pressing Q

### Other Info

howtovulkan-rs uses [ash](https://docs.rs/ash), providing direct (almost) bindings to Vulkan.  

[vk_format](./src/vk_format.rs), [gl_format](./src/gl_format.rs), and [extra_ktx](./src/extra_ktx.rs) contain constants and lookup functions for formats from libktx,
as at time of writing, there are no maintained Rust crates providing bindings or either full support for the
KTX format.   

No generative AI was used in the writing of howtovulkan-rs.

## Installation

See releases for a tarball containing an appimage and folders containing model and shader files.   

howtovulkan-rs has been tested as an appimage natively on Arch Linux with Intel integrated graphics (UHD Graphics 620),
NixOS (26.11, nixpkgs-unstable), with a discrete AMD GPU (rx 6800), and a Linux Mint virtual machine (Linux Mint 22.3).

### Usage

Run the appimage in the same directory as the model and shader directories. See [keybinds](#usage-keybinds).

### Dependencies

Running the appimage only requires Vulkan drivers to be installed (e.g. vulkan-intel or vulkan-radeon on Arch), which 
most systems should already have. The Vulkan loader (vulkan-icd-loader on Arch) is technically also required, but is often installed 
automatically alongside any Vulkan driver.

## Building and Running from Source

howtovulkan-rs can be built and run with cargo. The no-layers feature is provided for non-development builds which do not require 
the Vulkan validation layers to be installed. Note that release mode results in strange behaviour and crashing.   

Build with either `cargo build` or `cargo build --features no-layers` for systems without the Vulkan validation layers.

### Dependencies

Building requires cargo, as well as shaderc (if shaderc is not installed, cmake is required ahd shaderc with be built when compiling howtovulkan-rs).   

Running using cargo has the same requirements as running the appimage.

### Packaging

A [packaging script](./package.sh) is provided as a convenient means of producing a new appimage, and has the same requirements as building.
