package main

import (
	"math"
	"sort"
)

// fclToZL converts an FCL layout to a ZL layout.
func fclToZL(data *OrderedMap, includeDirections bool, strict bool, aspect float64, lossless bool, absoluteAsPercentage bool) *OrderedMap {
	includeDirections = includeDirections || lossless
	rootOriginal := metaOriginal(data, "zl", "layout")

	var stylesList []interface{}
	if bl := getOrList(data, "buttonStyles"); len(bl) > 0 {
		stylesList = bl
	} else {
		stylesList = []interface{}{defaultFCLStyle()}
	}
	styles, styleMap := fclStylesToZL(stylesList)

	var dirStylesInput []interface{}
	if ds := getOrList(data, "directionStyles"); len(ds) > 0 {
		dirStylesInput = ds
	} else {
		dirStylesInput = []interface{}{defaultFCLDirectionStyle()}
	}
	directionStyles := directionStyleMap(dirStylesInput)

	defaultStyleUUID := ""
	for _, uuid := range styleMap {
		defaultStyleUUID = uuid
		break
	}

	layers := []*OrderedMap{}
	var special *OrderedMap
	if rootOriginal != nil {
		if ro, ok := rootOriginal.(*OrderedMap); ok {
			if sp, ok := ro.Get("special"); ok {
				special = deepCopyJSON(sp).(*OrderedMap)
			}
		}
	}
	if special == nil {
		special = NewOrderedMap()
	}
	warnedJoystickSettings := false

	groupIDsByName := map[string]string{}
	for _, item := range getOrList(data, "viewGroups") {
		if group, ok := item.(*OrderedMap); ok {
			id := toString(getOr(group, "id", ""))
			if id != "" {
				name := toString(getOr(group, "name", "Layer"))
				groupIDsByName[name] = id
			}
		}
	}

	reciprocalOpeners := inferReciprocalLayerOpeners(data, aspect)

	// Iterate over reversed viewGroups
	viewGroups := getOrList(data, "viewGroups")
	for i := len(viewGroups) - 1; i >= 0; i-- {
		group, ok := viewGroups[i].(*OrderedMap)
		if !ok {
			continue
		}
		layerOriginal := metaOriginal(group, "zl", "layer")
		viewData := getOrOrderedMap(group, "viewData")
		groupName := toString(getOr(group, "name", "Layer"))
		if groupName == "" {
			groupName = "Layer"
		}

		var buttons []*OrderedMap
		var textBoxes []*OrderedMap
		fclButtons := []*OrderedMap{}
		for _, item := range getOrList(viewData, "buttonList") {
			if b, ok := item.(*OrderedMap); ok {
				fclButtons = append(fclButtons, b)
			}
		}

		overlayMatches, consumedDisplayIndices := matchFCLOverlayButtons(fclButtons, aspect)
		gridIndices := inferableGridIndices(fclButtons)

		for index, button := range fclButtons {
			if _, consumed := consumedDisplayIndices[index]; consumed {
				continue
			}
			hasPayload := fclButtonHasPayload(button)
			if hasPayload {
				var visualButton *OrderedMap
				if matchIdx, ok := overlayMatches[index]; ok {
					visualButton = fclButtons[matchIdx]
				}
				convertedButton := fclButtonToZL(
					button,
					styleMap,
					strict,
					groupName,
					groupIDsByName,
					visualButton,
					absoluteAsPercentage,
					aspect,
				)
				if len(getOrList(convertedButton, "clickEvents")) > 0 {
					buttons = append(buttons, convertedButton)
				} else {
					vb := visualButton
					if vb == nil {
						vb = button
					}
					textBoxes = append(textBoxes, fclButtonToZLTextbox(vb, styleMap, absoluteAsPercentage, aspect))
				}
			} else {
				buttonID := toString(getOr(button, "id", ""))
				openerTarget := reciprocalOpeners[buttonID]
				var inferredEvents []*OrderedMap
				if openerTarget != "" {
					inferredEvents = []*OrderedMap{NewOrderedMapFromPairs("type", "switch_layer", "key", openerTarget)}
				}
				if len(inferredEvents) == 0 {
					if _, inGrid := gridIndices[index]; inGrid {
						inferredEvents = inferEventsFromGroupNames(button, groupIDsByName, groupName)
					}
				}
				if len(inferredEvents) == 0 {
					if _, inGrid := gridIndices[index]; inGrid {
						inferredEvents = inferBuiltinMenuEvents(button)
					}
				}
				if len(inferredEvents) > 0 {
					inferredButton := fclButtonToZL(
						button,
						styleMap,
						strict,
						groupName,
						groupIDsByName,
						nil,
						absoluteAsPercentage,
						aspect,
					)
					inferredButton.Set("clickEvents", clickEventsToInterface(inferredEvents))
					inferredButton.Set("isSwipple", false)
					inferredButton.Set("isPenetrable", false)
					buttons = append(buttons, inferredButton)
				} else {
					buttons = append(buttons, fclButtonToZL(
						button,
						styleMap,
						strict,
						groupName,
						groupIDsByName,
						nil,
						absoluteAsPercentage,
						aspect,
					))
				}
			}
		}

		directions := getOrList(viewData, "directionList")
		var directionButtons []*OrderedMap
		if len(directions) > 0 && !includeDirections {
			warn("skipped "+itoa(len(directions))+" FCL direction control(s) in group "+strconvQuote(toString(getOr(group, "name", "")))+"; use --include-directions to convert them", strict, false)
		}
		if includeDirections {
			for _, dirItem := range directions {
				direction, ok := dirItem.(*OrderedMap)
				if !ok {
					continue
				}
				directionStyle := resolveDirectionStyle(direction, directionStyles)
				isRocker := toString(getOr(directionStyle, "styleType", "")) == "ROCKER"
				if isRocker {
					if !special.Has("joystickStyle") {
						special.Set("joystickStyle", fclRockerStyleToZLJoystick(directionStyle))
					}
					if !warnedJoystickSettings {
						warn("converted FCL ROCKER style to ZL special.joystickStyle and approximated rocker controls as 8-way button grid", strict, false)
						warnedJoystickSettings = true
					}
				}
				substitutionCounts["directions"]++
				directionButtons = append(directionButtons, directionToZLButtons(direction, directionStyle, defaultStyleUUID, strict, aspect, isRocker)...)
			}
		}

		// Sort buttons by area ratio (descending)
		sort.SliceStable(buttons, func(i, j int) bool {
			ratioI := zlButtonAreaRatio(buttons[i], aspect)
			ratioJ := zlButtonAreaRatio(buttons[j], aspect)
			return ratioI > ratioJ
		})

		// Prepend direction buttons
		buttons = append(directionButtons, buttons...)

		// Build layer object
		var layerObj *OrderedMap
		if layerOriginal != nil {
			if lo, ok := layerOriginal.(*OrderedMap); ok {
				layerObj = deepCopyJSON(lo).(*OrderedMap)
			}
		}
		if layerObj == nil {
			layerObj = NewOrderedMap()
		}
		layerObj.Set("name", groupName)
		layerObj.Set("uuid", toString(getOr(group, "id", getOr(layerObj, "uuid", shortID()))))
		layerObj.Set("hide", toString(getOr(group, "visibility", "")) == "INVISIBLE")
		layerObj.Set("hideWhenMouse", toBool(getOr(layerObj, "hideWhenMouse", false)))
		layerObj.Set("hideWhenGamepad", toBool(getOr(layerObj, "hideWhenGamepad", false)))
		layerObj.Set("hideWhenJoystick", toBool(getOr(layerObj, "hideWhenJoystick", false)))
		layerObj.Set("visibilityType", toString(getOr(layerObj, "visibilityType", "always")))
		if layerObj.GetMust("visibilityType") == "" {
			layerObj.Set("visibilityType", "always")
		}
		layerObj.Set("normalButtons", buttonsToInterface(buttons))
		layerObj.Set("textBoxes", buttonsToInterface(textBoxes))

		layerObj = setMeta(layerObj, makeMeta("fcl", "viewGroup", toString(getOr(group, "id", layerObj.GetMust("uuid"))), group))
		layers = append(layers, layerObj)
	}

	// Filter clickEvents that reference non-existent layers
	layerIDs := map[string]struct{}{}
	for _, layer := range layers {
		layerIDs[toString(getOr(layer, "uuid", ""))] = struct{}{}
	}
	for _, layer := range layers {
		normalButtons := getOrList(layer, "normalButtons")
		for _, item := range normalButtons {
			if button, ok := item.(*OrderedMap); ok {
				clickEvents := getOrList(button, "clickEvents")
				filtered := []interface{}{}
				for _, eventItem := range clickEvents {
					event, ok := eventItem.(*OrderedMap)
					if !ok {
						filtered = append(filtered, eventItem)
						continue
					}
					eventType := toString(getOr(event, "type", ""))
					eventKey := toString(getOr(event, "key", ""))
					if eventType == "switch_layer" || eventType == "show_layer" || eventType == "hide_layer" {
						if _, exists := layerIDs[eventKey]; !exists {
							continue
						}
					}
					filtered = append(filtered, eventItem)
				}
				button.Set("clickEvents", filtered)
				if len(filtered) == 0 {
					button.Set("isSwipple", true)
					button.Set("isPenetrable", true)
					button.Set("isToggleable", false)
				}
			}
		}
	}

	// Build result
	var result *OrderedMap
	if rootOriginal != nil {
		if ro, ok := rootOriginal.(*OrderedMap); ok {
			result = deepCopyJSON(ro).(*OrderedMap)
		}
	}
	if result == nil {
		result = NewOrderedMap()
	}

	resultInfo := getOrOrderedMap(result, "info")
	result.Set("info", NewOrderedMapFromPairs(
		"name", translatable(toString(getOr(data, "name", "Converted from FCL")), getOr(resultInfo, "name", nil)),
		"author", translatable(toString(getOr(data, "author", "")), getOr(resultInfo, "author", nil)),
		"description", translatable(toString(getOr(data, "description", "")), getOr(resultInfo, "description", nil)),
		"versionCode", maxInt(0, clampInt(getOr(data, "versionCode", clampInt(getOr(resultInfo, "versionCode", 1), 1)))),
		"versionName", toString(getOr(data, "version", getOr(resultInfo, "versionName", "1.0"))),
	))

	layersInterface := make([]interface{}, len(layers))
	for i, l := range layers {
		layersInterface[i] = l
	}
	result.Set("layers", layersInterface)

	// Styles: use root_original styles if they exist and are a non-empty list, otherwise use converted styles
	if existingStyles, ok := result.Get("styles"); ok {
		if sl, ok := existingStyles.([]interface{}); ok && len(sl) > 0 {
			result.Set("styles", deepCopyJSON(sl))
		} else {
			result.Set("styles", deepCopyJSON(styles))
		}
	} else {
		result.Set("styles", deepCopyJSON(styles))
	}

	result.Set("editorVersion", clampInt(getOr(result, "editorVersion", ZLEditorVersion), ZLEditorVersion))

	if special.Len() > 0 {
		result.Set("special", special)
	}

	resultID := toString(getOr(data, "id", ""))
	if resultID == "" {
		resultID = toString(getOr(getOrOrderedMap(result, "info"), "name", ""))
	}
	if resultID == "" {
		resultID = shortID()
	}
	return setMeta(result, makeMeta("fcl", "controller", resultID, data))
}

// zlButtonAreaRatio computes the area ratio of a ZL button (for sorting).
// This mirrors the Python inline lambda that builds a temporary FCL baseInfo.
func zlButtonAreaRatio(button *OrderedMap, aspect float64) float64 {
	pos := getOrOrderedMap(button, "position")
	buttonSize := getOrOrderedMap(button, "buttonSize")
	tempButton := NewOrderedMapFromPairs(
		"baseInfo", NewOrderedMapFromPairs(
			"xPosition", float64(clampInt(getOr(pos, "x", 0)))/10.0,
			"yPosition", float64(clampInt(getOr(pos, "y", 0)))/10.0,
			"sizeType", "PERCENTAGE",
			"percentageWidth", NewOrderedMapFromPairs("reference", "SCREEN_WIDTH", "size", float64(clampInt(getOr(buttonSize, "widthPercentage", 0)))/10.0),
			"percentageHeight", NewOrderedMapFromPairs("reference", "SCREEN_WIDTH", "size", float64(clampInt(getOr(buttonSize, "heightPercentage", 0)))/10.0),
		),
	)
	return fclButtonAreaRatio(tempButton, aspect)
}

// buttonsToInterface converts []*OrderedMap to []interface{}.
func buttonsToInterface(buttons []*OrderedMap) []interface{} {
	result := make([]interface{}, len(buttons))
	for i, b := range buttons {
		result[i] = b
	}
	return result
}

// normalizeZLLayout fills fields required by ZL models without changing semantics.
func normalizeZLLayout(layout *OrderedMap) *OrderedMap {
	result := deepCopyJSON(layout).(*OrderedMap)
	result.SetIfAbsent("special", NewOrderedMap())
	for _, item := range getOrList(result, "layers") {
		layer, ok := item.(*OrderedMap)
		if !ok {
			continue
		}
		layer.SetIfAbsent("hideWhenMouse", true)
		layer.SetIfAbsent("hideWhenGamepad", true)
		layer.SetIfAbsent("hideWhenJoystick", false)
		layer.SetIfAbsent("normalButtons", []interface{}{})
		layer.SetIfAbsent("textBoxes", []interface{}{})
	}
	return result
}

// convertFCLToZL is the main conversion entry point for fcl2zl mode.
func convertFCLToZL(data *OrderedMap, includeDirections bool, strict bool, aspect float64, lossless bool, absoluteAsPercentage bool) *OrderedMap {
	return normalizeZLLayout(fclToZL(data, includeDirections, strict, aspect, lossless, absoluteAsPercentage))
}

// Ensures math is used
var _ = math.Max
