plugins {
	kotlin("multiplatform") version "1.7.20"
}

group = "com.a_dinosaur"
version = "1.0-SNAPSHOT"

repositories {
	mavenCentral()
}

kotlin {
	val hostOs = System.getProperty("os.name")
	val arch = System.getProperty("os.arch")
	val isMingwX64 = hostOs.startsWith("Windows")
	val nativeTarget = when {
		hostOs == "Mac OS X" -> when (arch) {
			"aarch64" -> macosArm64("native")
			"amd64" -> macosX64("native")
			else -> throw GradleException("Unsupported MacOS architecture.")
		}
		hostOs == "Linux" -> linuxX64("native")
		isMingwX64 -> mingwX64("native")
		else -> throw GradleException("Host OS is not supported in Kotlin/Native.")
	}

	nativeTarget.apply {
		binaries {
			executable {
				entryPoint = "main"
			}
		}

		compilations.getByName("main").cinterops {
			val SDL2 by creating
			val SDL2_image by creating
		}
	}
	sourceSets {
		val nativeMain by getting {
			//dependencies {
			//	val korimVersion: String by project
			//	implementation("com.soywiz.korlibs.korim:korim:$korimVersion")
			//}
		}
	}
}
