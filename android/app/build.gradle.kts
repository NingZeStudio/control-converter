plugins {
    id("com.android.application")
}

android {
    namespace = "com.tungsten.fcl.jnitest"
    compileSdk = 37

    defaultConfig {
        applicationId = "com.tungsten.fcl.jnitest"
        // libcc.so is built with aarch64-linux-android21, so API 21 is the floor.
        minSdk = 21
        targetSdk = 37
        versionCode = 1
        versionName = "1.0"

        ndk {
            // Only the aarch64 JNI library is shipped (dist/libcc.so).
            abiFilters += "arm64-v8a"
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
}
