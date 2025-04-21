package com.a_dinosaur.kotlinsdl.state

interface State
{
	fun init() {}
	fun free() {}
	fun tick(deltaTime: Float)
	fun draw(deltaTime: Float)
}
