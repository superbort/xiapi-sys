## Low-level FFI bindings for xiAPI

### Supported versions and cameras
These bindings have been tested with xiAPI version 4.25 on Windows and Linux. Newer version will likely also work, but
some new features may be unsupported. All cameras that are supported by xiAPI are also supported by these bindings.

### Requirements
To use these bindings, the XIMEA software package must be installed in the default path 
(For Windows: C:\XIMEA; For Linux: /opt/XIMEA).

### Documentation
Specific documentation for this package is still WIP.
For general documentation on xiAPI please have a look at the [API manual](https://www.ximea.com/support/wiki/apis/XiAPI_Manual).

### MacOS Support
To build on macOS you need to install the m3api framework manually. To do this:
1. Download the software from [here](https://staging2.ximea.com/software-downloads)
2. Mount the DMG
3. Copy `m3api.framework` to `/Library/Frameworks`. This will require root permissions.
