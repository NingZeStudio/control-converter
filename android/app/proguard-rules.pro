# Keep the JNI bridge class and its native methods.
-keep class com.tungsten.fcl.util.LayoutConverter {
    public static native java.lang.String convertFclToZl2Native(java.lang.String, java.lang.String);
}
