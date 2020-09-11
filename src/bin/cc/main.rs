extern crate libc;
extern crate libloading;

use libc::*;
use libloading::{Library, Symbol};

/*
// test_c.dll 接口内容

SDK_API void test_normal(int a, float b, const char* c){
    printf("a: %d, b: %.2f, c: %s\n", a, b, c);
}

SDK_API void test_p(int* a, float* b, char* c){
    printf("set *a=112233, *b=3.1415926, c='1234567890'\n");
    *a = 112233;
    *b = 3.1415926f;
    strcpy(c, "1234567890");
}

typedef struct _TEST_OBJ
{
    int a;
    float* b;
    char c[256];
}TEST_OBJ;

SDK_API void test_t(TEST_OBJ* arg){
    arg->a = 222;
    arg->b = NULL;
    strcpy(arg->c, u8"hello, world! 你好！~");
}
typedef  int(*CB_FUN)(int a);

SDK_API int test_cb(CB_FUN p){
    printf("cb: %X, call p(10)\n", p);
    int a = p(10);
    return a;
}
*/

type fn_test_normal = unsafe fn(c_int, c_float, &str);
type fn_test_p = unsafe fn(&c_int, &c_float, *mut c_char);

#[repr(C)]
#[derive(Clone, Copy)]
struct fn_struct_t {
    a: c_int,
    b: *mut c_float,
    c: [u8; 256],
}

type fn_test_t = unsafe fn(&mut fn_struct_t);

type fn_test_cb = unsafe fn( fn(i32)->i32 ) -> i32;

fn main() {
    let lib = Library::new("test_c.dll").unwrap();
    unsafe {
        let fun1: Symbol<fn_test_normal> = lib.get(b"test_normal").unwrap();
        fun1(1, 2.21, "Hello,world\0"); // rust 字符串没有结尾的\0,
    };

    println!();
    unsafe {
        let mut arg1 = 0i32;
        let mut arg2 = 0f32;
        let mut arg3 = [0u8; 256];  // 分配存储空间
        let fun2: Symbol<fn_test_p> = lib.get(b"test_p").unwrap();
        fun2(&arg1, &arg2, arg3.as_mut_ptr() as *mut i8); // set *a=112233, *b=3.1415926, c='1234567890'
        println!("{} {} {:?}", arg1, arg2, arg3[0]); // 112233 3.1415925 49
//        let s = String::from_raw_parts(arg3.as_mut_ptr() as *mut u8, strlen(arg3.as_ptr()), arg3.len());
//        println!("ret: {}", s); // ret: 1234567890, 有buf, 会导致下面的无输出，故使用3的方式构建字符串
        let sv = &arg3[0..strlen(arg3.as_ptr() as *const i8)]; // 通过strlen构造对应字符串的数组
        let s = String::from_utf8_unchecked(sv.to_vec()); // 字符串使用utf8编码的u8
        println!("ret: {}", s); // ret: 1234567890
    };

    println!();
    unsafe {
        let mut arg = fn_struct_t { a: 0, b: 0 as *mut f32, c: [0u8; 256] };
        let fun3: Symbol<fn_test_t> = lib.get(b"test_t").unwrap();
        fun3(&mut arg);
        println!("{} {:?} {:?}", arg.a, arg.b, arg.c[0]); // 222 0x0 104
        let sv = &arg.c[0..strlen(arg.c.as_ptr() as *const i8)]; // 通过strlen构造对应字符串的数组
        let s = String::from_utf8_unchecked(sv.to_vec()); // 字符串使用utf8编码的u8
        println!("ret: {}", s); // ret: hello, world! 你好！~
    };

    println!();
    unsafe {
        let arg = |arg:i32| {
            println!("arg cb called! arg is {}", arg);
            arg*10+123
        };
        let fun4: Symbol<fn_test_cb> = lib.get(b"test_cb").unwrap();
        let r = fun4(arg);
        println!("ret: {}", r); // 223
    };
}