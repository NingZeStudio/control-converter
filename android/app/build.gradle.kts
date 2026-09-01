plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("org.jetbrains.kotlin.plugin.compose")
}

android {
    namespace = "com.zhizhu.controlconverter"
    compileSdk = 37
    buildToolsVersion = "37.0.0"

    defaultConfig {
        applicationId = "com.zhizhu.controlconverter"
        minSdk = 26
        targetSdk = 36
        versionCode = 3
        ndk {
            abiFilters += "arm64-v8a"
        }
        versionName = "2.0"
    }

    // 签名配置：从环境变量读取（CI 用 GitHub Secret 注入），或使用本地
    // android/release.keystore（不入库，见 .gitignore）。凭据不齐全时
    // 不挂接签名，assembleRelease 产出 unsigned APK，避免把密钥写进仓库。
    val ksPath = System.getenv("KEYSTORE_PATH")
    val ksPass = System.getenv("KEYSTORE_PASSWORD")
    val keyAlias = System.getenv("KEY_ALIAS")
    val keyPass = System.getenv("KEY_PASSWORD")
    val ksFile = if (!ksPath.isNullOrBlank()) file(ksPath) else rootProject.file("release.keystore")
    val releaseSigningReady =
        !ksPass.isNullOrBlank() && !keyAlias.isNullOrBlank() && !keyPass.isNullOrBlank() && ksFile.exists()

    signingConfigs {
        if (releaseSigningReady) {
            create("release") {
                storeFile = ksFile
                storePassword = ksPass
                this.keyAlias = keyAlias
                keyPassword = keyPass
            }
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = true
            isShrinkResources = true
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
            if (releaseSigningReady) {
                signingConfig = signingConfigs.getByName("release")
            }
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    buildFeatures {
        compose = true
    }
}

kotlin {
    compilerOptions {
        jvmTarget = org.jetbrains.kotlin.gradle.dsl.JvmTarget.JVM_17
    }
}

dependencies {
    implementation("top.yukonga.miuix.kmp:miuix-ui-android:0.9.3")
    implementation("top.yukonga.miuix.kmp:miuix-preference-android:0.9.3")
    implementation(platform("androidx.compose:compose-bom:2025.08.00"))
    implementation("androidx.activity:activity-compose:1.10.0")
    implementation("androidx.core:core-ktx:1.16.0")
    implementation("androidx.compose.ui:ui")
    implementation("androidx.compose.ui:ui-tooling-preview")
    implementation("androidx.compose.foundation:foundation")
    implementation("androidx.compose.material3:material3")
    implementation("io.github.kyant0:backdrop:2.0.1")
    debugImplementation("androidx.compose.ui:ui-tooling")
}
