@file:OptIn(ExperimentalForeignApi::class, ExperimentalForeignApi::class)

package com.a_dinosaur.kotlinsdl

import kotlinx.cinterop.CPointer
import com.a_dinosaur.kotlinsdl.maths.Vec2
import io.karma.sdl.SDL_Texture
import kotlinx.cinterop.ExperimentalForeignApi
import kotlin.random.Random

class LittleGuy
{
	private var pos = Vec2.ZERO
	private var vel = Vec2.ZERO
	private var flip = false
	private var lazorVec = Vec2.ZERO
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
		val dst = Renderer.Rect(
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

			fun lightning(v1: Vec2, angle: Float, levels: Int)
			{
				val v2 = v1 + Vec2.fromAngle(angle) * advance +
					Vec2(
						Random.nextFloat() * jittor - jittor * 0.5f,
						Random.nextFloat() * jittor - jittor * 0.5f)

				val bright = Random.nextInt(0x7F)
				val colour = Renderer.Colour(
					(0x7F + bright).toUByte(),
					(0x7F + bright).toUByte(),
					0xFFu, 0xFFu)
				Renderer.setDrawColour(colour)
				Renderer.line(v1, v2)

				fun pMod(x: Float, d: Float) = (x % d + d) % d

				val v3 = Vec2(
					pMod(v2.x + 32.0f, 640.0f + 64.0f) - 32.0f,
					pMod(v2.y + 48.0f, 480.0f + 96.0f) - 48.0f)

				if (levels > 1)
				{
					val rando = angle + Random.nextFloat() * angolJittor - angolJittor * 0.5f + 0.01f
					lightning(v3, rando, levels - if (Random.nextInt(3) == 0) 2 else 1)
					if (Random.nextInt(14) == 0)
					{
						val rando2 = angle - Random.nextFloat() * angolJittor - angolJittor * 0.5f
						lightning(v3, rando2, levels - 1)
					}
				}
			}

			lightning(pos, lazorVec.angle, 50)
		}
	}
}
