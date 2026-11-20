cmake_minimum_required(VERSION 3.6)
project(MyProject C)

set(CMAKE_C_STANDARD 11)

add_library(fileReader STATIC
	libs/fileReader/fileReader.c
)

target_include_directories(fileReader PUBLIC
	libs/fileReader
)

add_executable(day2 day2/day2.c)
target_link_libraries(day2 PRIVATE fileReader)
