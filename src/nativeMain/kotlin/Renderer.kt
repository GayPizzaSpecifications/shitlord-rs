@file:OptIn(ExperimentalForeignApi::class)

package com.a_dinosaur.kotlinsdl

import kotlinx.cinterop.*
import io.karma.sdl.*
import com.a_dinosaur.kotlinsdl.maths.Vec2
import kotlin.native.concurrent.ThreadLocal

@ThreadLocal
object Renderer
{
	data class Rect(val x: Float, val y: Float, val w: Float, val h: Float)

	enum class Flip(val value: SDL_FlipMode)
	{
		None(SDL_FlipMode.SDL_FLIP_NONE),
		Horizontal(SDL_FlipMode.SDL_FLIP_HORIZONTAL),
		Vertical(SDL_FlipMode.SDL_FLIP_VERTICAL),
		Both(SDL_FlipMode.byValue(SDL_FlipMode.SDL_FLIP_HORIZONTAL.value and SDL_FlipMode.SDL_FLIP_VERTICAL.value))
	}

	enum class BlendMode(val mode: SDL_BlendMode)
	{
		None(SDL_BLENDMODE_NONE),
		Blend(SDL_BLENDMODE_BLEND),
		Add(SDL_BLENDMODE_ADD),
		Modulate(SDL_BLENDMODE_MOD),
		Multiply(SDL_BLENDMODE_MUL),
		Invalid(SDL_BLENDMODE_INVALID)
	}

	data class Colour(val r: UByte, val g: UByte, val b: UByte, val a: UByte)
	{
		constructor(rgba: UInt): this(
			(rgba and 0xFF000000u shr 24).toUByte(),
			(rgba and 0x00FF0000u shr 16).toUByte(),
			(rgba and 0x0000FF00u shr 8).toUByte(),
			(rgba and 0x000000FFu).toUByte())

		companion object
		{
			val BLACK = Colour(0x000000FFu)
		}
	}

	private lateinit var renderer: CPointer<cnames.structs.SDL_Renderer>

	// https://en.wikipedia.org/wiki/Kotlin-class_destroyer
	private val sdlSrc = nativeHeap.alloc<SDL_FRect>()
	private val sdlDst = nativeHeap.alloc<SDL_FRect>()

	private val textures: MutableMap<String, CPointer<SDL_Texture>> = mutableMapOf()

	fun init(window: CPointer<cnames.structs.SDL_Window>, vsync: Boolean)
	{
		renderer = SDL_CreateRenderer(window, null)
			?: throw Error("SDL_CreateRenderer returned ${SDL_GetError()}")

		SDL_SetRenderVSync(renderer, if (vsync) 1 else 0)
		SDL_SetRenderLogicalPresentation(renderer, 640, 480,
			SDL_RendererLogicalPresentation.SDL_LOGICAL_PRESENTATION_LETTERBOX)
	}

	fun free()
	{
		textures.values.forEach {
			SDL_DestroyTexture(it)
		}
		textures.clear()
		SDL_DestroyRenderer(renderer)
	}

	fun present()
	{
		SDL_RenderPresent(renderer)
	}

	fun setDrawColour(colour: Colour)
	{
		SDL_SetRenderDrawColor(renderer, colour.r, colour.g, colour.g, colour.a)
	}

	fun setBlendMode(blendMode: BlendMode)
	{
		SDL_SetRenderDrawBlendMode(renderer, blendMode.mode)
	}

	fun clear()
	{
		SDL_RenderClear(renderer)
	}

	fun line(v1: Vec2, v2: Vec2)
	{
		SDL_RenderLine(renderer, v1.x, v1.y, v2.x, v2.y)
	}

	fun boxFill(dst: Rect)
	{
		sdlDst.x = dst.x
		sdlDst.y = dst.y
		sdlDst.w = dst.w
		sdlDst.h = dst.h
		SDL_RenderFillRect(renderer, sdlDst.ptr)
	}

	fun loadTexture(path: String): CPointer<SDL_Texture>? = textures.getOrElse(path) {
		SDL_CreateTextureFromSurface(renderer, SDL_LoadBMP(path.replaceAfter(".", "bmp")))
	}

	fun copy(texture: CPointer<SDL_Texture>)
	{
		SDL_RenderTexture(renderer, texture, null, null)
	}

	fun copy(texture: CPointer<SDL_Texture>, dst: Rect)
	{
		sdlDst.x = dst.x
		sdlDst.y = dst.y
		sdlDst.w = dst.w
		sdlDst.h = dst.h
		SDL_RenderTexture(renderer, texture, null, sdlDst.ptr)
	}

	fun copy(texture: CPointer<SDL_Texture>, src: Rect, dst: Rect)
	{
		sdlSrc.x = src.x
		sdlSrc.y = src.y
		sdlSrc.w = src.w
		sdlSrc.h = src.h
		sdlDst.x = dst.x
		sdlDst.y = dst.y
		sdlDst.w = dst.w
		sdlDst.h = dst.h
		SDL_RenderTexture(renderer, texture, sdlSrc.ptr, sdlDst.ptr)
	}

	fun copy(texture: CPointer<SDL_Texture>, dst: Rect, angle: Double, flip: Flip)
	{
		sdlDst.x = dst.x
		sdlDst.y = dst.y
		sdlDst.w = dst.w
		sdlDst.h = dst.h
		SDL_RenderTextureRotated(renderer, texture, null, sdlDst.ptr, angle, null, flip.value)
	}
}
