# caustic

***caustic*** provides a useful **ray-tracing** (or rather **path-tracing**) API for further Rust or C graphics-oriented projects.

*Further documentation will be handed out soon*


## Visuals

### Images

Images in **caustic** are serialized into the *PPM* format for easy encode-decode operations, which this file doesn't explore in depth. I suggest reading [the official Wikipedia article on PPM format](https://en.wikipedia.org/wiki/Portable_pixmap) for further details, some of which I mark down here for mere self-remembrance. Most importantly is that *PPM* uses RGB channels, unlike its *PBM* and *PGM* variants, which are differentiated by a plain magic number beginning of file.

*PPM* is **NOT** compressed, making it a very simple, but bulky type to play around with.

*Further documentation will be handed out soon*