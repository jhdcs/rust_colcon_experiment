cmake_minimum_required(VERSION 3.10)
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

# Set up Corrosion (for working with Rust)
# find_package(Corrosion REQUIRED)
add_subdirectory(src/corrosion)

message(STATUS "Working on Rust")
message(STATUS "Source dir: " ${CMAKE_CURRENT_SOURCE_DIR})
message(STATUS "Binary dir: " ${CMAKE_CURRENT_BINARY_DIR})

add_subdirectory(src/rust_helper)

# find dependencies
find_package(ament_cmake REQUIRED)
find_package(rclcpp REQUIRED)
find_package(std_msgs REQUIRED)

# Link up the talker
add_executable(talker src/publisher_member_function.cpp)

target_link_libraries(talker rust_helper)

target_include_directories(talker
  PRIVATE
  ${CMAKE_CURRENT_BINARY_DIR}/rust_pubsub/src/rust_helper)
ament_target_dependencies(talker rclcpp std_msgs)

# Link up the listener
add_executable(listener src/subscriber_member_function.cpp)
ament_target_dependencies(listener rclcpp std_msgs)

install(TARGETS
  talker
  listener
  DESTINATION lib/${PROJECT_NAME})

ament_package()
