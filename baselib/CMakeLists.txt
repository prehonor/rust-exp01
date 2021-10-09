cmake_minimum_required(VERSION 3.19)
project(bookmarks)

set(CMAKE_CXX_STANDARD 14)

set(SOURCES_IN
        ffi/sources/testlib_01.cpp
        )
add_library(others STATIC ${SOURCES_IN})
set(SOURCES
        ffi/library.cpp
        ffi/sources/testlib_01.cpp
        )

add_library(${PROJECT_NAME} STATIC ${SOURCES}) # SHARED（共享库） STATIC（静态库）
target_link_libraries(${PROJECT_NAME} others)

set(INC_DIR ${PROJECT_SOURCE_DIR}/ffi/sources)
set(CMAKE_INSTALL_PREFIX ${PROJECT_SOURCE_DIR})
include_directories(${INC_DIR})
MESSAGE( STATUS "this var key = ${CMAKE_INSTALL_PREFIX}.")

install (TARGETS ${PROJECT_NAME}
        ARCHIVE DESTINATION cglib)   # LIBRARY（共享库） ARCHIVE（静态库）

install(DIRECTORY ${PROJECT_SOURCE_DIR}/ffi/
        DESTINATION cglib
        FILES_MATCHING
        REGEX "\\.h$"
        REGEX "(sources)" EXCLUDE
       )