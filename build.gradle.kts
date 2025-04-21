plugins {
	kotlin("multiplatform") version "2.1.20"
}

group = "com.a_dinosaur"
version = "1.0-SNAPSHOT"

repositories {
	mavenCentral()
	maven("https://git.karmakrafts.dev/api/v4/projects/336/packages/maven")
}

java {
	toolchain {
		languageVersion = JavaLanguageVersion.of(17)
	}
	sourceCompatibility = JavaVersion.VERSION_17
	targetCompatibility = JavaVersion.VERSION_17
}

kotlin {
    applyDefaultHierarchyTemplate()
	val hostOs = System.getProperty("os.name")
	val arch = System.getProperty("os.arch")
	val isMingwX64 = hostOs.startsWith("Windows")
	val nativeTarget = when {
		hostOs == "Mac OS X" -> when (arch) {
			"aarch64" -> macosArm64()
			"amd64" -> macosX64()
			else -> throw GradleException("Unsupported MacOS architecture.")
		}
		hostOs == "Linux" -> linuxX64()
		isMingwX64 -> mingwX64()
		else -> throw GradleException("Host OS is not supported in Kotlin/Native.")
	}

	nativeTarget.apply {
		binaries {
			executable {
				entryPoint = "main"
			}
		}
	}

	sourceSets {
		nativeMain {
			dependencies {
				implementation("io.karma.sdl:multiplatform-sdl:2.0.1.12")
			}
		}
	}
}
