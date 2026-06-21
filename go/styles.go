package main

import "math"

// defaultFCLStyle returns the default FCL button style.
func defaultFCLStyle(name ...string) *OrderedMap {
	styleName := "Default"
	if len(name) > 0 {
		styleName = name[0]
	}
	return NewOrderedMapFromPairs(
		"name", styleName,
		"textColor", -1,
		"textSize", 12,
		"strokeColor", -12303292,
		"strokeWidth", 10,
		"cornerRadius", 100,
		"fillColor", 0,
		"textColorPressed", -1,
		"textSizePressed", 12,
		"strokeColorPressed", -12303292,
		"strokeWidthPressed", 10,
		"cornerRadiusPressed", 100,
		"fillColorPressed", -3355444,
	)
}

// defaultZLFallbackFCLStyle returns the ZL native default FCL style.
func defaultZLFallbackFCLStyle(name ...string) *OrderedMap {
	styleName := "ZL Native Default"
	if len(name) > 0 {
		styleName = name[0]
	}
	return NewOrderedMapFromPairs(
		"name", styleName,
		"textColor", -1,
		"textSize", 14,
		"strokeColor", -1,
		"strokeWidth", 0,
		"cornerRadius", 0,
		"fillColor", -2147483648,
		"textColorPressed", -1,
		"textSizePressed", 14,
		"strokeColorPressed", -1,
		"strokeWidthPressed", 0,
		"cornerRadiusPressed", 0,
		"fillColorPressed", -1282897784,
	)
}

// defaultFCLDirectionStyle returns the default FCL direction style.
func defaultFCLDirectionStyle() *OrderedMap {
	return NewOrderedMapFromPairs(
		"name", "Default",
		"styleType", "BUTTON",
		"buttonStyle", NewOrderedMapFromPairs(
			"interval", 50,
			"textColor", -1,
			"textSize", 12,
			"strokeColor", -12303292,
			"strokeWidth", 10,
			"cornerRadius", 100,
			"fillColor", 0,
			"textColorPressed", -1,
			"textSizePressed", 12,
			"strokeColorPressed", -12303292,
			"strokeWidthPressed", 10,
			"cornerRadiusPressed", 100,
			"fillColorPressed", -3355444,
		),
		"rockerStyle", NewOrderedMapFromPairs(
			"rockerSize", 400,
			"bgCornerRadius", 500,
			"bgStrokeWidth", 20,
			"bgStrokeColor", -12303292,
			"bgFillColor", 0,
			"rockerCornerRadius", 500,
			"rockerStrokeWidth", 10,
			"rockerStrokeColor", -12303292,
			"rockerFillColor", -7829368,
		),
	)
}

// emptyFCLEvent returns an empty FCL event.
func emptyFCLEvent() *OrderedMap {
	return NewOrderedMapFromPairs(
		"autoKeep", false,
		"autoClick", false,
		"openMenu", false,
		"switchTouchMode", false,
		"switchMouseMode", false,
		"input", false,
		"quickInput", false,
		"outputText", "",
		"outputKeycodes", []interface{}{},
		"bindViewGroup", []interface{}{},
	)
}

// fclButtonEvent returns the default FCL button event structure.
func fclButtonEvent() *OrderedMap {
	return NewOrderedMapFromPairs(
		"pointerFollow", false,
		"Movable", false,
		"pressEvent", emptyFCLEvent(),
		"longPressEvent", emptyFCLEvent(),
		"clickEvent", emptyFCLEvent(),
		"doubleClickEvent", emptyFCLEvent(),
	)
}

// zlShapeToFCLRadius converts ZL border radius shape to FCL corner radius.
func zlShapeToFCLRadius(shape interface{}) int {
	s, ok := asOrderedMap(shape)
	if !ok {
		return 100
	}
	keys := []string{"topStart", "topEnd", "bottomEnd", "bottomStart"}
	var values []float64
	for _, k := range keys {
		values = append(values, clampZLShape(getOr(s, k, 0.0), 0.0))
	}
	sum := 0.0
	for _, v := range values {
		sum += v
	}
	return clampInt(sum/float64(len(values))*10, 100)
}

// defaultZLJoystickStyleConfig returns the default ZL joystick style config.
func defaultZLJoystickStyleConfig() *OrderedMap {
	return NewOrderedMapFromPairs(
		"alpha", PyFloat(1.0),
		"backgroundColor", fclARGBToZLColor(0x80000000),
		"joystickColor", fclARGBToZLColor(0x80FFFFFF),
		"joystickCanLockColor", fclARGBToZLColor(0x80FFFF00),
		"joystickLockedColor", fclARGBToZLColor(0x8000FF00),
		"lockMarkColor", fclARGBToZLColor(0xFFFFFFFF),
		"borderWidthRatio", 0,
		"borderColor", fclARGBToZLColor(0xFFFFFFFF),
		"backgroundShape", 50,
		"joystickShape", 50,
		"joystickSize", PyFloat(0.5),
	)
}

// fclRockerStyleToZLJoystick converts FCL rocker style to ZL joystick style.
func fclRockerStyleToZLJoystick(style *OrderedMap) *OrderedMap {
	var rocker *OrderedMap
	if style != nil {
		if r, ok := style.Get("rockerStyle"); ok {
			if rm, ok := r.(*OrderedMap); ok && rm.Len() > 0 {
				rocker = rm
			}
		}
	}
	if rocker == nil {
		rocker = NewOrderedMap()
	}
	config := defaultZLJoystickStyleConfig()
	config.Set("backgroundColor", fclARGBToZLColor(getOr(rocker, "bgFillColor", 0x80000000)))
	config.Set("joystickColor", fclARGBToZLColor(getOr(rocker, "rockerFillColor", 0x80FFFFFF)))
	config.Set("borderColor", fclARGBToZLColor(getOr(rocker, "bgStrokeColor", 0xFFFFFFFF)))
	config.Set("borderWidthRatio", maxInt(0, minInt(50, clampInt(getOr(rocker, "bgStrokeWidth", 0))/10)))
	config.Set("backgroundShape", fclRadiusToZLPercent(getOr(rocker, "bgCornerRadius", 500)))
	config.Set("joystickShape", fclRadiusToZLPercent(getOr(rocker, "rockerCornerRadius", 500)))
	config.Set("joystickSize", PyFloat(fclRatioToZL(getOr(rocker, "rockerSize", 500))))
	return NewOrderedMapFromPairs(
		"uuid", shortID(),
		"lightStyle", config,
		"darkStyle", deepCopyJSON(config),
	)
}

// directionStyleMap builds a name->style map from a list of direction styles.
func directionStyleMap(styles []interface{}) map[string]*OrderedMap {
	result := map[string]*OrderedMap{}
	for _, item := range styles {
		s, ok := item.(*OrderedMap)
		if !ok {
			continue
		}
		name := toString(getOr(s, "name", ""))
		result[name] = s
	}
	return result
}

// resolveDirectionStyle resolves the direction style from a direction object.
func resolveDirectionStyle(direction *OrderedMap, styles map[string]*OrderedMap) *OrderedMap {
	style, ok := direction.Get("style")
	if ok {
		if sm, ok := style.(*OrderedMap); ok {
			return sm
		}
	}
	if s, ok := styles[toString(style)]; ok {
		return s
	}
	return NewOrderedMap()
}

// styleNameForZLStyle generates a style name with a suffix.
func styleNameForZLStyle(baseName, uuidValue string) string {
	suffix := ""
	if uuidValue != "" {
		if len(uuidValue) >= 6 {
			suffix = uuidValue[:6]
		} else {
			suffix = uuidValue
		}
	} else {
		s := shortID()
		if len(s) >= 6 {
			suffix = s[:6]
		} else {
			suffix = s
		}
	}
	return "ZL " + baseName + " " + suffix
}

// fclStylesToZL converts FCL button styles to ZL styles.
// Returns (styles list, name->uuid mapping).
func fclStylesToZL(styles []interface{}) ([]interface{}, map[string]string) {
	if len(styles) == 0 {
		styles = []interface{}{defaultFCLStyle()}
	}
	result := []interface{}{}
	mapping := map[string]string{}
	for _, item := range styles {
		style, ok := item.(*OrderedMap)
		if !ok {
			continue
		}
		name := toString(getOr(style, "name", "Default"))
		if name == "" {
			name = "Default"
		}
		sid := shortID()
		mapping[name] = sid
		radius := clampZLShape(clampFloat(getOr(style, "cornerRadius", 0), 0.0) / 10.0)
		pressedRadius := clampZLShape(clampFloat(getOr(style, "cornerRadiusPressed", getOr(style, "cornerRadius", 0)), 0.0) / 10.0)
		light := NewOrderedMapFromPairs(
		"alpha", PyFloat(1.0),
		"pressedAlpha", PyFloat(1.0),
		"backgroundColor", fclARGBToZLColor(getOr(style, "fillColor", 0)),
		"pressedBackgroundColor", fclARGBToZLColor(getOr(style, "fillColorPressed", -3355444)),
		"contentColor", fclARGBToZLColor(getOr(style, "textColor", -1), -1),
		"pressedContentColor", fclARGBToZLColor(getOr(style, "textColorPressed", -1), -1),
		"fontSize", fclFontToZL(getOr(style, "textSize", 12)),
		"pressedFontSize", fclFontToZL(getOr(style, "textSizePressed", getOr(style, "textSize", 12))),
		"borderWidth", clampZLBorderWidth(clampInt(getOr(style, "strokeWidth", 10))/10),
		"pressedBorderWidth", clampZLBorderWidth(clampInt(getOr(style, "strokeWidthPressed", 10))/10),
		"borderColor", fclARGBToZLColor(getOr(style, "strokeColor", -12303292), -12303292),
		"pressedBorderColor", fclARGBToZLColor(getOr(style, "strokeColorPressed", -12303292), -12303292),
		"borderRadius", NewOrderedMapFromPairs("topStart", PyFloat(radius), "topEnd", PyFloat(radius), "bottomEnd", PyFloat(radius), "bottomStart", PyFloat(radius)),
		"pressedBorderRadius", NewOrderedMapFromPairs("topStart", PyFloat(pressedRadius), "topEnd", PyFloat(pressedRadius), "bottomEnd", PyFloat(pressedRadius), "bottomStart", PyFloat(pressedRadius)),
	)
		result = append(result, NewOrderedMapFromPairs(
			"name", name,
			"uuid", sid,
			"animateSwap", false,
			"commonStyle", true,
			"lightStyle", light,
			"darkStyle", deepCopyJSON(light),
		))
	}
	return result, mapping
}

// makeZLButtonSize builds a ZL buttonSize object from FCL baseInfo.
func makeZLButtonSize(baseInfo *OrderedMap, absoluteAsPercentage bool, aspect float64) *OrderedMap {
	pw := getOrOrderedMap(baseInfo, "percentageWidth")
	ph := getOrOrderedMap(baseInfo, "percentageHeight")
	if toString(getOr(baseInfo, "sizeType", "")) == "ABSOLUTE" && absoluteAsPercentage {
		screenHeightDP := 411.0
		screenWidthDP := screenHeightDP * math.Max(0.1, clampFloat(aspect, 16.0/9.0))
		widthPercentage := maxInt(100, minInt(10000, int(math.Round(clampZLDP(getOr(baseInfo, "absoluteWidth", 50))/screenWidthDP*10000))))
		heightPercentage := maxInt(100, minInt(10000, int(math.Round(clampZLDP(getOr(baseInfo, "absoluteHeight", 50))/screenHeightDP*10000))))
		return NewOrderedMapFromPairs(
			"type", "percentage",
			"widthDp", PyFloat(clampZLDP(getOr(baseInfo, "absoluteWidth", 50))),
			"heightDp", PyFloat(clampZLDP(getOr(baseInfo, "absoluteHeight", 50))),
			"widthPercentage", widthPercentage,
			"heightPercentage", heightPercentage,
			"widthReference", "screen_width",
			"heightReference", "screen_height",
		)
	}
	sizeType := "percentage"
	if toString(getOr(baseInfo, "sizeType", "")) == "ABSOLUTE" {
		sizeType = "dp"
	}
	return NewOrderedMapFromPairs(
		"type", sizeType,
		"widthDp", PyFloat(clampZLDP(getOr(baseInfo, "absoluteWidth", 50))),
		"heightDp", PyFloat(clampZLDP(getOr(baseInfo, "absoluteHeight", 50))),
		"widthPercentage", fclSizeToZL(getOr(pw, "size", 50)),
		"heightPercentage", fclSizeToZL(getOr(ph, "size", 50)),
		"widthReference", fclRefToZL(toString(getOr(pw, "reference", ""))),
		"heightReference", fclRefToZl(toString(getOr(ph, "reference", ""))),
	)
}

// fclRefToZl converts FCL reference name to ZL reference name.
func fclRefToZl(reference string) string {
	if reference == "SCREEN_HEIGHT" {
		return "screen_height"
	}
	return "screen_width"
}
