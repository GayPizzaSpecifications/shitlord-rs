package com.a_dinosaur.kotlinsdl.maths

import kotlin.math.*

data class Vec2f(var x: Float, var y: Float)
{
	constructor(): this(0.0f, 0.0f)
	constructor(v: Float): this(v, v)

	companion object
	{
		inline val ZERO: Vec2f get() = Vec2f(0.0f, 0.0f)

		fun fromAngle(theta: Float) = Vec2f(cos(theta), sin(theta))
	}
	
	// relational
	override operator fun equals(other: Any?): Boolean
	{
		if (other !is Vec2f)
			return false
		return x == other.x && y == other.y
	}
	override fun hashCode(): Int { return 31 * x.hashCode() + y.hashCode() }
	
	// arithmetic
	operator fun plus(rhs: Vec2f) = Vec2f(x + rhs.x, y + rhs.y)
	operator fun minus(rhs: Vec2f) = Vec2f(x - rhs.x, y - rhs.y)
	operator fun times(rhs: Vec2f) = Vec2f(x * rhs.x, y * rhs.y)
	operator fun div(rhs: Vec2f) = Vec2f(x / rhs.x, y / rhs.y)

	// scalar arithmetic
	operator fun times(rhs: Float) = Vec2f(x * rhs, y * rhs)
	operator fun div(rhs: Float) = Vec2f(x / rhs, y / rhs)
	
	// unary
	operator fun unaryPlus() = this
	operator fun unaryMinus() = Vec2f(-x, -y)

	fun dot(rhs: Vec2f) = x * rhs.x + y * rhs.y
	fun reflect(n: Vec2f) = this - (n * 2.0f * dot(n))

	inline val magnitudeSquared: Float get() = x * x + y * y
	inline val magnitude: Float get() = sqrt(magnitudeSquared)
	inline val normalised: Vec2f get() = this / magnitude
	inline val angle: Float get() = atan2(y, x)
}

typealias Vec2 = Vec2f
