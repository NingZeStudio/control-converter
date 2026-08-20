package com.tungsten.fcl.util;

/**
 * JNI bridge to the Go implementation (go/main.go).
 *
 * The native symbol name is fixed by the Go c-shared build:
 *   Java_com_tungsten_fcl_util_LayoutConverter_convertFclToZl2Native
 *
 * The .so must be placed at jniLibs/arm64-v8a/libcc.so and is loaded
 * with System.loadLibrary("cc").
 */
public final class LayoutConverter {

    static {
        System.loadLibrary("cc");
    }

    private LayoutConverter() {
    }

    /**
     * Native entry: convert an FCL control layout JSON to a ZL2 layout JSON.
     *
     * @param inputPath  absolute path of the FCL layout JSON file
     * @param outputPath absolute path where the ZL2 JSON will be written
     * @return null on success, or an error message on failure
     */
    public static native String convertFclToZl2Native(String inputPath, String outputPath);

    /**
     * Convenience wrapper that throws on failure.
     */
    public static void convertFclToZl2(String inputPath, String outputPath) throws Exception {
        String error = convertFclToZl2Native(inputPath, outputPath);
        if (error != null) {
            throw new RuntimeException("JNI conversion failed: " + error);
        }
    }
}
