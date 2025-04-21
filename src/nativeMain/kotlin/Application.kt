package com.a_dinosaur.kotlinsdl

import kotlinx.cinterop.*
import SDL2.*
import SDL2.SDL_TRUE
import SDL2_image.*
import com.a_dinosaur.kotlinsdl.state.*

@ThreadLocal
object Application
{
	var window: CPointer<SDL_Window>? = null
	var sdlPad: CPointer<SDL_GameController>? = null
	var joyId: Int = 0

	var running = true

	lateinit var state: State

	fun changeState(newState: State)
	{
		//TODO: This should probably be delayed until the next tick lol
		state.free()
		state = newState
		state.init()
	}

	private fun openController(index: Int): Boolean
	{
		SDL_GameControllerOpen(index)?.let {
			sdlPad = it
			joyId = SDL_JoystickGetDeviceInstanceID(index)
		}
		return sdlPad?.let {
			println("Using gamepad #$joyId, \"${SDL_GameControllerName(sdlPad)?.toKString()}\"")
			true
		} ?: false
	}

	private fun init()
	{
		if (SDL_Init(SDL_INIT_VIDEO or SDL_INIT_GAMECONTROLLER) < 0)
			throw Error("SDL_Init returned failure")

		if (IMG_Init(IMG_INIT_PNG.toInt() or IMG_INIT_JPG.toInt()) < 0)
			throw Error("IMG_Init returned whatever")
			//throw Error("IMG_Init returned ${IMG_GetError()?.toKString()}")


		val windowFlags = SDL_WINDOW_ALLOW_HIGHDPI or SDL_WINDOW_RESIZABLE
		window = SDL_CreateWindow("Pissing in kotlin",
			SDL_WINDOWPOS_UNDEFINED.toInt(),
			SDL_WINDOWPOS_UNDEFINED.toInt(),
			640, 480, windowFlags)
		window ?: throw Error("SDL_CreateWindow returned ${SDL_GetError()}")

		SDL_SetHint(SDL_HINT_RENDER_LOGICAL_SIZE_MODE, "letterbox")
		SDL_SetHint(SDL_HINT_RENDER_SCALE_QUALITY, "linear")
		Renderer.init(window!!, SDL_RENDERER_PRESENTVSYNC)

		for (i in 0..SDL_NumJoysticks())
			if (SDL_IsGameController(i) == SDL_TRUE)
				openController(i)

		state = SplashState()
		state.init()
	}

	private fun onEvent(e: SDL_Event)
	{
		when (e.type)
		{
			SDL_QUIT -> running = false
			SDL_KEYDOWN -> when (e.key.keysym.sym.toUInt())
			{
				SDLK_ESCAPE -> running = false

				//SDLK_UP -> input = input or LittleGuy.UP
				//SDLK_DOWN -> input = input or LittleGuy.DOWN
				//SDLK_LEFT -> input = input or LittleGuy.LEFT
				//SDLK_RIGHT -> input = input or LittleGuy.RIGHT
			}
			SDL_KEYUP -> when (e.key.keysym.sym.toUInt())
			{
				//SDLK_UP -> input = input and LittleGuy.UP.inv()
				//SDLK_DOWN -> input = input and LittleGuy.DOWN.inv()
				//SDLK_LEFT -> input = input and LittleGuy.LEFT.inv()
				//SDLK_RIGHT -> input = input and LittleGuy.RIGHT.inv()
			}
			SDL_CONTROLLERDEVICEADDED ->
				if (sdlPad == null && SDL_IsGameController(e.cdevice.which) == SDL_TRUE)
					openController(e.cdevice.which)
			SDL_CONTROLLERDEVICEREMOVED ->
				if (e.cdevice.which == joyId)
				{
					SDL_GameControllerClose(sdlPad)
					sdlPad = null
					joyId = -1
				}
			SDL_CONTROLLERBUTTONDOWN -> if (e.cbutton.which == joyId) when (e.cbutton.button.toInt())
			{
				SDL_CONTROLLER_BUTTON_A -> GamePad.setButtonPressed(GamePad.Button.A)
				SDL_CONTROLLER_BUTTON_B -> GamePad.setButtonPressed(GamePad.Button.B)
				SDL_CONTROLLER_BUTTON_X -> GamePad.setButtonPressed(GamePad.Button.X)
				SDL_CONTROLLER_BUTTON_Y -> GamePad.setButtonPressed(GamePad.Button.Y)
				SDL_CONTROLLER_BUTTON_LEFTSHOULDER -> GamePad.setButtonPressed(GamePad.Button.LEFT_SHOULDER)
				SDL_CONTROLLER_BUTTON_LEFTSTICK -> GamePad.setButtonPressed(GamePad.Button.LEFT_STICK)
				SDL_CONTROLLER_BUTTON_RIGHTSHOULDER -> GamePad.setButtonPressed(GamePad.Button.RIGHT_SHOULDER)
				SDL_CONTROLLER_BUTTON_RIGHTSTICK -> GamePad.setButtonPressed(GamePad.Button.RIGHT_STICK)
				SDL_CONTROLLER_BUTTON_DPAD_UP -> GamePad.setButtonPressed(GamePad.Button.DPAD_UP)
				SDL_CONTROLLER_BUTTON_DPAD_DOWN -> GamePad.setButtonPressed(GamePad.Button.DPAD_DOWN)
				SDL_CONTROLLER_BUTTON_DPAD_LEFT -> GamePad.setButtonPressed(GamePad.Button.DPAD_LEFT)
				SDL_CONTROLLER_BUTTON_DPAD_RIGHT -> GamePad.setButtonPressed(GamePad.Button.DPAD_RIGHT)
				SDL_CONTROLLER_BUTTON_START -> GamePad.setButtonPressed(GamePad.Button.START)
				SDL_CONTROLLER_BUTTON_BACK -> GamePad.setButtonPressed(GamePad.Button.BACK)
				SDL_CONTROLLER_BUTTON_GUIDE -> GamePad.setButtonPressed(GamePad.Button.GUIDE)
				SDL_CONTROLLER_BUTTON_MISC1 -> GamePad.setButtonPressed(GamePad.Button.MISC)
			}
			SDL_CONTROLLERBUTTONUP -> if (e.cbutton.which == joyId) when (e.cbutton.button.toInt())
			{
				SDL_CONTROLLER_BUTTON_A -> GamePad.setButtonReleased(GamePad.Button.A)
				SDL_CONTROLLER_BUTTON_B -> GamePad.setButtonReleased(GamePad.Button.B)
				SDL_CONTROLLER_BUTTON_X -> GamePad.setButtonReleased(GamePad.Button.X)
				SDL_CONTROLLER_BUTTON_Y -> GamePad.setButtonReleased(GamePad.Button.Y)
				SDL_CONTROLLER_BUTTON_LEFTSHOULDER -> GamePad.setButtonReleased(GamePad.Button.LEFT_SHOULDER)
				SDL_CONTROLLER_BUTTON_LEFTSTICK -> GamePad.setButtonReleased(GamePad.Button.LEFT_STICK)
				SDL_CONTROLLER_BUTTON_RIGHTSHOULDER -> GamePad.setButtonReleased(GamePad.Button.RIGHT_SHOULDER)
				SDL_CONTROLLER_BUTTON_RIGHTSTICK -> GamePad.setButtonReleased(GamePad.Button.RIGHT_STICK)
				SDL_CONTROLLER_BUTTON_DPAD_UP -> GamePad.setButtonReleased(GamePad.Button.DPAD_UP)
				SDL_CONTROLLER_BUTTON_DPAD_DOWN -> GamePad.setButtonReleased(GamePad.Button.DPAD_DOWN)
				SDL_CONTROLLER_BUTTON_DPAD_LEFT -> GamePad.setButtonReleased(GamePad.Button.DPAD_LEFT)
				SDL_CONTROLLER_BUTTON_DPAD_RIGHT -> GamePad.setButtonReleased(GamePad.Button.DPAD_RIGHT)
				SDL_CONTROLLER_BUTTON_START -> GamePad.setButtonReleased(GamePad.Button.START)
				SDL_CONTROLLER_BUTTON_BACK -> GamePad.setButtonReleased(GamePad.Button.BACK)
				SDL_CONTROLLER_BUTTON_GUIDE -> GamePad.setButtonReleased(GamePad.Button.GUIDE)
				SDL_CONTROLLER_BUTTON_MISC1 -> GamePad.setButtonReleased(GamePad.Button.MISC)
			}
			SDL_CONTROLLERAXISMOTION -> if (e.caxis.which == joyId) when (e.caxis.axis.toInt())
			{
				SDL_CONTROLLER_AXIS_LEFTX -> GamePad.rawStickLeftX = e.caxis.value.toFloat() / 0x7FFF
				SDL_CONTROLLER_AXIS_LEFTY -> GamePad.rawStickLeftY = e.caxis.value.toFloat() / 0x7FFF
				SDL_CONTROLLER_AXIS_RIGHTX -> GamePad.rawStickRightX = e.caxis.value.toFloat() / 0x7FFF
				SDL_CONTROLLER_AXIS_RIGHTY -> GamePad.rawStickRightY = e.caxis.value.toFloat() / 0x7FFF
				SDL_CONTROLLER_AXIS_TRIGGERLEFT -> GamePad.rawTriggerLeft = e.caxis.value.toFloat() / 0x7FFF
				SDL_CONTROLLER_AXIS_TRIGGERRIGHT -> GamePad.rawTriggerRight = e.caxis.value.toFloat() / 0x7FFF
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
		IMG_Quit()
		SDL_Quit()
	}

	fun run()
	{
		try
		{
			init()
			//SDL_GameControllerRumble(sdlPad, 0xFFFF, 0xFFFF, 1000)
			memScoped {
				val cEvent = alloc<SDL_Event>()
				while (running)
				{
					while (SDL_PollEvent(cEvent.ptr) > 0)
						onEvent(cEvent)
					tick()
					draw()
					//SDL_Delay(100)
				}
			}
		}
		finally
		{
			free()
		}
	}
}
