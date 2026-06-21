package main

import (
	"math"
	"strings"
)

// rect represents a float64 rectangle (x1, y1, x2, y2).
type rect struct {
	x1, y1, x2, y2 float64
}

// fclButtonRect computes the screen rect of an FCL button.
func fclButtonRect(button *OrderedMap, aspect float64) rect {
	baseInfo := getOrOrderedMap(button, "baseInfo")
	screenH := 10000.0
	screenW := screenH * math.Max(0.1, clampFloat(aspect, 16.0/9.0))

	var width, height float64
	if toString(getOr(baseInfo, "sizeType", "")) == "ABSOLUTE" {
		width = math.Max(1.0, clampZLDP(getOr(baseInfo, "absoluteWidth", 50))*10.0)
		height = math.Max(1.0, clampZLDP(getOr(baseInfo, "absoluteHeight", 50))*10.0)
	} else {
		pw := getOrOrderedMap(baseInfo, "percentageWidth")
		ph := getOrOrderedMap(baseInfo, "percentageHeight")
		widthRef := screenW
		if toString(getOr(pw, "reference", "")) == "SCREEN_HEIGHT" {
			widthRef = screenH
		}
		heightRef := screenW
		if toString(getOr(ph, "reference", "")) == "SCREEN_HEIGHT" {
			heightRef = screenH
		}
		width = math.Max(1.0, widthRef*float64(clampInt(getOr(pw, "size", 50), 50))/1000.0)
		height = math.Max(1.0, heightRef*float64(clampInt(getOr(ph, "size", 50), 50))/1000.0)
	}

	x := (screenW - width) * float64(clampInt(getOr(baseInfo, "xPosition", 0))) / 1000.0
	y := (screenH - height) * float64(clampInt(getOr(baseInfo, "yPosition", 0))) / 1000.0
	return rect{x, y, x + width, y + height}
}

func rectArea(r rect) float64 {
	return math.Max(0.0, r.x2-r.x1) * math.Max(0.0, r.y2-r.y1)
}

func screenArea(aspect float64) float64 {
	return 10000.0 * 10000.0 * math.Max(0.1, clampFloat(aspect, 16.0/9.0))
}

func fclButtonAreaRatio(button *OrderedMap, aspect float64) float64 {
	return rectArea(fclButtonRect(button, aspect)) / math.Max(1.0, screenArea(aspect))
}

func rectOverlapArea(a, b rect) float64 {
	return math.Max(0.0, math.Min(a.x2, b.x2)-math.Max(a.x1, b.x1)) *
		math.Max(0.0, math.Min(a.y2, b.y2)-math.Max(a.y1, b.y1))
}

func rectCenter(r rect) (float64, float64) {
	return (r.x1 + r.x2) / 2.0, (r.y1 + r.y2) / 2.0
}

func rectContainsPoint(r rect, px, py float64) bool {
	return r.x1 <= px && px <= r.x2 && r.y1 <= py && py <= r.y2
}

func rectGap(a, b rect) (float64, float64) {
	horizontal := math.Max(0.0, math.Max(a.x1, b.x1)-math.Min(a.x2, b.x2))
	vertical := math.Max(0.0, math.Max(a.y1, b.y1)-math.Min(a.y2, b.y2))
	return horizontal, vertical
}

func rectDistance(a, b rect) float64 {
	horizontal, vertical := rectGap(a, b)
	return math.Hypot(horizontal, vertical)
}

func sameVisibility(a, b *OrderedMap) bool {
	aBase := getOrOrderedMap(a, "baseInfo")
	bBase := getOrOrderedMap(b, "baseInfo")
	aVis := toString(getOr(aBase, "visibilityType", "ALWAYS"))
	if aVis == "" {
		aVis = "ALWAYS"
	}
	bVis := toString(getOr(bBase, "visibilityType", "ALWAYS"))
	if bVis == "" {
		bVis = "ALWAYS"
	}
	return aVis == bVis
}

// overlayMatchScore computes how well a display button matches an event button.
func overlayMatchScore(eventButton, displayButton *OrderedMap, aspect float64) float64 {
	eventRect := fclButtonRect(eventButton, aspect)
	displayRect := fclButtonRect(displayButton, aspect)
	eventArea := rectArea(eventRect)
	displayArea := rectArea(displayRect)
	if eventArea <= 0 || displayArea <= 0 {
		return 0.0
	}

	overlap := rectOverlapArea(eventRect, displayRect)
	overlapMin := overlap / math.Max(1.0, math.Min(eventArea, displayArea))
	displayCenterX, displayCenterY := rectCenter(displayRect)
	displayCenterInEvent := rectContainsPoint(eventRect, displayCenterX, displayCenterY)
	eventCenterX, eventCenterY := rectCenter(eventRect)
	eventCenterInDisplay := rectContainsPoint(displayRect, eventCenterX, eventCenterY)
	horizontalGap, verticalGap := rectGap(eventRect, displayRect)
	eventW := eventRect.x2 - eventRect.x1
	eventH := eventRect.y2 - eventRect.y1
	displayW := displayRect.x2 - displayRect.x1
	displayH := displayRect.y2 - displayRect.y1
	verticalOverlap := math.Max(0.0, math.Min(eventRect.y2, displayRect.y2)-math.Max(eventRect.y1, displayRect.y1)) / math.Max(1.0, math.Min(eventH, displayH))
	horizontalOverlap := math.Max(0.0, math.Min(eventRect.x2, displayRect.x2)-math.Max(eventRect.x1, displayRect.x1)) / math.Max(1.0, math.Min(eventW, displayW))

	if overlapMin >= 0.25 || displayCenterInEvent || eventCenterInDisplay {
		score := 100.0 + overlapMin*100.0
		if displayCenterInEvent {
			score += 25.0
		}
		if eventCenterInDisplay {
			score += 10.0
		}
		return score
	}

	maxW := math.Max(eventW, displayW)
	maxH := math.Max(eventH, displayH)
	if verticalOverlap >= 0.65 && horizontalGap <= math.Max(250.0, maxW*0.25) {
		return 40.0 + verticalOverlap*20.0 - horizontalGap/math.Max(1.0, maxW)
	}
	if horizontalOverlap >= 0.65 && verticalGap <= math.Max(250.0, maxH*0.25) {
		return 40.0 + horizontalOverlap*20.0 - verticalGap/math.Max(1.0, maxH)
	}
	return 0.0
}

// matchFCLOverlayButtons matches event buttons with display buttons.
// Returns (event_index -> display_index mapping, consumed display indices set).
func matchFCLOverlayButtons(buttons []*OrderedMap, aspect float64) (map[int]int, map[int]struct{}) {
	displayIndices := []int{}
	eventIndices := []int{}
	for i, button := range buttons {
		if !fclButtonHasPayload(button) && strings.TrimSpace(toString(getOr(button, "text", ""))) != "" {
			displayIndices = append(displayIndices, i)
		}
		if fclButtonHasPayload(button) && strings.TrimSpace(toString(getOr(button, "text", ""))) == "" {
			eventIndices = append(eventIndices, i)
		}
	}
	matches := map[int]int{}
	consumed := map[int]struct{}{}

	for _, eventIndex := range eventIndices {
		eventButton := buttons[eventIndex]
		bestIndex := -1
		bestScore := 0.0
		for _, displayIndex := range displayIndices {
			if _, ok := consumed[displayIndex]; ok {
				continue
			}
			displayButton := buttons[displayIndex]
			if !sameVisibility(eventButton, displayButton) {
				continue
			}
			score := overlayMatchScore(eventButton, displayButton, aspect)
			if score > bestScore {
				bestScore = score
				bestIndex = displayIndex
			}
		}
		if bestIndex >= 0 && bestScore >= 40.0 {
			matches[eventIndex] = bestIndex
			consumed[bestIndex] = struct{}{}
		}
	}
	return matches, consumed
}

// fclButtonGridSignature computes a grid signature for a button.
type gridSignature struct {
	style        string
	width        int
	height       int
	visibility   string
}

func fclButtonGridSignature(button *OrderedMap) gridSignature {
	baseInfo := getOrOrderedMap(button, "baseInfo")
	pw := getOrOrderedMap(baseInfo, "percentageWidth")
	ph := getOrOrderedMap(baseInfo, "percentageHeight")
	return gridSignature{
		style:      toString(getOr(button, "style", "")),
		width:      clampInt(getOr(pw, "size", 0), 0),
		height:     clampInt(getOr(ph, "size", 0), 0),
		visibility: toString(getOr(baseInfo, "visibilityType", "ALWAYS")),
	}
}

// inferableGridIndices returns indices of buttons that are part of grids (>=4 same signature).
func inferableGridIndices(buttons []*OrderedMap) map[int]struct{} {
	buckets := map[gridSignature][]int{}
	for i, button := range buttons {
		if fclButtonHasPayload(button) || strings.TrimSpace(toString(getOr(button, "text", ""))) == "" {
			continue
		}
		sig := fclButtonGridSignature(button)
		if sig.width <= 0 || sig.height <= 0 {
			continue
		}
		buckets[sig] = append(buckets[sig], i)
	}
	result := map[int]struct{}{}
	for _, indices := range buckets {
		if len(indices) >= 4 {
			for _, idx := range indices {
				result[idx] = struct{}{}
			}
		}
	}
	return result
}

// inferEventsFromGroupNames infers switch_layer events from button text matching group names.
func inferEventsFromGroupNames(button *OrderedMap, groupIDsByName map[string]string, groupName string) []*OrderedMap {
	text := toString(getOr(button, "text", ""))
	textWords := normalizedControlWords(text)
	normalizedText := normalizedControlText(text)
	if len(textWords) == 0 && normalizedText == "" {
		return nil
	}

	var matches []groupMatch
	groupPrefix := normalizedControlText(groupName)
	for candidateName, groupID := range groupIDsByName {
		if groupID == "" || candidateName == groupName {
			continue
		}
		candidateWords := normalizedControlWords(candidateName)
		normalizedCandidate := normalizedControlText(candidateName)
		if len(candidateWords) == 0 && normalizedCandidate == "" {
			continue
		}
		// Check if candidateWords is subset of textWords
		candidateSubset := false
		if len(candidateWords) > 0 {
			candidateSubset = true
			for w := range candidateWords {
				if _, ok := textWords[w]; !ok {
					candidateSubset = false
					break
				}
			}
		}
		if (candidateSubset) || (normalizedCandidate != "" && strings.Contains(normalizedText, normalizedCandidate)) {
			prefixScore := 0
			if groupPrefix != "" && strings.HasPrefix(normalizedCandidate, groupPrefix) {
				prefixScore = 1
			}
			matches = append(matches, groupMatch{
				prefixScore:   prefixScore,
				normalizedLen: len([]rune(normalizedCandidate)),
				candidateLen:  len([]rune(candidateName)),
				groupID:       groupID,
			})
		}
	}

	// Sort matches reverse=True (descending by prefixScore, normalizedLen, candidateLen)
	sortMatchesReverse(matches)
	if len(matches) == 0 {
		return nil
	}
	// Take only the first match
	result := []*OrderedMap{
		NewOrderedMapFromPairs("type", "switch_layer", "key", matches[0].groupID),
	}
	return dedupeEvents(result)
}

// inferBuiltinMenuEvents infers events from known button text patterns.
func inferBuiltinMenuEvents(button *OrderedMap) []*OrderedMap {
	text := normalizedControlText(toString(getOr(button, "text", "")))
	if text == "fcl菜单" || text == "菜单" {
		return []*OrderedMap{NewOrderedMapFromPairs("type", "launcher_event", "key", "launcher.event.switch_menu")}
	}
	if text == "输入法" || text == "输入文字" {
		return []*OrderedMap{NewOrderedMapFromPairs("type", "launcher_event", "key", "launcher.event.switch_ime")}
	}
	if text == "社交" {
		return []*OrderedMap{NewOrderedMapFromPairs("type", "key", "key", "GLFW_KEY_P")}
	}
	if text == "聊天" {
		return []*OrderedMap{NewOrderedMapFromPairs("type", "key", "key", "GLFW_KEY_T")}
	}
	return nil
}

// eventBindTargets returns the set of bindViewGroup targets across all event types.
func eventBindTargets(button *OrderedMap) map[string]struct{} {
	targets := map[string]struct{}{}
	eventRoot := getOrOrderedMap(button, "event")
	for _, eventName := range []string{"pressEvent", "clickEvent", "doubleClickEvent", "longPressEvent"} {
		event := getOrOrderedMap(eventRoot, eventName)
		for _, groupID := range getOrList(event, "bindViewGroup") {
			targets[toString(groupID)] = struct{}{}
		}
	}
	return targets
}

// layerEventTargets returns all bind targets for buttons in a group.
func layerEventTargets(group *OrderedMap) map[string]struct{} {
	targets := map[string]struct{}{}
	viewData := getOrOrderedMap(group, "viewData")
	for _, item := range getOrList(viewData, "buttonList") {
		if button, ok := item.(*OrderedMap); ok {
			for t := range eventBindTargets(button) {
				targets[t] = struct{}{}
			}
		}
	}
	return targets
}

// inferReciprocalLayerOpeners infers which decorative buttons open which layers.
func inferReciprocalLayerOpeners(data *OrderedMap, aspect float64) map[string]string {
	var groups []*OrderedMap
	for _, item := range getOrList(data, "viewGroups") {
		if g, ok := item.(*OrderedMap); ok {
			groups = append(groups, g)
		}
	}
	openerScores := map[string]struct {
		score    float64
		targetID string
	}{}

	groupIndex := map[string]int{}
	for i, group := range groups {
		groupIndex[toString(getOr(group, "id", ""))] = i
	}
	groupIDsByName := map[string]string{}
	for _, group := range groups {
		id := toString(getOr(group, "id", ""))
		if id == "" {
			continue
		}
		name := toString(getOr(group, "name", "Layer"))
		groupIDsByName[name] = id
	}
	targetsByGroupID := map[string]map[string]struct{}{}
	for _, group := range groups {
		id := toString(getOr(group, "id", ""))
		targetsByGroupID[id] = layerEventTargets(group)
	}

	for _, sourceGroup := range groups {
		sourceID := toString(getOr(sourceGroup, "id", ""))
		viewData := getOrOrderedMap(sourceGroup, "viewData")
		var sourceButtons []*OrderedMap
		for _, item := range getOrList(viewData, "buttonList") {
			if b, ok := item.(*OrderedMap); ok {
				sourceButtons = append(sourceButtons, b)
			}
		}
		var candidates []*OrderedMap
		for _, button := range sourceButtons {
			if !fclButtonHasPayload(button) &&
				strings.TrimSpace(toString(getOr(button, "text", ""))) != "" &&
				fclButtonAreaRatio(button, aspect) < 0.05 {
				candidates = append(candidates, button)
			}
		}
		if len(candidates) == 0 {
			continue
		}

		for _, candidate := range candidates {
			inferredEvents := inferEventsFromGroupNames(candidate, groupIDsByName, toString(getOr(sourceGroup, "name", "")))
			for _, event := range inferredEvents {
				targetID := toString(getOr(event, "key", ""))
				if targetID != "" && targetID != sourceID {
					indexDistance := absInt(groupIndex[targetID] - groupIndex[sourceID])
					buttonID := toString(getOr(candidate, "id", ""))
					score := float64(indexDistance)*10000.0 - 1.0
					if prev, ok := openerScores[buttonID]; !ok || score < prev.score {
						openerScores[buttonID] = struct {
							score    float64
							targetID string
						}{score, targetID}
					}
				}
			}
		}

		for _, targetGroup := range groups {
			targetID := toString(getOr(targetGroup, "id", ""))
			if targetID == "" || targetID == sourceID {
				continue
			}
			if toString(getOr(targetGroup, "visibility", "")) != "INVISIBLE" {
				continue
			}
			sourceWords := normalizedControlWords(toString(getOr(sourceGroup, "name", "")))
			targetWords := normalizedControlWords(toString(getOr(targetGroup, "name", "")))
			sourceTargets := targetsByGroupID[sourceID]
			// Check if sourceWords and targetWords have intersection
			hasIntersection := false
			if len(sourceWords) > 0 && len(targetWords) > 0 {
				for w := range sourceWords {
					if _, ok := targetWords[w]; ok {
						hasIntersection = true
						break
					}
				}
			}
			if hasIntersection {
				if _, ok := sourceTargets[targetID]; !ok {
					continue
				}
			}

			viewData := getOrOrderedMap(targetGroup, "viewData")
			var targetButtons []*OrderedMap
			for _, item := range getOrList(viewData, "buttonList") {
				if b, ok := item.(*OrderedMap); ok {
					targetButtons = append(targetButtons, b)
				}
			}
			var closeButtons []*OrderedMap
			for _, button := range targetButtons {
				bindTargets := eventBindTargets(button)
				_, hasSource := bindTargets[sourceID]
				_, hasTarget := bindTargets[targetID]
				ratio := fclButtonAreaRatio(button, aspect)
				if hasSource && hasTarget && ratio >= 0.08 && ratio <= 0.50 {
					closeButtons = append(closeButtons, button)
				}
			}
			if len(closeButtons) == 0 {
				continue
			}

			var bestCandidate *OrderedMap
			bestDistance := math.Inf(1)
			for _, candidate := range candidates {
				candidateRect := fclButtonRect(candidate, aspect)
				for _, closeButton := range closeButtons {
					distance := rectDistance(candidateRect, fclButtonRect(closeButton, aspect))
					if distance < bestDistance {
						bestDistance = distance
						bestCandidate = candidate
					}
				}
			}
			if bestCandidate != nil && bestDistance <= 500.0 {
				buttonID := toString(getOr(bestCandidate, "id", ""))
				indexDistance := absInt(groupIndex[targetID] - groupIndex[sourceID])
				score := float64(indexDistance)*10000.0 + bestDistance
				if prev, ok := openerScores[buttonID]; !ok || score < prev.score {
					openerScores[buttonID] = struct {
						score    float64
						targetID string
					}{score, targetID}
				}
			}
		}
	}

	result := map[string]string{}
	for buttonID, v := range openerScores {
		result[buttonID] = v.targetID
	}
	return result
}

func absInt(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

// groupMatch represents a candidate group match for event inference.
type groupMatch struct {
	prefixScore   int
	normalizedLen int
	candidateLen  int
	groupID       string
}

// sortMatchesReverse sorts matches in descending order (Python: matches.sort(reverse=True)).
func sortMatchesReverse(matches []groupMatch) {
	// Simple insertion sort, descending
	for i := 1; i < len(matches); i++ {
		for j := i; j > 0; j-- {
			a := matches[j]
			b := matches[j-1]
			// Compare descending: prefixScore, then normalizedLen, then candidateLen
			if a.prefixScore > b.prefixScore ||
				(a.prefixScore == b.prefixScore && a.normalizedLen > b.normalizedLen) ||
				(a.prefixScore == b.prefixScore && a.normalizedLen == b.normalizedLen && a.candidateLen > b.candidateLen) {
				matches[j], matches[j-1] = matches[j-1], matches[j]
			} else {
				break
			}
		}
	}
}
