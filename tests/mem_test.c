#include "rtthread.h"
#include "unity.h"
#include "ci_tests.h";

#define TEST_MEMORY_SIZE 10240

static char test_memory[TEST_MEMORY_SIZE];

#define __TEST_MALLOC_NULL(size) \
    void *pdata = 111111; \
    pdata = rt_malloc(size); \
    TEST_ASSERT_NULL(pdata); \
    rt_free(pdata)

#define __TEST_MALLOC_NOT_NULL(size) \
    void *pdata = RT_NULL; \
    pdata = rt_malloc(size); \
    TEST_ASSERT_NOT_NULL(pdata); \
    rt_free(pdata)

void test_memory_setup(void)
{
   rt_system_heap_init(test_memory, &test_memory[TEST_MEMORY_SIZE-1]);
}

void test_memory_malloc_right(void)
{
   __TEST_MALLOC_NOT_NULL(27);
}

void test_memory_malloc_too_big(void)
{
   __TEST_MALLOC_NULL(10000000000);
}

void test_memory_malloc_zero(void)
{
   __TEST_MALLOC_NULL(0);
}

void test_memory_malloc_not_a_size(void)
{
   __TEST_MALLOC_NULL(-12);
}