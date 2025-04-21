package com.a_dinosaur.kotlinsdl

import kotlin.math.*

data class Vector2f(var x: Float, var y: Float)
{
	companion object
	{
		val ZERO: Vector2f get() = Vector2f(0.0f, 0.0f)

		fun fromAngle(theta: Float): Vector2f = Vector2f(cos(theta), sin(theta))
	}

	constructor(): this(0.0f, 0.0f)
	constructor(v: Float): this(v, v)

	// relational
	override operator fun equals(other: Any?): Boolean
	{
		if (other !is Vector2f)
			return false
		return x == other.x && y == other.y
	}
	override fun hashCode(): Int { return 31 * x.hashCode() + y.hashCode() }

	// arithmetic
	operator fun plus(rhs: Vector2f): Vector2f { return Vector2f(x + rhs.x, y + rhs.y) }
	operator fun minus(rhs: Vector2f): Vector2f { return Vector2f(x - rhs.x, y - rhs.y) }
	operator fun times(rhs: Vector2f): Vector2f { return Vector2f(x * rhs.x, y * rhs.y) }
	operator fun div(rhs: Vector2f): Vector2f { return Vector2f(x / rhs.x, y / rhs.y) }

	// scalar arithmetic
	operator fun times(rhs: Float): Vector2f { return Vector2f(x * rhs, y * rhs) }
	operator fun div(rhs: Float): Vector2f { return Vector2f(x / rhs, y / rhs) }

	/*
	// compound arithmetic
	operator fun plusAssign(rhs: Vector2f) { x += rhs.x; y += rhs.y }
	operator fun minusAssign(rhs: Vector2f) { x -= rhs.x; y -= rhs.y }
	operator fun timesAssign(rhs: Vector2f) { x *= rhs.x; y *= rhs.y }
	operator fun divAssign(rhs: Vector2f) { x /= rhs.x; y /= rhs.y }

	// compound scalar arithmetic
	operator fun timesAssign(rhs: Float) { x *= rhs; y *= rhs }
	operator fun divAssign(rhs: Float) { x /= rhs; y /= rhs }
	*/

	// unary
	operator fun unaryPlus(): Vector2f { return this }
	operator fun unaryMinus(): Vector2f { return Vector2f(-x, -y) }

	fun dot(rhs: Vector2f): Float { return x * rhs.x + y * rhs.y }
	fun reflect(n: Vector2f): Vector2f { return this - (n * 2.0f * dot(n)) }

	val magnitudeSquared: Float get() = x * x + y * y
	val magnitude: Float get() = sqrt(magnitudeSquared)
	val normalised: Vector2f get() = this / magnitude
	val angle: Float get() = atan2(y, x)
}
