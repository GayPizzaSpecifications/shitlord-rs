package com.a_dinosaur.kotlinsdl

@ThreadLocal
object GamePad
{
	enum class Button(val mask: UInt)
	{
		NONE(0x00u),

		A(0x0001u),
		B(0x0002u),
		X(0x0004u),
		Y(0x0008u),
		LEFT_SHOULDER(0x0010u),
		LEFT_STICK(0x0020u),
		RIGHT_SHOULDER(0x0040u),
		RIGHT_STICK(0x0080u),
		DPAD_UP(0x0100u),
		DPAD_DOWN(0x0200u),
		DPAD_LEFT(0x0400u),
		DPAD_RIGHT(0x0800u),
		START(0x1000u),
		BACK(0x2000u),
		GUIDE(0x4000u),
		MISC(0x8000u)
	}

	data class Stick(
		var raw: Vector2f = Vector2f.ZERO,
		var compensated: Vector2f = Vector2f.ZERO,
		var dirty: Boolean = true)
	{
		fun update()
		{
			if (!dirty)
				return
			compensated = radialDeadzone(raw, 0.1f, 1.0f)
			dirty = false
		}
	}

	var rumble: Float = 0.0f

	private val stickLeft = Stick()
	private val stickRight = Stick()
	private var triggerLeft = 0.0f
	private var triggerRight = 0.0f
	private var buttonState = Button.NONE.mask

	var rawStickLeftX: Float get() = stickLeft.raw.x; set(v) { stickLeft.raw.x = v; stickLeft.dirty = true }
	var rawStickLeftY: Float get() = stickLeft.raw.y; set(v) { stickLeft.raw.y = v; stickLeft.dirty = true }
	var rawStickRightX: Float get() = stickRight.raw.x; set(v) { stickRight.raw.x = v; stickRight.dirty = true }
	var rawStickRightY: Float get() = stickRight.raw.y; set(v) { stickRight.raw.y = v; stickRight.dirty = true }
	var rawTriggerLeft: Float get() = triggerLeft; set(v) { triggerLeft = v }
	var rawTriggerRight: Float get() = triggerRight; set(v) { triggerRight = v }

	val leftStick: Vector2f get() = stickLeft.compensated
	val rightStick: Vector2f get() = stickRight.compensated
	val leftTrigger: Float get() = triggerLeft
	val rightTrigger: Float get() = triggerRight

	fun setButtonPressed(button: Button)
	{
		buttonState = buttonState or button.mask
	}

	fun setButtonReleased(button: Button)
	{
		buttonState = buttonState and button.mask.inv()
	}

	fun down(button: Button): Boolean = buttonState and button.mask == button.mask
	fun up(button: Button): Boolean = buttonState and button.mask != button.mask

	private fun radialDeadzone(v: Vector2f, min: Float, max: Float): Vector2f
	{
		val magnitude = v.magnitude
		if (magnitude == 0.0f || magnitude < min)
			return Vector2f.ZERO
		if (magnitude > max)
			return v / magnitude
		val rescale = (magnitude - min) / (max - min)
		return v / magnitude * rescale
	}

	fun tick()
	{
		stickLeft.update()
		stickRight.update()
	}
}
