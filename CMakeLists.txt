cmake_minimum_required(VERSION 3.2)
project(interop-test VERSION 0.1.0 LANGUAGES CXX)

# Include and lib directories
include_directories(${CMAKE_SOURCE_DIR}/librtest/include)
link_directories(${CMAKE_SOURCE_DIR}/librtest/target/release)

# Add runtime project
add_subdirectory(librtest)

# Add nballerinacc
file(GLOB SOURCES
${PROJECT_SOURCE_DIR}/src/*.cpp
)
add_executable(interopcc ${SOURCES})
add_dependencies(interopcc librtest)
target_link_libraries(interopcc PRIVATE rtest pthread dl)

