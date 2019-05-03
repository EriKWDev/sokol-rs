/*!

This crate provides a renderer backend and user input handler to
[imgui-sys](https://crates.io/crates/imgui-sys). It is aimed to ease integration of
[Dear ImGui](https://github.com/ocornut/imgui) for applications using the sokol API.

*/

extern crate sokol;
extern crate sokol_sys as sys;

pub mod gfx;
pub mod imgui;
