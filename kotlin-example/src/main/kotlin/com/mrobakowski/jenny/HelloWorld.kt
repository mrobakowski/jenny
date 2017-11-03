package com.mrobakowski.jenny

object HelloWorld {
    init {
        System.load("${System.getProperty("user.dir")}/target/debug/examples/${platformSpecificLibName("hello-world")}")
    }

    external fun helloWorld()
    external fun foo(x: Long, y: Float): Double
    external fun bestLangName(): String
    external fun containsRust(s: String): Boolean
}

fun platformSpecificLibName(name: String): String {
    val platform = System.getProperty("os.name").toLowerCase()
    return when {
        "win" in platform -> "$name.dll"
        "nix" in platform || "nux" in platform -> "lib$name.so"
        else -> "<unknown platform>"
    }
}