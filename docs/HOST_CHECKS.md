# After Effects preview acceptance

Status: **not run in After Effects**. Automated core tests and Windows builds
cannot establish successful load, visual output, installer execution or parity.
Run on a saved copy of a project. Record AE version/build, Windows build, GPU
and driver, color space, preview resolution and bit depth.

1. Close AE, install the setup, reopen AE and find Effects > Nasaat > GlitchTrace.
   About should show GlitchTrace 1.0 Preview 1 and Made by Nasaat.
2. In a 640x360 composition, precompose two separated green shapes on an opaque
   black background. Apply the effect to the precomp. Set Key Color green,
   Detection Quality Extreme and Threshold 10. Show Mask should isolate them;
   disable it and expect two frames and XY labels.
3. Toggle each of boxes/text/markers/connections independently. Set opacity,
   thickness and fill to 0 and verify zero remains zero. Test Dot, Plus, Cross,
   gaps, negative/zero/positive curvature, dashes and filled/outlined arrows at
   start/middle/end. With >=3 regions verify the closing connection.
4. Alpha Layer: only the graphics should remain over AE's checkerboard. Check
   semitransparent edges over bright and dark backgrounds for dark fringes.
5. Switch the project to 16 bpc; compare detection/appearance. The current effect
   does not advertise 32 bpc. Record what AE does at 32 bpc; no native HDR claim.
6. Test full/half/quarter and asymmetric preview resolution, objects against each
   layer edge, empty/transparent footage, very small layers and cropped bounds.
   Output is intentionally limited to source bounds; outside labels can clip.
7. Save/reopen, duplicate instances with different settings, undo parameter
   changes, scrub out of order and render the same frames twice. Capture ordering
   or coordinate differences. MFR support is not advertised.
8. Try large blur values, 4K, dense regions and >512 separated regions. Cancel
   during processing. AE should remain usable; record render times and any error.
9. Click Copy Diagnostic Report immediately after reproducing a problem. Paste
   it into text. Verify version/build, frame dimensions, depth and original error.
   Separately run Start > GlitchTrace > Collect Diagnostics and review its report.
10. Close AE and reinstall/update. Confirm backups exist before overwriting;
    uninstall should remove this product, preserve backups and leave the original
    reference plugin untouched. Check a clean Windows installation separately.

For side-by-side parity testing use the exact supplied reference SHA described
in REFERENCE_ANALYSIS.md. Do not compare against Tracery 2 or another release and
attribute differences to this reference without recording that change of input.
