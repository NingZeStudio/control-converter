package main

import "math"

// fclDirectionRectToZLGrid computes the grid layout for an FCL direction control.
// Returns: widgetX, widgetY, size, p0, p1, p2, screenW, screenH, reference, buttonSize, childPX
type directionGrid struct {
	widgetX    int
	widgetY    int
	size       int
	p0         int
	p1         int
	p2         int
	screenW    float64
	screenH    float64
	reference  string
	buttonSize *OrderedMap
	childPX    float64
}

func fclDirectionRectToZLGrid(direction, style *OrderedMap, aspect float64, joined bool) directionGrid {
	base := getOrOrderedMap(direction, "baseInfo")
	buttonStyle := getOrOrderedMap(style, "buttonStyle")
	absolute := toString(getOr(base, "sizeType", "")) == "ABSOLUTE"

	var screenH, screenW, referenceSize float64
	var reference string
	var viewSize int

	if absolute {
		screenH = 411.0
		screenW = screenH * math.Max(0.1, clampFloat(aspect, 16.0/9.0))
		reference = "SCREEN_HEIGHT"
		referenceSize = screenH
		viewSize = maxInt(1, clampInt(getOr(base, "absoluteWidth", 50), 50))
	} else {
		screenH = 10000.0
		screenW = screenH * math.Max(0.1, clampFloat(aspect, 16.0/9.0))
		pw := getOrOrderedMap(base, "percentageWidth")
		reference = toString(getOr(pw, "reference", "SCREEN_WIDTH"))
		if reference == "" {
			reference = "SCREEN_WIDTH"
		}
		if reference == "SCREEN_HEIGHT" {
			referenceSize = screenH
		} else {
			referenceSize = screenW
		}
		viewSize = maxInt(1, int(referenceSize*float64(clampInt(getOr(pw, "size", 100)))/1000.0))
	}

	widgetX := int((screenW - float64(viewSize)) * float64(clampInt(getOr(base, "xPosition", 0))) / 1000.0)
	widgetY := int((screenH - float64(viewSize)) * float64(clampInt(getOr(base, "yPosition", 0))) / 1000.0)
	interval := maxInt(0, minInt(499, clampInt(getOr(buttonStyle, "interval", 50), 50)))
	childSize := maxInt(1, int(float64(viewSize)*float64(1000-(2*interval))/3000.0))

	var p0, p1, p2 int
	if joined {
		if !absolute {
			reference = "SCREEN_HEIGHT"
			referenceSize = screenH
			childSize = maxInt(childSize, int(screenH*1350/10000))
		}
		gap := maxInt(0, int(float64(childSize)*3*float64(interval)/math.Max(1, float64(1000-(2*interval)))))
		p0 = 0
		p1 = childSize + gap
		p2 = (childSize + gap) * 2
	} else {
		p0 = 0
		p1 = childSize + int(float64(viewSize)*float64(interval)/1000.0)
		p2 = viewSize - childSize
	}

	childPercentage := maxInt(100, minInt(10000, pyRound(float64(childSize)/referenceSize*10000)))
	var buttonSize *OrderedMap
	if absolute {
		buttonSize = NewOrderedMapFromPairs(
			"type", "dp",
			"widthDp", PyFloat(clampZLDP(childSize)),
			"heightDp", PyFloat(clampZLDP(childSize)),
			"widthPercentage", childPercentage,
			"heightPercentage", childPercentage,
			"widthReference", "screen_height",
			"heightReference", "screen_height",
		)
	} else {
		buttonSize = NewOrderedMapFromPairs(
			"type", "percentage",
			"widthDp", PyFloat(50.0),
			"heightDp", PyFloat(50.0),
			"widthPercentage", childPercentage,
			"heightPercentage", childPercentage,
			"widthReference", fclRefNameToZL(reference),
			"heightReference", fclRefNameToZL(reference),
		)
	}
	return directionGrid{
		widgetX:    widgetX,
		widgetY:    widgetY,
		size:       childPercentage,
		p0:         p0,
		p1:         p1,
		p2:         p2,
		screenW:    screenW,
		screenH:    screenH,
		reference:  reference,
		buttonSize: buttonSize,
		childPX:    float64(childSize),
	}
}

func fclRefNameToZL(reference string) string {
	if reference == "SCREEN_HEIGHT" {
		return "screen_height"
	}
	return "screen_width"
}

func pixelToZLPosition(pixel int, screen, child float64) int {
	available := math.Max(1.0, screen-child)
	return maxInt(0, minInt(10000, pyRound(float64(pixel)/available*10000)))
}

// directionEventKeycodes extracts keycodes for a direction event, with a default.
func directionEventKeycodes(event *OrderedMap, name string, defaultKeycode int) []interface{} {
	value, ok := event.Get(name)
	if !ok || value == nil {
		return []interface{}{defaultKeycode}
	}
	keycodes := fclKeycodeList(value)
	if len(keycodes) == 0 {
		return []interface{}{defaultKeycode}
	}
	return keycodes
}

// directionToZLButtons converts an FCL direction control to a list of ZL buttons.
func directionToZLButtons(
	direction, style *OrderedMap,
	styleUUID string,
	strict bool,
	aspect float64,
	joined bool,
) []*OrderedMap {
	base := getOrOrderedMap(direction, "baseInfo")
	event := getOrOrderedMap(direction, "event")
	grid := fclDirectionRectToZLGrid(direction, style, aspect, joined)

	upKeys := directionEventKeycodes(event, "upKeycode", GLFWToFCL["GLFW_KEY_W"])
	downKeys := directionEventKeycodes(event, "downKeycode", GLFWToFCL["GLFW_KEY_S"])
	leftKeys := directionEventKeycodes(event, "leftKeycode", GLFWToFCL["GLFW_KEY_A"])
	rightKeys := directionEventKeycodes(event, "rightKeycode", GLFWToFCL["GLFW_KEY_D"])

	type entry struct {
		text     string
		dx, dy   int
		keycodes []interface{}
		isCenter bool
	}
	entries := []entry{
		{"◤", grid.p0, grid.p0, append(append([]interface{}{}, upKeys...), leftKeys...), false},
		{"▲", grid.p1, grid.p0, upKeys, false},
		{"◥", grid.p2, grid.p0, append(append([]interface{}{}, upKeys...), rightKeys...), false},
		{"◀", grid.p0, grid.p1, leftKeys, false},
		{"", grid.p1, grid.p1, []interface{}{}, true},
		{"▶", grid.p2, grid.p1, rightKeys, false},
		{"◣", grid.p0, grid.p2, append(append([]interface{}{}, downKeys...), leftKeys...), false},
		{"▼", grid.p1, grid.p2, downKeys, false},
		{"◢", grid.p2, grid.p2, append(append([]interface{}{}, downKeys...), rightKeys...), false},
	}

	var buttons []*OrderedMap
	for _, e := range entries {
		var clickEvents []*OrderedMap
		var substitutions []interface{}
		for _, kc := range e.keycodes {
			converted := convertKeyToZL(clampInt(kc), strict, false, e.text, &substitutions)
			if converted != nil {
				clickEvents = append(clickEvents, NewOrderedMapFromPairs("type", converted.eventType, "key", converted.key))
			}
		}
		if e.isCenter {
			continue
		}
		buttonObj := NewOrderedMapFromPairs(
			"text", translatable(e.text),
			"uuid", shortID()+shortID()[:6],
			"position", NewOrderedMapFromPairs(
				"x", pixelToZLPosition(grid.widgetX+e.dx, grid.screenW, grid.childPX),
				"y", pixelToZLPosition(grid.widgetY+e.dy, grid.screenH, grid.childPX),
			),
			"buttonSize", deepCopyJSON(grid.buttonSize),
			"buttonStyle", styleUUID,
			"textAlignment", "Left",
			"textBold", false,
			"textItalic", false,
			"textUnderline", false,
			"visibilityType", visibilityFCLToZL(toString(getOr(base, "visibilityType", ""))),
			"clickEvents", clickEventsToInterface(clickEvents),
			"isSwipple", true,
			"isPenetrable", false,
			"isToggleable", false,
		)
		buttons = append(buttons, setMeta(buttonObj, makeMeta(
			"fcl",
			"direction",
			toString(getOr(direction, "id", "")),
			direction,
			appendSubstitutions(NewOrderedMapFromPairs("synthetic", true, "generatedFrom", "direction-grid"), substitutions),
		)))
	}
	return buttons
}

// directionViewSize computes the pixel size of the square FCL direction view,
// mirroring the Python direction_view_size (which mirrors fclDirectionRectToZLGrid).
func directionViewSize(base *OrderedMap, aspect float64) int {
	if toString(getOr(base, "sizeType", "")) == "ABSOLUTE" {
		return maxInt(1, clampInt(getOr(base, "absoluteWidth", 50), 50))
	}
	screenH := 10000.0
	screenW := screenH * math.Max(0.1, clampFloat(aspect, 16.0/9.0))
	pw := getOrOrderedMap(base, "percentageWidth")
	reference := toString(getOr(pw, "reference", "SCREEN_WIDTH"))
	referenceSize := screenW
	if reference == "SCREEN_HEIGHT" {
		referenceSize = screenH
	}
	return maxInt(1, int(referenceSize*float64(clampInt(getOr(pw, "size", 100)))/1000.0))
}

// zlKeyEventsFromKeycodes converts a list of FCL keycodes to ZL key events.
func zlKeyEventsFromKeycodes(keycodes []interface{}, strict bool) []*OrderedMap {
	var events []*OrderedMap
	for _, kc := range keycodes {
		converted := convertKeyToZL(clampInt(kc), strict, false, "", nil)
		if converted != nil {
			events = append(events, NewOrderedMapFromPairs("type", converted.eventType, "key", converted.key))
		}
	}
	return events
}

// directionToZLJoystick converts an FCL ROCKER direction control to a ZL joystick control.
func directionToZLJoystick(
	direction, style *OrderedMap,
	joystickStyleUUID string,
	strict bool,
	aspect float64,
) *OrderedMap {
	base := getOrOrderedMap(direction, "baseInfo")
	event := getOrOrderedMap(direction, "event")
	grid := fclDirectionRectToZLGrid(direction, style, aspect, true)
	absolute := toString(getOr(base, "sizeType", "")) == "ABSOLUTE"
	viewSize := directionViewSize(base, aspect)

	upKeys := directionEventKeycodes(event, "upKeycode", GLFWToFCL["GLFW_KEY_W"])
	downKeys := directionEventKeycodes(event, "downKeycode", GLFWToFCL["GLFW_KEY_S"])
	leftKeys := directionEventKeycodes(event, "leftKeycode", GLFWToFCL["GLFW_KEY_A"])
	rightKeys := directionEventKeycodes(event, "rightKeycode", GLFWToFCL["GLFW_KEY_D"])
	up := zlKeyEventsFromKeycodes(upKeys, strict)
	down := zlKeyEventsFromKeycodes(downKeys, strict)
	left := zlKeyEventsFromKeycodes(leftKeys, strict)
	right := zlKeyEventsFromKeycodes(rightKeys, strict)

	var sizeType string
	var sizeDp float64
	var sizePercentage int
	if absolute {
		sizeType = "dp"
		sizeDp = clampZLDP(viewSize)
		sizePercentage = 2500
	} else {
		sizeType = "percentage"
		sizePercentage = maxInt(2000, minInt(10000, int(math.Round(float64(viewSize)/grid.screenH*10000))))
		sizeDp = 200.0
	}

	joystickObj := NewOrderedMapFromPairs(
		"uuid", shortID()+shortID()[:6],
		"position", NewOrderedMapFromPairs(
			"x", pixelToZLPosition(grid.widgetX, grid.screenW, float64(viewSize)),
			"y", pixelToZLPosition(grid.widgetY, grid.screenH, float64(viewSize)),
		),
		"sizeType", sizeType,
		"sizeDp", PyFloat(sizeDp),
		"sizePercentage", sizePercentage,
		"visibilityType", visibilityFCLToZL(toString(getOr(base, "visibilityType", ""))),
		"joystickStyleId", joystickStyleUUID,
		"deadZoneRatio", PyFloat(0.5),
		"lockThreshold", PyFloat(0.3),
		"canLock", true,
		"triggerMode", "drag",
		"directionEvents", NewOrderedMapFromPairs(
			"north", eventsToInterface(up),
			"north_east", eventsToInterface(appendEvents(up, right)),
			"north_west", eventsToInterface(appendEvents(up, left)),
			"south", eventsToInterface(down),
			"south_east", eventsToInterface(appendEvents(down, right)),
			"south_west", eventsToInterface(appendEvents(down, left)),
			"east", eventsToInterface(right),
			"west", eventsToInterface(left),
		),
		"lockEvents", []interface{}{},
	)

	originID := toString(getOr(direction, "id", toString(getOr(joystickObj, "uuid", ""))))
	return setMeta(joystickObj, makeMeta(
		"fcl",
		"direction",
		originID,
		direction,
		NewOrderedMapFromPairs("synthetic", true, "generatedFrom", "direction-joystick"),
	))
}

// eventsToInterface converts []*OrderedMap to []interface{}.
func eventsToInterface(events []*OrderedMap) []interface{} {
	result := make([]interface{}, len(events))
	for i, e := range events {
		result[i] = e
	}
	return result
}

// appendEvents returns a new slice concatenating a and b (shallow copy).
func appendEvents(a, b []*OrderedMap) []*OrderedMap {
	result := make([]*OrderedMap, 0, len(a)+len(b))
	result = append(result, a...)
	result = append(result, b...)
	return result
}
