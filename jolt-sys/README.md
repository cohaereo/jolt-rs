# JoltPhysicsC v0.0.6 - C API for Jolt Physics C++ library

[Jolt Physics](https://github.com/jrouwe/JoltPhysics) is a fast and modern physics library written in C++.

This project aims to provide high-performance, consistent and roboust C API for Jolt.<br/>
Based on the JoltC from the [zig-gamedev zphysics library](https://github.com/michal-z/zig-gamedev/tree/main/libs/zphysics/libs/JoltC)

JoltPhysicsC is not yet complete but already usable, to get started please take a look at the examples.

Folder structure:

* `Jolt/` - contains the complete source code of Jolt Physics 3.0.1
* `JoltC/`
    * `JoltPhysicsC.h` - C API header file
    * `JoltPhysicsC.cpp` - C API implementation
    * `JoltPhysicsC_Extensions.cpp` - some additional, low-level functions implemented for performance reasons
    * `JoltPhysicsC_Tests.c` - tests for our C API
