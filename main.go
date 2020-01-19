package main

/*
#cgo LDFLAGS: -L./hyper -lhyper
#include "./lib/hyper_translate.h"
*/
import "C"

func main() {
	C.space_find(C.CString("John Smith"))
}
