package com.a_dinosaur.kotlinsdl.state

import com.a_dinosaur.kotlinsdl.*

class GameState: State
{
	val guy = LittleGuy()

	override fun tick(deltaTime: Float)
	{
		guy.update()
	}

	override fun draw(deltaTime: Float)
	{
		Renderer.setDrawColour(Renderer.Colour(0x1F1F1FFFu))
		Renderer.clear()

		guy.draw()
	}
}
