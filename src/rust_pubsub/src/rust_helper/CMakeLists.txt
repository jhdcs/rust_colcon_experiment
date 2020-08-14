set(THIS_LIB rust_helper)
set(THIS_LIB_BRIDGED ${THIS_LIB}_bridged)

add_crate(Cargo.toml)

# # Tie Rust's compiler to compile in the same mode as the rest of the CMake project
# if (CMAKE_BUILD_TYPE STREQUAL "Debug")
#     message(STATUS "Compiling Rust in Debug mode")
#     set(CARGO_CMD cargo build)
#     set(TARGET_DIR "debug")
# else ()
#     message(STATUS "Compiling Rust in Release mode")
#     set(CARGO_CMD cargo build --release)
#     set(TARGET_DIR "release")
# endif ()

# set(CLIENT_SO "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/librust_helper.a")

# # Compile the Rust library
# message(STATUS "Running Cargo on Rust package")
# add_custom_target(rust_helper_lib ALL
#     COMMENT "Compiling client module"
#     COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD} 
#     COMMAND cp ${CLIENT_SO} ${CMAKE_CURRENT_BINARY_DIR}
#     WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/src/rust_helper)
# set_target_properties(rust_helper_lib PROPERTIES LOCATION ${CMAKE_CURRENT_BINARY_DIR})

get_property(THIS_LIB_BRIDGED_STATIC
  TARGET ${THIS_LIB_BRIDGED}
  PROPERTY INTERFACE_LINK_LIBRARIES
)

# Set up the code generator for the Rust library's .h file
add_custom_command(
  OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.h
  COMMAND cxxbridge ${CMAKE_CURRENT_SOURCE_DIR}/src/lib.rs --header > ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.h
  COMMENT "Building C++ bridge header files"
	DEPENDS src/lib.rs)
# add_custom_target(rust_h ALL
#   COMMENT "Building C++ bridge header files"
#   COMMAND cxxbridge ${CMAKE_CURRENT_SOURCE_DIR}/src/rust_helper/src/lib.rs --header > ${CMAKE_CURRENT_BINARY_DIR}/rust_bridge.h
#   DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/src/rust_helper/src/lib.rs)

# Set up the code generator for the Rust library's .cc file
add_custom_command(
  OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.cc
  COMMAND cxxbridge ${CMAKE_CURRENT_SOURCE_DIR}/src/lib.rs > ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.cc
  COMMENT "Building C++ bridge cc files"
  DEPENDS src/lib.rs)
# add_custom_target(rust_cc ALL
#   COMMENT "Building C++ bridge cc files"
#   COMMAND cxxbridge ${CMAKE_CURRENT_SOURCE_DIR}/src/rust_helper/src/lib.rs > ${CMAKE_CURRENT_BINARY_DIR}/rust_bridge.cc
#   DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/src/rust_helper/src/lib.rs)

add_library(${THIS_LIB} SHARED
  ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.h
  ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.cc
)

target_include_directories(${THIS_LIB} PUBLIC src ${CMAKE_CURRENT_BINARY_DIR})
target_link_libraries(${THIS_LIB}
  PRIVATE
    -Wl,--whole-archive
    $<TARGET_FILE:${THIS_LIB_BRIDGED_STATIC}>
    -Wl,--no-whole-archive
  PUBLIC
    ${THIS_LIB_BRIDGED}
)
