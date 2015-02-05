cmake_minimum_required(VERSION 2.8.3)
project(ros_rust)

catkin_package()

if(CATKIN_ENABLE_TESTING)
    find_package(rostest REQUIRED)
    add_rostest(test/empty.test)
endif()
