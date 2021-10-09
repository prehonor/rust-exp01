#ifndef BMLIB_LIBRARY_H
#define BMLIB_LIBRARY_H
#include <bzlib.h>

/**
 * 该文件主要是适配 c++库
 * */
#ifdef __cplusplus
extern "C" {
#endif

int hello();

#ifdef __cplusplus
}
#endif
#endif //BMLIB_LIBRARY_H
