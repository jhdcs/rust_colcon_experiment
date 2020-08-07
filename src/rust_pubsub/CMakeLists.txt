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

# message(STATUS "Running rust_helper CMAKE")
# add_subdirectory(src/rust_helper)

message(STATUS "Working on Rust")
message(STATUS "Source dir: " ${CMAKE_CURRENT_SOURCE_DIR})
message(STATUS "Binary dir: " ${CMAKE_CURRENT_BINARY_DIR})

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    message(STATUS "Compiling Rust in Debug mode")
    set(CARGO_CMD cargo build)
    set(TARGET_DIR "debug")
else ()
    message(STATUS "Compiling Rust in Release mode")
    set(CARGO_CMD cargo build --release)
    set(TARGET_DIR "release")
endif ()

set(CLIENT_SO "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/librust_helper.so")

# message(STATUS "Generating C++ bridge header files")
# add_custom_command(OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.h
#   COMMAND cxxbridge ${CMAKE_CURRENT_SOURCE_DIR}/src/rust_helper/src/lib.rs --header > ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.h
#   COMMENT "Building C++ bridge header files"
# 	DEPENDS src/lib.rs)

# message(STATUS "Generating C++ bridge cc files")
# add_custom_command(OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.cc
#   COMMAND cxxbridge ${CMAKE_CURRENT_SOURCE_DIR}/src/rust_helper/src/lib.rs > ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.cc
#   COMMENT "Building C++ bridge cc files"
# 	DEPENDS src/lib.rs)

message(STATUS "Running Cargo on Rust package")
add_custom_target(client ALL
    COMMENT "Compiling client module"
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD} 
    COMMAND cp ${CLIENT_SO} ${CMAKE_CURRENT_BINARY_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/src/rust_helper)
set_target_properties(client PROPERTIES LOCATION ${CMAKE_CURRENT_BINARY_DIR})
# add_dependencies(client
#   ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.h
#   ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.cc
# )

add_custom_command(TARGET client
  # OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.h
  COMMAND cxxbridge ${CMAKE_CURRENT_SOURCE_DIR}/src/rust_helper/src/lib.rs --header > ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.h
  COMMENT "Building C++ bridge header files"
	DEPENDS src/lib.rs)

add_custom_command(TARGET client
  # OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.cc
  COMMAND cxxbridge ${CMAKE_CURRENT_SOURCE_DIR}/src/rust_helper/src/lib.rs > ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.cc
  COMMENT "Building C++ bridge cc files"
  DEPENDS src/lib.rs)

add_library(rust_helper_library STATIC IMPORTED GLOBAL)
add_dependencies(rust_helper_library ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.h ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.cc)

# find dependencies
find_package(ament_cmake REQUIRED)
find_package(rclcpp REQUIRED)
find_package(std_msgs REQUIRED)

add_executable(talker src/publisher_member_function.cpp)
add_dependencies(talker rust_helper_library)
ament_target_dependencies(talker rclcpp std_msgs)

add_executable(listener src/subscriber_member_function.cpp)
ament_target_dependencies(listener rclcpp std_msgs)

install(TARGETS
  talker
  listener
  DESTINATION lib/${PROJECT_NAME})

ament_package()
