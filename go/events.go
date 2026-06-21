package main

import "strings"

// fclEventHasPayload checks if an FCL event has any meaningful payload.
func fclEventHasPayload(event *OrderedMap) bool {
	if event == nil {
		return false
	}
	if len(fclKeycodeList(getOr(event, "outputKeycodes", nil))) > 0 {
		return true
	}
	if toBool(getOr(event, "input", false)) {
		return true
	}
	if toBool(getOr(event, "openMenu", false)) {
		return true
	}
	if toString(getOr(event, "outputText", "")) != "" {
		return true
	}
	if len(getOrList(event, "bindViewGroup")) > 0 {
		return true
	}
	if toBool(getOr(event, "switchTouchMode", false)) {
		return true
	}
	if toBool(getOr(event, "switchMouseMode", false)) {
		return true
	}
	if toBool(getOr(event, "quickInput", false)) {
		return true
	}
	return false
}

// fclButtonHasPayload checks if a button has any event payload.
func fclButtonHasPayload(button *OrderedMap) bool {
	eventRoot := getOrOrderedMap(button, "event")
	for _, eventName := range []string{"pressEvent", "clickEvent", "doubleClickEvent", "longPressEvent"} {
		event := getOrOrderedMap(eventRoot, eventName)
		if fclEventHasPayload(event) {
			return true
		}
	}
	return false
}

// fclButtonIsDecorative checks if a button is decorative (no payload, no pointer follow/movable).
func fclButtonIsDecorative(button *OrderedMap) bool {
	if fclButtonHasPayload(button) {
		return false
	}
	eventRoot := getOrOrderedMap(button, "event")
	if toBool(getOr(eventRoot, "pointerFollow", false)) || toBool(getOr(eventRoot, "Movable", false)) {
		return false
	}
	return true
}

// fclEventToZLEvents converts an FCL event to a list of ZL events.
func fclEventToZLEvents(
	event *OrderedMap,
	strict bool,
	label string,
	eventName string,
	groupIDsByName map[string]string,
	substitutions *[]interface{},
) []*OrderedMap {
	result := []*OrderedMap{}
	autoClick := toBool(getOr(event, "autoClick", false))
	var keycodes []int
	for _, kc := range fclKeycodeList(getOr(event, "outputKeycodes", nil)) {
		keycodes = append(keycodes, clampInt(kc))
	}

	if (eventName == "clickEvent" || eventName == "doubleClickEvent") && fclEventHasPayload(event) {
		reason := "FCL " + eventName + " has no exact ZL timing equivalent; converted to a normal ZL press/release event"
		warn(reason+" on button "+strconvQuote(label), strict, true)
		if substitutions != nil {
			*substitutions = append(*substitutions, substitution(
				NewOrderedMapFromPairs("type", "fcl_event", "event", eventName),
				NewOrderedMapFromPairs("type", "zl_click_events"),
				reason,
				"events",
			))
		}
	}
	if eventName == "longPressEvent" && fclEventHasPayload(event) {
		reason := "FCL longPressEvent has no exact ZL timing equivalent; converted to a normal event"
		warn(reason+" on button "+strconvQuote(label), strict, true)
		if substitutions != nil {
			*substitutions = append(*substitutions, substitution(
				NewOrderedMapFromPairs("type", "fcl_event", "event", eventName),
				NewOrderedMapFromPairs("type", "zl_click_events"),
				reason,
				"events",
			))
		}
	}
	if autoClick {
		hasNonScroll := false
		for _, kc := range keycodes {
			if _, ok := FCLScrollReverse[kc]; !ok {
				hasNonScroll = true
				break
			}
		}
		if hasNonScroll {
			reason := "FCL autoClick only has a ZL equivalent for scroll events; non-scroll keys are converted as normal press events"
			warn(reason, strict, true)
			if substitutions != nil {
				*substitutions = append(*substitutions, substitution(
					NewOrderedMapFromPairs("type", "fcl_auto_click", "event", eventName),
					NewOrderedMapFromPairs("type", "zl_normal_press"),
					reason,
					"events",
				))
			}
		}
	}

	for _, keycode := range keycodes {
		converted := convertKeyToZL(keycode, strict, autoClick, label, substitutions)
		if converted != nil {
			result = append(result, NewOrderedMapFromPairs("type", converted.eventType, "key", converted.key))
		}
	}
	if toBool(getOr(event, "input", false)) {
		result = append(result, NewOrderedMapFromPairs("type", "launcher_event", "key", "launcher.event.switch_ime"))
	}
	if toBool(getOr(event, "openMenu", false)) {
		result = append(result, NewOrderedMapFromPairs("type", "launcher_event", "key", "launcher.event.switch_menu"))
	}
	if toBool(getOr(event, "switchTouchMode", false)) {
		reason := "FCL switchTouchMode has no ZL equivalent; substituted with launcher menu toggle"
		warn(reason, strict, true)
		result = append(result, NewOrderedMapFromPairs("type", "launcher_event", "key", "launcher.event.switch_menu"))
		if substitutions != nil {
			*substitutions = append(*substitutions, substitution(
				NewOrderedMapFromPairs("type", "fcl_event", "key", "switchTouchMode"),
				NewOrderedMapFromPairs("type", "launcher_event", "key", "launcher.event.switch_menu"),
				reason,
				"events",
			))
		}
	}
	if toBool(getOr(event, "switchMouseMode", false)) {
		reason := "FCL switchMouseMode has no ZL equivalent; substituted with launcher menu toggle"
		warn(reason, strict, true)
		result = append(result, NewOrderedMapFromPairs("type", "launcher_event", "key", "launcher.event.switch_menu"))
		if substitutions != nil {
			*substitutions = append(*substitutions, substitution(
				NewOrderedMapFromPairs("type", "fcl_event", "key", "switchMouseMode"),
				NewOrderedMapFromPairs("type", "launcher_event", "key", "launcher.event.switch_menu"),
				reason,
				"events",
			))
		}
	}
	if toBool(getOr(event, "quickInput", false)) {
		reason := "FCL quickInput has no ZL equivalent; substituted with input method toggle"
		warn(reason, strict, true)
		result = append(result, NewOrderedMapFromPairs("type", "launcher_event", "key", "launcher.event.switch_ime"))
		if substitutions != nil {
			*substitutions = append(*substitutions, substitution(
				NewOrderedMapFromPairs("type", "fcl_event", "key", "quickInput"),
				NewOrderedMapFromPairs("type", "launcher_event", "key", "launcher.event.switch_ime"),
				reason,
				"events",
			))
		}
	}
	if outputText := toString(getOr(event, "outputText", "")); outputText != "" {
		result = append(result, NewOrderedMapFromPairs("type", "send_text", "key", outputText))
	}

	if groupIDsByName == nil {
		groupIDsByName = map[string]string{}
	}
	var bindGroups []string
	for _, groupID := range getOrList(event, "bindViewGroup") {
		bindGroups = append(bindGroups, toString(groupID))
	}

	// Check if we should suppress chat layer
	suppressChatLayer := false
	chatID := groupIDsByName["聊天"]
	if chatID != "" {
		hasKeyT := false
		for _, item := range result {
			if toString(getOr(item, "type", "")) == "key" && toString(getOr(item, "key", "")) == "GLFW_KEY_T" {
				hasKeyT = true
				break
			}
		}
		if hasKeyT {
			for _, gid := range bindGroups {
				if gid == chatID {
					suppressChatLayer = true
					break
				}
			}
		}
	}

	for _, groupID := range bindGroups {
		if suppressChatLayer && groupID == chatID {
			continue
		}
		result = append(result, NewOrderedMapFromPairs("type", "switch_layer", "key", groupID))
	}
	return result
}

// strconvQuote wraps a string in Python repr-style quotes for warning messages.
// Python uses repr() which prefers single quotes. We use %q which uses double quotes.
// For warning messages this difference is cosmetic and acceptable.
func strconvQuote(s string) string {
	return strconvQuoteImpl(s)
}

func strconvQuoteImpl(s string) string {
	// Use Go's %q which produces a double-quoted string with escapes.
	// Python repr() for simple strings without special chars produces 'string'.
	// For warning messages, the exact quoting style doesn't affect functionality.
	var b strings.Builder
	b.WriteByte('"')
	for _, r := range s {
		switch r {
		case '"':
			b.WriteString("\\\"")
		case '\\':
			b.WriteString("\\\\")
		case '\n':
			b.WriteString("\\n")
		case '\r':
			b.WriteString("\\r")
		case '\t':
			b.WriteString("\\t")
		default:
			b.WriteRune(r)
		}
	}
	b.WriteByte('"')
	return b.String()
}

// normalizeZLClickEvents dedupes events and keeps only the first send_text event.
func normalizeZLClickEvents(events []*OrderedMap) []*OrderedMap {
	deduped := dedupeEvents(events)
	var sendTextEvents []*OrderedMap
	var otherEvents []*OrderedMap
	for _, event := range deduped {
		if toString(getOr(event, "type", "")) == "send_text" && toString(getOr(event, "key", "")) != "" {
			sendTextEvents = append(sendTextEvents, event)
		} else {
			otherEvents = append(otherEvents, event)
		}
	}
	if len(sendTextEvents) > 0 {
		otherEvents = append(otherEvents, sendTextEvents[0])
	}
	return otherEvents
}
