package com.mrobakowski.jenny

object HelloWorld {
    init {
        System.load("${System.getProperty("user.dir")}/target/debug/examples/hello-world.dll")
    }

    external fun helloWorld()
    external fun foo(x: Long, y: Float): Double
}