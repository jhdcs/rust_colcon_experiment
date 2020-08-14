set(THIS_LIB rust_helper)
set(THIS_LIB_BRIDGED ${THIS_LIB}_bridged)

add_crate(Cargo.toml)

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

# Set up the code generator for the Rust library's .cc file
add_custom_command(
  OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.cc
  COMMAND cxxbridge ${CMAKE_CURRENT_SOURCE_DIR}/src/lib.rs > ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.cc
  COMMENT "Building C++ bridge cc files"
  DEPENDS src/lib.rs)

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
