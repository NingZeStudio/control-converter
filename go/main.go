package main

/*
#cgo LDFLAGS: -llog
#include <jni.h>
#include <stdlib.h>
#include <string.h>

// C wrappers for JNI function-pointer table calls (cgo cannot resolve them directly)
static const char* jni_get_string_utf_chars(JNIEnv *env, jstring s) {
    return (*env)->GetStringUTFChars(env, s, NULL);
}
static void jni_release_string_utf_chars(JNIEnv *env, jstring s, const char *cstr) {
    (*env)->ReleaseStringUTFChars(env, s, cstr);
}
static jstring jni_new_string_utf(JNIEnv *env, const char *cstr) {
    return (*env)->NewStringUTF(env, cstr);
}
*/
import "C"

import (
	"fmt"
	"math"
	"os"
	"path/filepath"
	"strconv"
	"unsafe"
)

// osReadFile reads a file's contents.
func osReadFile(path string) ([]byte, error) {
	return os.ReadFile(path)
}

// osWriteFile writes data to a file, creating parent directories if needed.
func osWriteFile(path string, data []byte, perm os.FileMode) error {
	dir := filepath.Dir(path)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(path, data, perm)
}

// osStderr returns os.Stderr.
func osStderr() *os.File {
	return os.Stderr
}

// parseAspect parses an aspect ratio string like "16/9" or "1.778".
func parseAspect(s string) (float64, error) {
	if idx := indexByte(s, '/'); idx >= 0 {
		num, err := strconv.ParseFloat(s[:idx], 64)
		if err != nil {
			return 0, err
		}
		den, err := strconv.ParseFloat(s[idx+1:], 64)
		if err != nil {
			return 0, err
		}
		if den == 0 {
			return 0, fmt.Errorf("denominator is zero")
		}
		return num / den, nil
	}
	return strconv.ParseFloat(s, 64)
}

func indexByte(s string, c byte) int {
	for i := 0; i < len(s); i++ {
		if s[i] == c {
			return i
		}
	}
	return -1
}

// main is required by c-shared build mode but never called.
func main() {}

// --- JNI helper functions ---

// jstringToGo converts a JNI jstring to a Go string.
func jstringToGo(env *C.JNIEnv, s C.jstring) string {
	cstr := C.jni_get_string_utf_chars(env, s)
	if cstr == nil {
		return ""
	}
	defer C.jni_release_string_utf_chars(env, s, cstr)
	return C.GoString(cstr)
}

// goToJstring converts a Go string to a JNI jstring.
func goToJstring(env *C.JNIEnv, s string) C.jstring {
	cstr := C.CString(s)
	defer C.free(unsafe.Pointer(cstr))
	return C.jni_new_string_utf(env, cstr)
}

// Java_com_tungsten_fcl_util_LayoutConverter_convertFclToZl2Native is the JNI
// entry point called from LayoutConverter.java.
//
// Parameters:
//   - inputPath:  FCL control layout JSON file path
//   - outputPath: output ZL2 JSON file path
//
// Returns:
//   - nil on success
//   - error message jstring on failure
//
//export Java_com_tungsten_fcl_util_LayoutConverter_convertFclToZl2Native
func Java_com_tungsten_fcl_util_LayoutConverter_convertFclToZl2Native(env *C.JNIEnv, clazz C.jclass, inputPath C.jstring, outputPath C.jstring) C.jstring {
	inputGo := jstringToGo(env, inputPath)
	outputGo := jstringToGo(env, outputPath)

	if inputGo == "" || outputGo == "" {
		return goToJstring(env, "input and output paths are required")
	}

	// Read input file
	source, err := loadJSONFile(inputGo)
	if err != nil {
		return goToJstring(env, "failed to read input file: "+err.Error())
	}

	// Reset global state for each conversion
	warnedMessages = map[string]struct{}{}
	substitutionCounts = map[string]int{"keys": 0, "events": 0, "layers": 0, "directions": 0}

	// Convert: lossless mode, default 16:9 aspect ratio
	result := convertFCLToZL(source, false, false, 16.0/9.0, true, false)

	// Write output file
	if err := writeJSONFile(outputGo, result, false); err != nil {
		return goToJstring(env, "failed to write output file: "+err.Error())
	}

	var nilRet C.jstring // zero value = NULL jstring, signals success to Java
	return nilRet
}

// Ensure math is used (aspect ratio validation in CLI was removed)
var _ = math.Pi
