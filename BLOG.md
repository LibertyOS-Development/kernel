# BLOG

## August 22, 2022
Today I have updated some (very little) documentation, added this file, and prepared some plans for the next updates. There are many things planned for the 1.0 release, so I would rather spend this post discussing the shift in the development strategy for this project.

Rather than writing a specific language for LOS, learning how to write in that language, and writing documentation for said language, as well as all the necessary programs and features to use said language, I have decided to change the approach to developing this kernel. The focus of my development (as the lead developer for this project) is to provide independent developers with as many features as I can, so that they can improve the kernel, write software for it, port programs over to LibertyOS, test the kernel, and get the word out about this project. The major focus that I have been working on is support for different architectures, as well as a mathematics library. The mathematics library will be part of the larger libcore crate, and will provide support for mathematical functions, measurements, converting between various formats, and other miscellaneous features that will be useful for expanding upon what has already been added to this kernel.

The architecture that is going to be the primary target for LibertyOS (as of now) is x86_64, but there are plans for supporting many different architectures, including microcontrollers and less common processors. There will also be support for various devices, so that this OS can be used in more situations.

This OS is not dead. There will be more updates coming soon.

- Daniel Teberian