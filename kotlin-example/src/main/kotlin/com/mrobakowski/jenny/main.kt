package com.mrobakowski.jenny

fun main(args: Array<String>) {
    HelloWorld.helloWorld()
    println("Result of expensive ${HelloWorld.bestLangName()} computation: ${HelloWorld.foo(1, 2.0f)}")
    println(""""rust is fun" contains `rust`: ${HelloWorld.containsRust("rust is fun")}""")
}
