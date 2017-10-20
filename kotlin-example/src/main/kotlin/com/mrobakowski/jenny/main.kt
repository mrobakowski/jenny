package com.mrobakowski.jenny

fun main(args: Array<String>) {
    HelloWorld.helloWorld()
    println("Result of expensive native computation: ${HelloWorld.foo(1, 2.0f)}")
}
