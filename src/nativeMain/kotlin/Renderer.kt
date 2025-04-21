package com.a_dinosaur.kotlinsdl

import kotlinx.cinterop.*
import SDL2.*
import SDL2.SDL_Rect
import SDL2_image.*

@ThreadLocal
object Renderer
{
	data class Rect(val x: Int, val y: Int, val w: Int, val h: Int)
	data class FRect(val x: Float, val y: Float, val w: Float, val h: Float)

	enum class Flip(val value: SDL_RendererFlip)
	{
		None(SDL_FLIP_NONE),
		Horizontal(SDL_FLIP_HORIZONTAL),
		Vertical(SDL_FLIP_VERTICAL),
		Both(SDL_FLIP_HORIZONTAL and SDL_FLIP_VERTICAL)
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

	private lateinit var renderer: CPointer<SDL_Renderer>

	// https://en.wikipedia.org/wiki/Kotlin-class_destroyer
	private val sdlSrc = nativeHeap.alloc<SDL_Rect>()
	private val sdlDst = nativeHeap.alloc<SDL_Rect>()
	private val sdlFDst = nativeHeap.alloc<SDL_FRect>()

	private val textures: MutableMap<String, CPointer<SDL_Texture>> = mutableMapOf()

	fun init(window: CPointer<SDL_Window>, flags: SDL_RendererFlags)
	{
		renderer = SDL_CreateRenderer(window, -1, flags)
			?: throw Error("SDL_CreateRenderer returned ${SDL_GetError()}")

		SDL_RenderSetLogicalSize(renderer, 640, 480)
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

	fun line(v1: Vector2f, v2: Vector2f)
	{
		SDL_RenderDrawLineF(renderer, v1.x, v1.y, v2.x, v2.y)
	}

	fun boxFill(dst: Rect)
	{
		sdlDst.x = dst.x
		sdlDst.y = dst.y
		sdlDst.w = dst.w
		sdlDst.h = dst.h
		SDL_RenderFillRect(renderer, sdlDst.ptr)
	}

	fun loadTexture(path: String): CPointer<SDL_Texture>?
	{
		return textures.getOrElse(path) {
			IMG_LoadTexture(renderer, path)?.let { texture -> textures[path] = texture; texture }
		}
	}

	fun copy(texture: CPointer<SDL_Texture>)
	{
		SDL_RenderCopy(renderer, texture, null, null)
	}

	fun copy(texture: CPointer<SDL_Texture>, dst: Rect)
	{
		sdlDst.x = dst.x
		sdlDst.y = dst.y
		sdlDst.w = dst.w
		sdlDst.h = dst.h
		SDL_RenderCopy(renderer, texture, null, sdlDst.ptr)
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
		SDL_RenderCopy(renderer, texture, sdlSrc.ptr, sdlDst.ptr)
	}

	fun copy(texture: CPointer<SDL_Texture>, dst: FRect, angle: Double, flip: Flip)
	{
		sdlFDst.x = dst.x
		sdlFDst.y = dst.y
		sdlFDst.w = dst.w
		sdlFDst.h = dst.h
		SDL_RenderCopyExF(renderer, texture, null, sdlFDst.ptr, angle, null, flip.value)
	}
}
