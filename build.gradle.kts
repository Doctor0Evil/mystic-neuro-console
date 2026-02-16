plugins {
    kotlin("jvm") version "1.9.22"
    application
}

repositories {
    mavenCentral()
}

dependencies {
    implementation(kotlin("stdlib"))
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.8.1")
    implementation("org.java-websocket:Java-WebSocket:1.5.6")
    implementation("com.fasterxml.jackson.module:jackson-module-kotlin:2.17.1")
}

application {
    mainClass.set("MainKt")
}
