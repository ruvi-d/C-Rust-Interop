# Add Rust Runtime
include(ExternalProject)
set_directory_properties(PROPERTIES EP_PREFIX ${CMAKE_BINARY_DIR}/librtest)
ExternalProject_Add(
    librtest
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BUILD_COMMAND cargo build --release
    BINARY_DIR "${CMAKE_SOURCE_DIR}/librtest"
    INSTALL_COMMAND cbindgen --config cbindgen.toml --crate librtest --output include/librtest.h
    LOG_BUILD ON
    BUILD_ALWAYS TRUE)
