package main

// overlaySharedFieldsZL restores overlay fields from original onto current ZL button.
func overlaySharedFieldsZL(original, current *OrderedMap, styleMap map[string]string, absoluteAsPercentage bool, aspect float64) *OrderedMap {
	restored := deepCopyJSON(original).(*OrderedMap)
	baseInfo := getOrOrderedMap(current, "baseInfo")
	sourceText := getOrEmpty(current, "text")
	if st, ok := sourceText.(*OrderedMap); ok {
		restored.Set("text", translatable(textDefault(st), st))
	} else {
		restored.Set("text", translatable(toString(sourceText), getOr(restored, "text", nil)))
	}
	restored.Set("uuid", toString(getOr(current, "id", getOr(restored, "uuid", shortID()+shortID()[:6]))))
	restored.Set("position", NewOrderedMapFromPairs(
		"x", scalePositionToZL(getOr(baseInfo, "xPosition", 0)),
		"y", scalePositionToZL(getOr(baseInfo, "yPosition", 0)),
	))
	restored.Set("buttonSize", makeZLButtonSize(baseInfo, absoluteAsPercentage, aspect))
	restored.Set("buttonStyle", getStyleFromMap(styleMap, toString(getOr(current, "style", "Default")), getOr(restored, "buttonStyle", nil)))
	restored.Set("visibilityType", visibilityFCLToZL(toString(getOr(baseInfo, "visibilityType", ""))))
	return restored
}

func getStyleFromMap(styleMap map[string]string, name string, defaultVal interface{}) interface{} {
	if uuid, ok := styleMap[name]; ok {
		return uuid
	}
	return defaultVal
}

// fclButtonToZLTextbox converts an FCL button to a ZL textbox.
func fclButtonToZLTextbox(button *OrderedMap, styleMap map[string]string, absoluteAsPercentage bool, aspect float64) *OrderedMap {
	original := metaOriginal(button, "zl")
	if original != nil {
		if origMap, ok := original.(*OrderedMap); ok {
			if _, hasClickEvents := origMap.Get("clickEvents"); !hasClickEvents {
				restored := overlaySharedFieldsZL(origMap, button, styleMap, absoluteAsPercentage, aspect)
				return setMeta(restored, makeMeta("fcl", "button", toString(getOr(button, "id", getOr(restored, "uuid", shortID()))), button, NewOrderedMapFromPairs("synthetic", true, "generatedFrom", "decorative-textbox")))
			}
		}
	}

	baseInfo := getOrOrderedMap(button, "baseInfo")
	text := toString(getOr(button, "text", ""))
	result := NewOrderedMapFromPairs(
		"text", translatable(text),
		"uuid", toString(getOr(button, "id", shortID()+shortID()[:6])),
		"position", NewOrderedMapFromPairs(
			"x", scalePositionToZL(getOr(baseInfo, "xPosition", 0)),
			"y", scalePositionToZL(getOr(baseInfo, "yPosition", 0)),
		),
		"buttonSize", makeZLButtonSize(baseInfo, absoluteAsPercentage, aspect),
		"buttonStyle", styleMap[toString(getOr(button, "style", "Default"))],
		"textAlignment", "Left",
		"textBold", false,
		"textItalic", false,
		"textUnderline", false,
		"visibilityType", visibilityFCLToZL(toString(getOr(baseInfo, "visibilityType", ""))),
	)
	return setMeta(result, makeMeta("fcl", "button", toString(getOr(button, "id", result.GetMust("uuid"))), button, NewOrderedMapFromPairs("synthetic", true, "generatedFrom", "decorative-textbox")))
}

// fclButtonToZL converts an FCL button to a ZL button.
func fclButtonToZL(
	button *OrderedMap,
	styleMap map[string]string,
	strict bool,
	groupName string,
	groupIDsByName map[string]string,
	visualButton *OrderedMap,
	absoluteAsPercentage bool,
	aspect float64,
) *OrderedMap {
	original := metaOriginal(button, "zl")
	if original != nil {
		if origMap, ok := original.(*OrderedMap); ok {
			if _, hasClickEvents := origMap.Get("clickEvents"); hasClickEvents {
				vb := visualButton
				if vb == nil {
					vb = button
				}
				restored := overlaySharedFieldsZL(origMap, vb, styleMap, absoluteAsPercentage, aspect)
				return setMeta(restored, makeMeta("fcl", "button", toString(getOr(button, "id", getOr(restored, "uuid", shortID()))), button))
			}
		}
	}

	if visualButton == nil {
		visualButton = button
	}
	baseInfo := getOrOrderedMap(visualButton, "baseInfo")
	eventRoot := getOrOrderedMap(button, "event")
	text := toString(getOr(visualButton, "text", toString(getOr(button, "text", ""))))
	if groupIDsByName == nil {
		groupIDsByName = map[string]string{}
	}

	clickEvents := []*OrderedMap{}
	var substitutions []interface{}

	// Find meaningful events
	meaningfulEvents := []string{}
	for _, eventName := range []string{"pressEvent", "clickEvent", "doubleClickEvent", "longPressEvent"} {
		event := getOrOrderedMap(eventRoot, eventName)
		if fclEventHasPayload(event) {
			meaningfulEvents = append(meaningfulEvents, eventName)
		}
	}

	for _, eventName := range []string{"pressEvent", "clickEvent", "doubleClickEvent", "longPressEvent"} {
		event := getOrOrderedMap(eventRoot, eventName)
		clickEvents = append(clickEvents, fclEventToZLEvents(event, strict, text, eventName, groupIDsByName, &substitutions)...)
	}
	clickEvents = normalizeZLClickEvents(clickEvents)

	pressEvent := getOrOrderedMap(eventRoot, "pressEvent")
	pressKeycodes := fclKeycodeList(getOr(pressEvent, "outputKeycodes", nil))
	canToggle := toBool(getOr(pressEvent, "autoKeep", false)) && len(pressKeycodes) > 0 && len(meaningfulEvents) == 1 && meaningfulEvents[0] == "pressEvent"

	if toBool(getOr(eventRoot, "Movable", false)) {
		reason := "FCL movable button cannot be represented in ZL layout JSON; preserved in metadata"
		warn(reason+" on button "+strconvQuote(text), strict, true)
		substitutions = append(substitutions, substitution(
			NewOrderedMapFromPairs("type", "fcl_button_flag", "key", "Movable"),
			NewOrderedMapFromPairs("type", "metadata_only"),
			reason,
			"events",
		))
	}
	if toBool(getOr(eventRoot, "pointerFollow", false)) {
		hasMouseKeycode := false
		for _, k := range pressKeycodes {
			if _, ok := FCLMouseReverse[clampInt(k)]; ok {
				hasMouseKeycode = true
				break
			}
		}
		if !hasMouseKeycode {
			reason := "FCL pointerFollow cannot be represented exactly in ZL; preserved in metadata"
			warn(reason+" on button "+strconvQuote(text), strict, true)
			substitutions = append(substitutions, substitution(
				NewOrderedMapFromPairs("type", "fcl_button_flag", "key", "pointerFollow"),
				NewOrderedMapFromPairs("type", "metadata_only"),
				reason,
				"events",
			))
		}
	}

	isDecorative := len(clickEvents) == 0
	result := NewOrderedMapFromPairs(
		"text", translatable(text),
		"uuid", toString(getOr(button, "id", shortID()+shortID()[:6])),
		"position", NewOrderedMapFromPairs(
			"x", scalePositionToZL(getOr(baseInfo, "xPosition", 0)),
			"y", scalePositionToZL(getOr(baseInfo, "yPosition", 0)),
		),
		"buttonSize", makeZLButtonSize(baseInfo, absoluteAsPercentage, aspect),
		"buttonStyle", getStyleFromMap(styleMap, toString(getOr(visualButton, "style", toString(getOr(button, "style", "Default")))), nil),
		"textAlignment", "Left",
		"textBold", false,
		"textItalic", false,
		"textUnderline", false,
		"visibilityType", visibilityFCLToZL(toString(getOr(baseInfo, "visibilityType", ""))),
		"clickEvents", clickEventsToInterface(clickEvents),
		"isSwipple", isDecorative,
		"isPenetrable", isDecorative,
		"isToggleable", canToggle,
	)

	var mapping *OrderedMap
	if visualButton != button {
		mapping = NewOrderedMapFromPairs(
			"synthetic", true,
			"generatedFrom", "overlay-merge",
			"pairedVisualId", toString(getOr(visualButton, "id", "")),
			"pairedEventId", toString(getOr(button, "id", "")),
		)
	}
	mapping = appendSubstitutions(mapping, substitutions)
	return setMeta(result, makeMeta("fcl", "button", toString(getOr(button, "id", result.GetMust("uuid"))), button, mapping))
}

// clickEventsToInterface converts []*OrderedMap to []interface{}.
func clickEventsToInterface(events []*OrderedMap) []interface{} {
	result := make([]interface{}, len(events))
	for i, e := range events {
		result[i] = e
	}
	return result
}
