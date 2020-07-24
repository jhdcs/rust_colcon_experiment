cmake_minimum_required(VERSION 3.5)
project(rust_helper)

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD cargo build --release)
    set(TARGET_DIR "release")
endif ()

set(CLIENT_SO "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/rust_helper.so")

add_custom_command(OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.h
	COMMAND cxxbridge ${CMAKE_CURRENT_SOURCE_DIR}/src/lib.rs --header > ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.h
	DEPENDS src/lib.rs)

add_custom_command(OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.cc
	COMMAND cxxbridge ${CMAKE_CURRENT_SOURCE_DIR}/src/lib.rs > ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.cc
	DEPENDS src/lib.rs)

add_custom_target(client ALL
    COMMENT "Compiling client module"
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD} 
    COMMAND cp ${CLIENT_SO} ${CMAKE_CURRENT_BINARY_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
set_target_properties(client PROPERTIES LOCATION ${CMAKE_CURRENT_BINARY_DIR})

add_test(NAME client_test 
    COMMAND cargo test
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})