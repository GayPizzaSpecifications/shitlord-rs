@file:OptIn(ExperimentalForeignApi::class, ExperimentalForeignApi::class)

package com.a_dinosaur.kotlinsdl.state

import com.a_dinosaur.kotlinsdl.Application
import com.a_dinosaur.kotlinsdl.Renderer
import kotlinx.cinterop.ExperimentalForeignApi
import kotlin.math.*

class SplashState: State
{
	val backgroundTexture = Renderer.loadTexture("gamepad.jpeg")
	var time = 0.0f
	var fade = 1.0f

	override fun tick(deltaTime: Float)
	{
		time += deltaTime
		if (time < 2.0f)
			fade = max(fade - 0.75f * deltaTime, 0.0f)
		else if (time < 3.2f)
			fade = min(fade + 0.85f * deltaTime, 1.0f)
		else
			Application.changeState(GameState())
	}

	override fun draw(deltaTime: Float)
	{
		Renderer.setDrawColour(Renderer.Colour.BLACK)
		Renderer.clear()
		backgroundTexture?.let { Renderer.copy(it) }
		Renderer.setBlendMode(Renderer.BlendMode.Blend)
		val alpha = (fade * 255.0f).toUInt().toUByte()
		Renderer.setDrawColour(Renderer.Colour(0x00u, 0x00u, 0x00u, alpha))
		Renderer.boxFill(Renderer.Rect(0f, 0f, 640f, 480f))
	}
}
