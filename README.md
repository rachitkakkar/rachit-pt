# rachit-pt
A toy rendering engine written Rust. Generates beautiful images of 3D scenes with path tracing.

![Current Output](output.png)
*Render time: [01:31:56]*

## Planned Features
- A multithreaded progressive rendering system
- A Phong bidirectional reflectance distribution function (BRDF) Model for physically-based rendering
- HDR skybox support with sRGB gamma correction ✅
- Depth of field ✅
- Bounding box acceleration structures
- Support for triangle meshes / GLTF and OBJ Model loading
- Texture and normal map support
- Terminal progress bar ✅
- GitHub continuous integration to build the Rust project on commit ✅

## Low-Priority Future Improvements
- Disney BSDF support

## Sources
'Ray Tracing in One Weekend' by Peter Shirley https://raytracing.github.io/

'An Anisotropic Phong BRDF Model' by Michael Ashikhmin and Peter Shirley https://www.cs.utah.edu/~shirley/papers/jgtbrdf.pdf