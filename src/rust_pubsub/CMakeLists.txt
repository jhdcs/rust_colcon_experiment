cmake_minimum_required(VERSION 3.5)
project(rust_pubsub)

# Default to C99
if(NOT CMAKE_C_STANDARD)
  set(CMAKE_C_STANDARD 99)
endif()

# Default to C++14
if(NOT CMAKE_CXX_STANDARD)
  set(CMAKE_CXX_STANDARD 14)
endif()

if(CMAKE_COMPILER_IS_GNUCXX OR CMAKE_CXX_COMPILER_ID MATCHES "Clang")
  add_compile_options(-Wall -Wextra -Wpedantic)
endif()

message(STATUS "Working on Rust")
message(STATUS "Source dir: " ${CMAKE_CURRENT_SOURCE_DIR})
message(STATUS "Binary dir: " ${CMAKE_CURRENT_BINARY_DIR})

# Tie Rust's compiler to compile in the same mode as the rest of the CMake project
if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    message(STATUS "Compiling Rust in Debug mode")
    set(CARGO_CMD cargo build)
    set(TARGET_DIR "debug")
else ()
    message(STATUS "Compiling Rust in Release mode")
    set(CARGO_CMD cargo build --release)
    set(TARGET_DIR "release")
endif ()

set(CLIENT_SO "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/librust_helper.a")

# Compile the Rust library
message(STATUS "Running Cargo on Rust package")
add_custom_target(rust_helper_lib ALL
    COMMENT "Compiling client module"
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD} 
    COMMAND cp ${CLIENT_SO} ${CMAKE_CURRENT_BINARY_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/src/rust_helper)
set_target_properties(rust_helper_lib PROPERTIES LOCATION ${CMAKE_CURRENT_BINARY_DIR})

# Set up the code generator for the Rust library's .h file
# add_custom_command(TARGET rust_helper_lib
#   # OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.h
#   COMMAND cxxbridge ${CMAKE_CURRENT_SOURCE_DIR}/src/rust_helper/src/lib.rs --header > ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.h
#   COMMENT "Building C++ bridge header files"
# 	DEPENDS src/lib.rs)
add_custom_target(rust_h ALL
  COMMENT "Building C++ bridge header files"
  COMMAND cxxbridge ${CMAKE_CURRENT_SOURCE_DIR}/src/rust_helper/src/lib.rs --header > ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.h)

# Set up the code generator for the Rust library's .cc file
# add_custom_command(TARGET rust_helper_lib
#   # OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.cc
#   COMMAND cxxbridge ${CMAKE_CURRENT_SOURCE_DIR}/src/rust_helper/src/lib.rs > ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.cc
#   COMMENT "Building C++ bridge cc files"
#   DEPENDS src/lib.rs)
add_custom_target(rust_cc ALL
  COMMENT "Building C++ bridge cc files"
  COMMAND cxxbridge ${CMAKE_CURRENT_SOURCE_DIR}/src/rust_helper/src/lib.rs > ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.cc)

# find dependencies
find_package(ament_cmake REQUIRED)
find_package(rclcpp REQUIRED)
find_package(std_msgs REQUIRED)

# Link up the talker
add_executable(talker src/publisher_member_function.cpp)
target_include_directories(talker
  PRIVATE
  ${CMAKE_CURRENT_BINARY_DIR})
add_dependencies(talker rust_helper_lib rust_h rust_cc)
ament_target_dependencies(talker rclcpp std_msgs)

# Link up the listener
add_executable(listener src/subscriber_member_function.cpp)
ament_target_dependencies(listener rclcpp std_msgs)

install(TARGETS
  talker
  listener
  DESTINATION lib/${PROJECT_NAME})

ament_package()
