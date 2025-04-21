@file:OptIn(ExperimentalForeignApi::class, ExperimentalForeignApi::class)

package com.a_dinosaur.kotlinsdl

import kotlinx.cinterop.*
import io.karma.sdl.*
import com.a_dinosaur.kotlinsdl.state.*
import kotlin.native.concurrent.ThreadLocal

@ThreadLocal
object Application
{
	var window: CPointer<cnames.structs.SDL_Window>? = null
	var sdlPad: CPointer<cnames.structs.SDL_Gamepad>? = null
	var joyId: SDL_JoystickID = 0u

	var running = true

	lateinit var state: State

	fun changeState(newState: State)
	{
		//TODO: This should probably be delayed until the next tick lol
		state.free()
		state = newState
		state.init()
	}

	private fun openController(index: SDL_JoystickID): Boolean
	{
		SDL_OpenGamepad(index)?.let {
			sdlPad = it
			joyId = index
		}
		return sdlPad?.let {
			println("Using gamepad #$joyId, \"${SDL_GetGamepadName(sdlPad)?.toKString()}\"")
			true
		} ?: false
	}

	private fun init()
	{
		if (!SDL_Init(SDL_INIT_VIDEO or SDL_INIT_GAMEPAD))
			throw Error("SDL_Init returned failure")

		val windowFlags = SDL_WINDOW_HIGH_PIXEL_DENSITY or SDL_WINDOW_RESIZABLE
		window = SDL_CreateWindow("Pissing in kotlin", 640, 480, windowFlags)
		window ?: throw Error("SDL_CreateWindow returned ${SDL_GetError()}")

		Renderer.init(window!!, true)

		state = SplashState()
		state.init()
	}

	private fun onEvent(e: SDL_Event)
	{
		when (e.type)
		{
			SDL_EVENT_QUIT -> running = false
			SDL_EVENT_KEY_DOWN -> when (e.key.key)
			{
				SDLK_ESCAPE -> running = false

				//SDLK_UP -> input = input or LittleGuy.UP
				//SDLK_DOWN -> input = input or LittleGuy.DOWN
				//SDLK_LEFT -> input = input or LittleGuy.LEFT
				//SDLK_RIGHT -> input = input or LittleGuy.RIGHT
			}
			SDL_EVENT_KEY_UP -> when (e.key.key)
			{
				//SDLK_UP -> input = input and LittleGuy.UP.inv()
				//SDLK_DOWN -> input = input and LittleGuy.DOWN.inv()
				//SDLK_LEFT -> input = input and LittleGuy.LEFT.inv()
				//SDLK_RIGHT -> input = input and LittleGuy.RIGHT.inv()
			}
			SDL_EVENT_GAMEPAD_ADDED ->
				if (sdlPad == null && SDL_IsGamepad(e.cdevice.which))
					openController(e.cdevice.which)
			SDL_EVENT_GAMEPAD_REMOVED ->
				if (e.cdevice.which == joyId)
				{
					SDL_CloseGamepad(sdlPad)
					sdlPad = null
					joyId = 0u
				}
			SDL_EVENT_GAMEPAD_BUTTON_DOWN -> if (e.gbutton.which == joyId) when (e.gbutton.button.toInt())
			{
				SDL_GAMEPAD_BUTTON_SOUTH -> GamePad.setButtonPressed(GamePad.Button.A)
				SDL_GAMEPAD_BUTTON_EAST -> GamePad.setButtonPressed(GamePad.Button.B)
				SDL_GAMEPAD_BUTTON_WEST -> GamePad.setButtonPressed(GamePad.Button.X)
				SDL_GAMEPAD_BUTTON_NORTH -> GamePad.setButtonPressed(GamePad.Button.Y)
				SDL_GAMEPAD_BUTTON_LEFT_SHOULDER -> GamePad.setButtonPressed(GamePad.Button.LEFT_SHOULDER)
				SDL_GAMEPAD_BUTTON_LEFT_STICK -> GamePad.setButtonPressed(GamePad.Button.LEFT_STICK)
				SDL_GAMEPAD_BUTTON_RIGHT_SHOULDER -> GamePad.setButtonPressed(GamePad.Button.RIGHT_SHOULDER)
				SDL_GAMEPAD_BUTTON_RIGHT_STICK -> GamePad.setButtonPressed(GamePad.Button.RIGHT_STICK)
				SDL_GAMEPAD_BUTTON_DPAD_UP -> GamePad.setButtonPressed(GamePad.Button.DPAD_UP)
				SDL_GAMEPAD_BUTTON_DPAD_DOWN -> GamePad.setButtonPressed(GamePad.Button.DPAD_DOWN)
				SDL_GAMEPAD_BUTTON_DPAD_LEFT -> GamePad.setButtonPressed(GamePad.Button.DPAD_LEFT)
				SDL_GAMEPAD_BUTTON_DPAD_RIGHT -> GamePad.setButtonPressed(GamePad.Button.DPAD_RIGHT)
				SDL_GAMEPAD_BUTTON_START -> GamePad.setButtonPressed(GamePad.Button.START)
				SDL_GAMEPAD_BUTTON_BACK -> GamePad.setButtonPressed(GamePad.Button.BACK)
				SDL_GAMEPAD_BUTTON_GUIDE -> GamePad.setButtonPressed(GamePad.Button.GUIDE)
				SDL_GAMEPAD_BUTTON_MISC1 -> GamePad.setButtonPressed(GamePad.Button.MISC)
			}
			SDL_EVENT_GAMEPAD_BUTTON_UP -> if (e.gbutton.which == joyId) when (e.gbutton.button.toInt())
			{
				SDL_GAMEPAD_BUTTON_SOUTH -> GamePad.setButtonReleased(GamePad.Button.A)
				SDL_GAMEPAD_BUTTON_EAST -> GamePad.setButtonReleased(GamePad.Button.B)
				SDL_GAMEPAD_BUTTON_WEST -> GamePad.setButtonReleased(GamePad.Button.X)
				SDL_GAMEPAD_BUTTON_NORTH -> GamePad.setButtonReleased(GamePad.Button.Y)
				SDL_GAMEPAD_BUTTON_LEFT_SHOULDER -> GamePad.setButtonReleased(GamePad.Button.LEFT_SHOULDER)
				SDL_GAMEPAD_BUTTON_LEFT_STICK -> GamePad.setButtonReleased(GamePad.Button.LEFT_STICK)
				SDL_GAMEPAD_BUTTON_RIGHT_SHOULDER -> GamePad.setButtonReleased(GamePad.Button.RIGHT_SHOULDER)
				SDL_GAMEPAD_BUTTON_RIGHT_STICK -> GamePad.setButtonReleased(GamePad.Button.RIGHT_STICK)
				SDL_GAMEPAD_BUTTON_DPAD_UP -> GamePad.setButtonReleased(GamePad.Button.DPAD_UP)
				SDL_GAMEPAD_BUTTON_DPAD_DOWN -> GamePad.setButtonReleased(GamePad.Button.DPAD_DOWN)
				SDL_GAMEPAD_BUTTON_DPAD_LEFT -> GamePad.setButtonReleased(GamePad.Button.DPAD_LEFT)
				SDL_GAMEPAD_BUTTON_DPAD_RIGHT -> GamePad.setButtonReleased(GamePad.Button.DPAD_RIGHT)
				SDL_GAMEPAD_BUTTON_START -> GamePad.setButtonReleased(GamePad.Button.START)
				SDL_GAMEPAD_BUTTON_BACK -> GamePad.setButtonReleased(GamePad.Button.BACK)
				SDL_GAMEPAD_BUTTON_GUIDE -> GamePad.setButtonReleased(GamePad.Button.GUIDE)
				SDL_GAMEPAD_BUTTON_MISC1 -> GamePad.setButtonReleased(GamePad.Button.MISC)
			}
			SDL_EVENT_GAMEPAD_AXIS_MOTION -> if (e.gaxis.which == joyId) when (e.gaxis.axis.toInt())
			{
				SDL_GAMEPAD_AXIS_LEFTX -> GamePad.rawStickLeftX = e.gaxis.value.toFloat() / 0x7FFF
				SDL_GAMEPAD_AXIS_LEFTY -> GamePad.rawStickLeftY = e.gaxis.value.toFloat() / 0x7FFF
				SDL_GAMEPAD_AXIS_RIGHTX -> GamePad.rawStickRightX = e.gaxis.value.toFloat() / 0x7FFF
				SDL_GAMEPAD_AXIS_RIGHTY -> GamePad.rawStickRightY = e.gaxis.value.toFloat() / 0x7FFF
				SDL_GAMEPAD_AXIS_LEFT_TRIGGER -> GamePad.rawTriggerLeft = e.gaxis.value.toFloat() / 0x7FFF
				SDL_GAMEPAD_AXIS_RIGHT_TRIGGER -> GamePad.rawTriggerRight = e.gaxis.value.toFloat() / 0x7FFF
			}
		}
	}

	private fun tick()
	{
		GamePad.tick()
		state.tick(1.0f / 120.0f)
	}

	private fun draw()
	{
		state.draw(1.0f / 120.0f)
		Renderer.present()
	}

	private fun free()
	{
		state.free()
		Renderer.free()
		SDL_DestroyWindow(window)
		SDL_Quit()
	}

	fun run()
	{
		try
		{
			init()
			SDL_RumbleGamepad(sdlPad, 0xFFFFu, 0xFFFFu, 1000u)
			memScoped {
				val event = alloc<SDL_Event>()
				while (running)
				{
					while (SDL_PollEvent(event.ptr))
						onEvent(event)
					tick()
					draw()
					//SDL_Delay(100u)
				}
			}
		}
		finally
		{
			free()
		}
	}
}
