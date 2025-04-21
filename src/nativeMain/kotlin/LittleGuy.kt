package com.a_dinosaur.kotlinsdl

import kotlinx.cinterop.CPointer
import SDL2.SDL_Texture
import kotlin.random.Random

class LittleGuy
{
	private var pos = Vector2f.ZERO
	private var vel = Vector2f.ZERO
	private var flip = false
	private var lazorVec = Vector2f.ZERO
	private var lazor = false

	val guyTexture: CPointer<SDL_Texture>? = Renderer.loadTexture("beato.png")

	fun update()
	{
		val acceleration = 0.25f
		val friction = 0.05f

		if (GamePad.down(GamePad.Button.DPAD_LEFT))
		{
			vel.x -= acceleration
			flip = true
		}
		if (GamePad.down(GamePad.Button.DPAD_RIGHT))
		{
			vel.x += acceleration
			flip = false
		}
		if (GamePad.down(GamePad.Button.DPAD_UP))
			vel.y -= acceleration
		if (GamePad.down(GamePad.Button.DPAD_DOWN))
			vel.y += acceleration

		vel += GamePad.leftStick * acceleration
		if (GamePad.leftStick.x < -0.1f)
			flip = true
		else if (GamePad.leftStick.x > 0.1f)
			flip = false

		val left = -32
		val right = 640 + 32
		val top = -48
		val bottom = 480 + 48

		pos += vel
		if (pos.x < left)
			pos.x += right - left
		if (pos.y < top)
			pos.y += bottom - top
		if (pos.x >= right)
			pos.x -= right - left
		if (pos.y >= bottom)
			pos.y -= bottom - top

		vel -= vel * friction

		val lazorMag = GamePad.rightStick.magnitude
		if (lazorMag > 0.125)
		{
			lazor = if (GamePad.rightTrigger >= 0.5f) {
				lazorVec = GamePad.rightStick / lazorMag
				true } else false
			flip = GamePad.rightStick.x < 0.0f

			if (lazor)
				GamePad.rumble = 1.0f
		}
		else
		{
			lazor = false
			GamePad.rumble = 0.0f
		}
	}

	fun draw()
	{
		val width = 48.0f
		val height = 64.0f
		val dst = Renderer.FRect(
			x = pos.x - width * 0.5f,
			y = pos.y - height * 0.5f,
			w = width,
			h = height)
		val flip = if (flip) Renderer.Flip.Horizontal else Renderer.Flip.None
		val angle = vel.x.toDouble() * 2.45
		guyTexture?.let { Renderer.copy(guyTexture, dst, angle, flip) }

		if (lazor)
		{
			val advance = 10.0f
			val jittor = 8.0f
			val angolJittor = 0.18f

			fun lightning(v1: Vector2f, angle: Float, levels: Int)
			{
				val v2 = v1 + Vector2f.fromAngle(angle) * advance +
					Vector2f(
						Random.nextFloat() * jittor - jittor * 0.5f,
						Random.nextFloat() * jittor - jittor * 0.5f)

				val bright = Random.nextInt(0x7F)
				val colour = Renderer.Colour(
					(0x7F + bright).toUByte(),
					(0x7F + bright).toUByte(),
					0xFFu, 0xFFu)
				Renderer.setDrawColour(colour)
				Renderer.line(v1, v2)

				if (levels > 1)
				{
					val rando = angle + Random.nextFloat() * angolJittor - angolJittor * 0.5f + 0.01f
					lightning(v2, rando, levels - if (Random.nextInt(3) == 0) 2 else 1)
					if (Random.nextInt(14) == 0)
					{
						val rando2 = angle - Random.nextFloat() * angolJittor - angolJittor * 0.5f
						lightning(v2, rando2, levels - 1)
					}
				}
			}

			lightning(pos, lazorVec.angle, 50)
		}
	}
}
