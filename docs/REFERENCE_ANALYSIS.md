# Reference analysis and parity contract

Reference: user-supplied `MyTracePlugin.aex`, SHA-256
`108bb818afc250c4076e24efbb297498e963b4ae97b00455ee2ca298dd5a4f7a`.
The input remains outside this repository. Its inventory is recorded in
[reference_manifest.json](reference_manifest.json), reproducible with
`python tools/analyze_reference.py /path/to/reference.aex`.

## Confirmed statically

- PE32+ Windows x64 DLL. PiPL effect name Tracery, category Dragoy, match name
  DRAGOY Tracery. Exports EffectMain and PluginDataEntryFunction2.
- OpenGL renderer, GLSL 330 shader strings, video/mask/font textures and a uniform
  buffer with 512 bounding boxes. Runtime dependencies include Microsoft Visual
  C++ libraries. The reference contains aescripts licensing and watermark logic.
- RGB Euclidean color distance is blurred with a square Gaussian kernel.
  A smoothstep over threshold/100 is converted to a binary mask at its midpoint.
- Graphics order: box fill, box borders, connections, center markers, coordinate
  labels. Alpha Layer starts with transparent output; normal mode uses footage
  or binary mask preview as the base.
- Quadratic Bezier control point is the midpoint offset perpendicularly by
  endpoint distance times curvature. The reference uses a tiny nonzero offset
  at exactly zero curvature to avoid its analytic solver degeneracy.
- Line dashes use approximate arc length; arrows follow the curve tangent.
  Successive centers are connected; the last connects to the first for >=3 boxes.
- Dot size is a radius; Plus/Cross endpoints use half the marker size.
- Labels format X/Y pixel coordinates using a font atlas. No editable arbitrary
  text control was found. A disabled background define occurs only once and is
  not referenced by the displayed text function.

## Independent implementation and remaining differences

| Area | GlitchTrace status | Evidence / remaining gap |
|---|---|---|
| Exposed graphics controls | Implemented | Source and synthetic core tests; AE UI not run |
| Color-distance/Gaussian/binary mask | Implemented | Independent separable Gaussian; edge behavior chosen as clamp |
| Region extraction | Implemented independently | Eight-connected components; original CPU connectivity/filtering/sorting not recovered |
| Quality levels | All six exposed | Strides 1/2/3/4/6/8 are independent choices, not recovered reference values |
| Defaults/ranges/units | Usable independent defaults | Exact original values not recovered; not certified identical |
| Box/fill/gap composition | Implemented | Pixel bounds/antialiasing can differ |
| Coordinate labels | Implemented | Original 5x7 numeric font; original atlas/metrics are not reused |
| Markers | Implemented | Same three types; antialiasing can differ |
| Curves/dashes/arrows | Implemented | Polyline approximation vs original analytic shader; zero curvature is straight |
| Transparent output | Implemented | Premultiplied-alpha synthetic tests; AE render still required |
| Maximum region count | 512 | Original shader capacity; original CPU cap behavior unknown |
| 8/16 bpc | Adapter implemented | Windows compilation; AE load/render still required |
| OpenGL/GPU | Not implemented | CPU reference renderer, no acceleration parity claim |
| 32 bpc / SmartFX / MFR | Not advertised | Separate host work if needed; reference support not established by strings alone |
| Temporal identity / motion tracking | Not implemented | Original tracking persistence not established |
| License/activation/watermark | Not reproduced | Not required for this independent product's image-processing functions |
| Project/preset migration | Not implemented | Independent match name and parameter IDs |

This is a functional reimplementation preview, **not a recovered source project,
complete decompilation, 100% clone, or validated replacement**. No shader source,
font atlas, license framework or binary bytes from the reference are published.
A disassembly/strings inventory cannot prove the complete runtime behavior.

## Comparison needed for exact parity

Render both effects on the same test footage/settings, AE build, project color
space, bit depth, resolution and frame range. Record defaults, detections, ordering,
preview scales and timings. Test single pixels, noise, touching regions, partial
alpha, edge objects, >512 components, disappearing/reappearing objects and curves
with negative/zero curvature. Compare box coordinates separately from rasterized
pixels. Choose explicit tolerances and performance targets from these results.
Do not infer original temporal tracking from boxes moving on consecutive frames.

## Source/build choice

The native Rust binding source and published crate were inspected; crate 0.4.0
has a different default callback mutability than current upstream main, and the
adapter is checked against the locked release. Its pre-generated bindings avoid
requiring an SDK download from the user. Binding boilerplate and PiPL are from
that dependency, not reconstructed from a guessed host ABI.

Primary interfaces: [Adobe AE development](https://developer.adobe.com/after-effects/),
[bindings](https://github.com/virtualritz/after-effects),
[crate 0.4.0](https://docs.rs/after-effects/0.4.0/after_effects/).
