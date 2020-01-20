package main

/*
#cgo LDFLAGS: -L./lib -lhyper
#include "./lib/hyper.h"
*/
import "C"

import (
	"fmt"
)

func main() {
	ret := C.space_find_export(C.CString("hello world"))
	fmt.Println(C.GoString(ret.left))
	fmt.Println(C.GoString(ret.right))
}
