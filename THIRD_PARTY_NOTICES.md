# Third-party notices

GlitchTrace's original implementation is MIT licensed. Dependencies retain their
own licenses. Windows packages include a `third-party-licenses` directory generated
from the exact Cargo.lock source packages; it must accompany redistribution.

- `after-effects` 0.4.0 and `after-effects-sys` 0.4.0: Adrian Eddy, Moritz Moeller
  and contributors. [Upstream](https://github.com/virtualritz/after-effects).
  The package declares Apache-2.0 OR BSD-3-Clause OR MIT OR Zlib. The bindings include
  pre-generated interfaces for Adobe's AE SDK 25.6. No proprietary SDK archive is
  downloaded or bundled by GlitchTrace.
- `pipl` 0.1.1: Adrian Eddy and contributors; MIT OR Apache-2.0.
  [Upstream](https://github.com/virtualritz/after-effects/tree/master/pipl).
- Other transitive dependencies are pinned in Cargo.lock; their individual notices
  and license files are bundled with binary packages.

The supplied reference identifies as DRAGOY Tracery, by Dragoy. GlitchTrace is an
independent implementation, not an official Dragoy product. The reference binary,
its shader source, font atlas, licensing library, trademarks and private build
paths are not included. This project's license does not grant rights to them.
Adobe and After Effects are trademarks of Adobe.
