package main

import (
	"crypto/rand"
	"encoding/json"
	"fmt"
	"math"
	"os"
	"regexp"
	"strconv"
	"strings"
)

// itoa converts int to string.
func itoa(i int) string {
	return strconv.Itoa(i)
}

// PyFloat is a float64 that marshals to JSON like Python's float repr:
// whole numbers get a trailing ".0" (e.g., 50.0, 1.0, 0.0) to match
// Python's json.dumps behavior for float values.
type PyFloat float64

// MarshalJSON implements json.Marshaler.
// For whole numbers within a reasonable range, outputs "X.0" format.
// For other values, uses the shortest round-trippable representation.
func (f PyFloat) MarshalJSON() ([]byte, error) {
	v := float64(f)
	if math.IsNaN(v) || math.IsInf(v, 0) {
		return []byte("null"), nil
	}
	if v == math.Trunc(v) && math.Abs(v) < 1e16 {
		return []byte(strconv.FormatFloat(v, 'f', 1, 64)), nil
	}
	return []byte(strconv.FormatFloat(v, 'g', -1, 64)), nil
}

// warn prints a warning to stderr, or raises an error in strict mode.
// In once mode, each message is printed only once.
func warn(message string, strict bool, once bool) {
	if strict {
		panic(fmt.Errorf("%s", message))
	}
	if once {
		if _, ok := warnedMessages[message]; ok {
			return
		}
		warnedMessages[message] = struct{}{}
	}
	fmt.Fprintf(os.Stderr, "warning: %s\n", message)
}

// deepCopyJSON performs a deep copy by round-tripping through JSON.
// Uses decodeJSON to ensure all objects become *OrderedMap.
func deepCopyJSON(value interface{}) interface{} {
	if value == nil {
		return nil
	}
	b, err := json.Marshal(value)
	if err != nil {
		return nil
	}
	result, err := decodeJSON(b)
	if err != nil {
		return nil
	}
	return result
}

// toFloat converts interface{} to float64, returning ok=false on failure.
func toFloat(v interface{}) (float64, bool) {
	switch x := v.(type) {
	case nil:
		return 0, false
	case float64:
		return x, true
	case float32:
		return float64(x), true
	case int:
		return float64(x), true
	case int64:
		return float64(x), true
	case json.Number:
		f, err := x.Float64()
		return f, err == nil
	case string:
		f, err := strconv.ParseFloat(x, 64)
		return f, err == nil
	case bool:
		if x {
			return 1, true
		}
		return 0, true
	}
	return 0, false
}

// toInt converts interface{} to int, returning ok=false on failure.
func toInt(v interface{}) (int, bool) {
	switch x := v.(type) {
	case nil:
		return 0, false
	case int:
		return x, true
	case int64:
		return int(x), true
	case float64:
		return int(x), true
	case json.Number:
		i, err := x.Int64()
		return int(i), err == nil
	case string:
		i, err := strconv.Atoi(x)
		return i, err == nil
	case bool:
		if x {
			return 1, true
		}
		return 0, true
	}
	return 0, false
}

// toString converts interface{} to string.
func toString(v interface{}) string {
	if v == nil {
		return ""
	}
	switch x := v.(type) {
	case string:
		return x
	case bool:
		if x {
			return "True"
		}
		return "False"
	default:
		return fmt.Sprintf("%v", v)
	}
}

// toBool converts interface{} to bool (Python truthiness).
func toBool(v interface{}) bool {
	if v == nil {
		return false
	}
	switch x := v.(type) {
	case bool:
		return x
	case float64:
		return x != 0
	case int:
		return x != 0
	case int64:
		return x != 0
	case json.Number:
		s := x.String()
		return s != "0" && s != "0.0" && s != ""
	case string:
		return x != ""
	case []interface{}:
		return len(x) > 0
	case *OrderedMap:
		return x.Len() > 0
	case map[string]interface{}:
		return len(x) > 0
	}
	return true
}

// clampInt converts value to int (rounding), returns default on failure.
func clampInt(value interface{}, defaultVal ...int) int {
	def := 0
	if len(defaultVal) > 0 {
		def = defaultVal[0]
	}
	f, ok := toFloat(value)
	if !ok {
		return def
	}
	return int(math.Round(f))
}

// clampFloat converts value to float64, returns default on failure or non-finite.
func clampFloat(value interface{}, defaultVal ...float64) float64 {
	def := 0.0
	if len(defaultVal) > 0 {
		def = defaultVal[0]
	}
	f, ok := toFloat(value)
	if !ok {
		return def
	}
	if math.IsNaN(f) || math.IsInf(f, 0) {
		return def
	}
	return f
}

// clampRange clamps value to [min, max].
func clampRange(value interface{}, minimum, maximum, defaultVal float64) float64 {
	return math.Max(minimum, math.Min(maximum, clampFloat(value, defaultVal)))
}

// clampZLDP clamps a dp value to >= 5.0.
func clampZLDP(value interface{}, defaultVal ...float64) float64 {
	def := 50.0
	if len(defaultVal) > 0 {
		def = defaultVal[0]
	}
	return math.Max(5.0, clampFloat(value, def))
}

// clampZLShape clamps a shape value to [0, 100].
func clampZLShape(value interface{}, defaultVal ...float64) float64 {
	def := 0.0
	if len(defaultVal) > 0 {
		def = defaultVal[0]
	}
	return clampRange(value, 0.0, 100.0, def)
}

// clampZLBorderWidth clamps border width to [0, 50].
func clampZLBorderWidth(value interface{}, defaultVal ...int) int {
	def := 0
	if len(defaultVal) > 0 {
		def = defaultVal[0]
	}
	return maxInt(0, minInt(50, clampInt(value, def)))
}

func maxInt(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func minInt(a, b int) int {
	if a < b {
		return a
	}
	return b
}

// scalePositionToFCL: FCL uses 0..1000, ZL uses 0..10000.
func scalePositionToFCL(value interface{}) int {
	return maxInt(0, minInt(1000, clampInt(clampInt(value)/10)))
}

// scalePositionToZL: FCL uses 0..1000, ZL uses 0..10000.
func scalePositionToZL(value interface{}) int {
	return maxInt(0, minInt(10000, clampInt(clampInt(value)*10)))
}

func zlRefToFCL(ref string) string {
	if ref == "screen_height" {
		return "SCREEN_HEIGHT"
	}
	return "SCREEN_WIDTH"
}

func fclRefToZL(ref string) string {
	if ref == "SCREEN_HEIGHT" {
		return "screen_height"
	}
	return "screen_width"
}

func visibilityZLToFCL(value string) string {
	if value == "" {
		value = "always"
	}
	switch value {
	case "always":
		return "ALWAYS"
	case "in_game":
		return "IN_GAME"
	case "menu", "in_menu":
		return "MENU"
	}
	return "ALWAYS"
}

func visibilityFCLToZL(value string) string {
	if value == "" {
		value = "ALWAYS"
	}
	switch value {
	case "ALWAYS":
		return "always"
	case "IN_GAME":
		return "in_game"
	case "MENU":
		return "in_menu"
	}
	return "always"
}

// textDefault extracts default text from a value (dict or scalar).
func textDefault(value interface{}) string {
	if m, ok := value.(*OrderedMap); ok {
		if d, ok := m.Get("default"); ok {
			return toString(d)
		}
		return ""
	}
	if value == nil {
		return ""
	}
	return toString(value)
}

// translatable builds a translatable text object.
func translatable(text string, source ...interface{}) *OrderedMap {
	if len(source) > 0 && source[0] != nil {
		if m, ok := source[0].(*OrderedMap); ok {
			defaultVal, _ := m.Get("default")
			d := toString(defaultVal)
			if d == "" {
				d = text
			}
			if mq, ok := m.Get("matchQueue"); ok {
				if mqList, ok := mq.([]interface{}); ok {
					return NewOrderedMapFromPairs(
						"default", d,
						"matchQueue", deepCopyJSON(mqList),
					)
				}
			}
			return NewOrderedMapFromPairs(
				"default", d,
				"matchQueue", []interface{}{},
			)
		}
	}
	return NewOrderedMapFromPairs(
		"default", text,
		"matchQueue", []interface{}{},
	)
}

// signedInt32 converts to signed 32-bit int.
func signedInt32(value int64) int64 {
	value &= 0xFFFFFFFF
	if value >= 0x80000000 {
		return value - 0x100000000
	}
	return value
}

// applyARGBAlpha scales alpha channel of an ARGB color.
func applyARGBAlpha(color int64, alpha interface{}) int64 {
	alphaValue := clampRange(alpha, 0.0, 1.0, 1.0)
	if alphaValue >= 0.999 {
		return color
	}
	argb := color & 0xFFFFFFFF
	a := (argb >> 24) & 0xFF
	a = int64(math.Max(0, math.Min(255, math.Round(float64(a)*alphaValue))))
	return signedInt32((a << 24) | (argb & 0x00FFFFFF))
}

// zlColorToFCL converts ZL color (packed Long) to FCL ARGB int.
func zlColorToFCL(color interface{}, fallback int64, alpha ...interface{}) int64 {
	alphaVal := 1.0
	if len(alpha) > 0 {
		alphaVal = clampFloat(alpha[0], 1.0)
	}
	if c, ok := toInt64(color); ok {
		// Use uint64 for bit manipulation to handle full 64-bit range
		packed := uint64(c)
		argb := int64((packed >> 32) & 0xFFFFFFFF)
		if argb != 0 || c == 0 {
			return applyARGBAlpha(signedInt32(argb), alphaVal)
		}
		if c >= -2147483648 && c <= 2147483647 {
			return applyARGBAlpha(c, alphaVal)
		}
	}
	return applyARGBAlpha(fallback, alphaVal)
}

// toInt64 converts interface{} to int64.
func toInt64(v interface{}) (int64, bool) {
	switch x := v.(type) {
	case nil:
		return 0, false
	case int:
		return int64(x), true
	case int64:
		return x, true
	case float64:
		return int64(x), true
	case json.Number:
		i, err := x.Int64()
		return i, err == nil
	case string:
		i, err := strconv.ParseInt(x, 10, 64)
		return i, err == nil
	case bool:
		if x {
			return 1, true
		}
		return 0, true
	}
	return 0, false
}

// fclARGBToZLColor converts FCL ARGB int to ZL packed Long.
func fclARGBToZLColor(color interface{}, fallback ...int64) int64 {
	def := int64(0)
	if len(fallback) > 0 {
		def = fallback[0]
	}
	// Use uint64 for the shift to avoid overflow
	value := uint64(clampInt(color, int(def))) & 0xFFFFFFFF
	packed := value << 32
	// int64() conversion handles the signed representation automatically:
	// if packed >= 2^63, int64(packed) gives the negative two's complement value,
	// which matches Python's `packed - (1 << 64)`.
	return int64(packed)
}

// fclFontToZL clamps font size to [2, 30].
func fclFontToZL(value interface{}, defaultVal ...int) int {
	def := 12
	if len(defaultVal) > 0 {
		def = defaultVal[0]
	}
	return maxInt(2, minInt(30, clampInt(value, def)))
}

// fclRadiusToZLPercent converts FCL corner radius to ZL percent [0, 50].
func fclRadiusToZLPercent(value interface{}, defaultVal ...int) int {
	def := 500
	if len(defaultVal) > 0 {
		def = defaultVal[0]
	}
	return maxInt(0, minInt(50, clampInt(value, def)/10))
}

// fclRatioToZL converts FCL ratio (0..1000) to ZL float (0..1).
func fclRatioToZL(value interface{}, defaultVal ...int) float64 {
	def := 500
	if len(defaultVal) > 0 {
		def = defaultVal[0]
	}
	return math.Max(0.0, math.Min(1.0, float64(clampInt(value, def))/1000.0))
}

// fclSizeToZL converts FCL percentage size to ZL percentage size.
func fclSizeToZL(value interface{}) int {
	return maxInt(100, minInt(10000, clampInt(clampInt(value, 50)*10)))
}

// fclKeycodeList normalizes a keycode value to a list.
func fclKeycodeList(value interface{}) []interface{} {
	if l, ok := value.([]interface{}); ok {
		return l
	}
	if value == nil {
		return []interface{}{}
	}
	return []interface{}{value}
}

// stripConverterMeta recursively removes META_KEY from dicts/lists.
func stripConverterMeta(value interface{}) interface{} {
	switch x := value.(type) {
	case *OrderedMap:
		result := NewOrderedMap()
		for _, key := range x.Keys() {
			if key == META_KEY {
				continue
			}
			v, _ := x.Get(key)
			result.Set(key, stripConverterMeta(v))
		}
		return result
	case map[string]interface{}:
		result := NewOrderedMap()
		for k, v := range x {
			if k == META_KEY {
				continue
			}
			result.Set(k, stripConverterMeta(v))
		}
		return result
	case []interface{}:
		result := make([]interface{}, len(x))
		for i, item := range x {
			result[i] = stripConverterMeta(item)
		}
		return result
	}
	return value
}

// normalizeZLKey normalizes a ZL key name using aliases.
func normalizeZLKey(eventKey string) string {
	key := strings.TrimSpace(eventKey)
	upperKey := strings.ToUpper(key)
	if strings.HasPrefix(upperKey, "GLFW_") || strings.HasPrefix(upperKey, "MOUSE_") {
		key = upperKey
	}
	if alias, ok := ZLKeyAliases[key]; ok {
		return alias
	}
	return key
}

// convertKeyToFCL converts a ZL key name to FCL keycode.
func convertKeyToFCL(eventKey string, strict bool, substitutions *[]interface{}) int {
	eventKey = normalizeZLKey(eventKey)
	if kc, ok := FCLMouse[eventKey]; ok {
		return kc
	}
	if kc, ok := GLFWToFCL[eventKey]; ok {
		return kc
	}
	if fb, ok := ZLToFCLFallbacks[eventKey]; ok {
		warn(fmt.Sprintf("ZL key event %q has no exact FCL equivalent; %s", eventKey, fb.reason), strict, false)
		if substitutions != nil {
			*substitutions = append(*substitutions, substitution(
				NewOrderedMapFromPairs("type", "key", "key", eventKey),
				NewOrderedMapFromPairs("type", "fcl_keycode", "keycode", fb.keycode),
				fb.reason,
				"keys",
			))
		}
		return fb.keycode
	}
	if reason, ok := UnsupportedZLKeyReasons[eventKey]; ok {
		warn(fmt.Sprintf("ZL key event %q has no FCL control keycode equivalent: %s; substituted with UNKNOWN", eventKey, reason), strict, false)
	} else {
		warn(fmt.Sprintf("unsupported ZL key event %q; substituted with UNKNOWN", eventKey), strict, false)
	}
	fallback := GLFWToFCL["GLFW_KEY_UNKNOWN"]
	if substitutions != nil {
		*substitutions = append(*substitutions, substitution(
			NewOrderedMapFromPairs("type", "key", "key", eventKey),
			NewOrderedMapFromPairs("type", "fcl_keycode", "keycode", fallback),
			"No known FCL equivalent; substituted with UNKNOWN",
			"keys",
		))
	}
	return fallback
}

// convertKeyToZL converts an FCL keycode to (type, key) ZL event.
// Returns nil if no conversion (shouldn't happen in practice).
type zlEvent struct {
	eventType string
	key       string
}

func convertKeyToZL(keycode int, strict bool, autoClick bool, label string, substitutions *[]interface{}) *zlEvent {
	if keycode == -1 && strings.TrimSpace(label) == "*" {
		return &zlEvent{"key", "GLFW_KEY_KP_MULTIPLY"}
	}
	if key, ok := FCLMouseReverse[keycode]; ok {
		return &zlEvent{"key", key}
	}
	if scroll, ok := FCLScrollReverse[keycode]; ok {
		singleEvent := scroll[0]
		longEvent := scroll[1]
		if autoClick {
			return &zlEvent{"launcher_event", longEvent}
		}
		return &zlEvent{"launcher_event", singleEvent}
	}
	if key, ok := FCLToGLFW[keycode]; ok {
		return &zlEvent{"key", key}
	}
	if fb, ok := FCLToZLFallbacks[keycode]; ok {
		warn(fmt.Sprintf("FCL keycode %d has no exact ZL equivalent; %s", keycode, fb.reason), strict, false)
		if substitutions != nil {
			*substitutions = append(*substitutions, substitution(
				NewOrderedMapFromPairs("type", "fcl_keycode", "keycode", keycode),
				NewOrderedMapFromPairs("type", fb.eventType, "key", fb.key),
				fb.reason,
				"keys",
			))
		}
		return &zlEvent{fb.eventType, fb.key}
	}
	if reason, ok := UnsupportedFCLKeyReasons[keycode]; ok {
		warn(fmt.Sprintf("FCL keycode %d has no ZL control event equivalent: %s; substituted with GLFW_KEY_UNKNOWN", keycode, reason), strict, false)
	} else {
		warn(fmt.Sprintf("unsupported FCL keycode %d; substituted with GLFW_KEY_UNKNOWN", keycode), strict, false)
	}
	if substitutions != nil {
		*substitutions = append(*substitutions, substitution(
			NewOrderedMapFromPairs("type", "fcl_keycode", "keycode", keycode),
			NewOrderedMapFromPairs("type", "key", "key", "GLFW_KEY_UNKNOWN"),
			"No known ZL equivalent; substituted with GLFW_KEY_UNKNOWN",
			"keys",
		))
	}
	return &zlEvent{"key", "GLFW_KEY_UNKNOWN"}
}

// substitution creates a substitution record and increments the category counter.
func substitution(source, target *OrderedMap, reason string, category string) *OrderedMap {
	if _, ok := substitutionCounts[category]; ok {
		substitutionCounts[category]++
	}
	return NewOrderedMapFromPairs(
		"source", deepCopyJSON(source),
		"target", deepCopyJSON(target),
		"reason", reason,
	)
}

// appendSubstitutions appends substitution records to a mapping.
func appendSubstitutions(mapping *OrderedMap, substitutions []interface{}) *OrderedMap {
	if len(substitutions) == 0 {
		return mapping
	}
	var result *OrderedMap
	if mapping == nil {
		result = NewOrderedMap()
	} else {
		result = deepCopyJSON(mapping).(*OrderedMap)
	}
	var existing []interface{}
	if e, ok := result.Get("substitutions"); ok {
		if el, ok := e.([]interface{}); ok {
			existing = el
		}
	}
	for _, s := range substitutions {
		existing = append(existing, deepCopyJSON(s))
	}
	result.Set("substitutions", existing)
	return result
}

// makeMeta creates a metadata object.
func makeMeta(originFormat, originKind, originID string, original interface{}, mapping ...*OrderedMap) *OrderedMap {
	originalCopy := stripConverterMeta(original)
	meta := NewOrderedMapFromPairs(
		"schema", METASchemaVersion,
		"originFormat", originFormat,
		"originKind", originKind,
		"originId", originID,
		"original", originalCopy,
	)
	if len(mapping) > 0 && mapping[0] != nil {
		meta.Set("mapping", deepCopyJSON(mapping[0]))
	}
	return meta
}

// setMeta sets metadata on an object.
func setMeta(obj *OrderedMap, meta *OrderedMap) *OrderedMap {
	if meta != nil && meta.Len() > 0 {
		obj.Set(META_KEY, meta)
	}
	return obj
}

// getMeta retrieves metadata from an object.
func getMeta(obj interface{}) *OrderedMap {
	if m, ok := obj.(*OrderedMap); ok {
		if meta, ok := m.Get(META_KEY); ok {
			if mm, ok := meta.(*OrderedMap); ok {
				return mm
			}
		}
	}
	return nil
}

// metaOriginal retrieves the original from metadata if format/kind match.
func metaOriginal(obj interface{}, expectedFormat string, expectedKind ...string) interface{} {
	meta := getMeta(obj)
	if meta == nil {
		return nil
	}
	if of, _ := meta.Get("originFormat"); of != expectedFormat {
		return nil
	}
	if len(expectedKind) > 0 && expectedKind[0] != "" {
		if ok, _ := meta.Get("originKind"); ok != expectedKind[0] {
			return nil
		}
	}
	original, _ := meta.Get("original")
	if _, ok := original.(*OrderedMap); !ok {
		return nil
	}
	return deepCopyJSON(original)
}

// metaKind retrieves the originKind from metadata.
func metaKind(obj interface{}) string {
	meta := getMeta(obj)
	if meta == nil {
		return ""
	}
	kind, _ := meta.Get("originKind")
	if s, ok := kind.(string); ok {
		return s
	}
	return ""
}

// shortID generates a 12-char hex ID (like uuid4().hex[:12]).
func shortID() string {
	b := make([]byte, 16)
	if _, err := rand.Read(b); err != nil {
		panic(err)
	}
	return fmt.Sprintf("%x", b)[:12]
}

// fclID generates a full UUID string.
func fclID() string {
	b := make([]byte, 16)
	if _, err := rand.Read(b); err != nil {
		panic(err)
	}
	// Set version (4) and variant (RFC 4122)
	b[6] = (b[6] & 0x0f) | 0x40
	b[8] = (b[8] & 0x3f) | 0x80
	return fmt.Sprintf("%x-%x-%x-%x-%x", b[0:4], b[4:6], b[6:8], b[8:10], b[10:16])
}

// estimateWrapContentDP estimates dp size for wrap_content widgets.
func estimateWrapContentDP(widget *OrderedMap, styleName string, fclStyles []interface{}) (int, int) {
	text := textDefault(getOrEmpty(widget, "text"))
	var style *OrderedMap
	for _, item := range fclStyles {
		if s, ok := item.(*OrderedMap); ok {
			if name, _ := s.Get("name"); toString(name) == styleName {
				style = s
				break
			}
		}
	}
	if style == nil {
		style = defaultZLFallbackFCLStyle()
	}
	fontSize := maxInt(2, clampInt(getOr(style, "textSize", 14), 14))
	lines := strings.Split(text, "\n")
	if len(lines) == 0 || (len(lines) == 1 && lines[0] == "") {
		lines = []string{""}
	}
	longest := 0
	for _, line := range lines {
		// Python len() counts characters; Go utf8.RuneCountInString does too.
		n := len([]rune(line))
		if n > longest {
			longest = n
		}
	}
	width := maxInt(5, minInt(480, int(math.Round(float64(longest)*float64(fontSize)*0.62+8))))
	height := maxInt(5, minInt(240, int(math.Round(float64(len(lines))*float64(fontSize)*1.25+6))))
	return width, height
}

// getOr retrieves a value from an OrderedMap, returning default if not found.
func getOr(m *OrderedMap, key string, defaultVal interface{}) interface{} {
	if m == nil {
		return defaultVal
	}
	if v, ok := m.Get(key); ok {
		return v
	}
	return defaultVal
}

// getOrEmpty retrieves a value from an OrderedMap, returning nil if not found.
func getOrEmpty(m *OrderedMap, key string) interface{} {
	if m == nil {
		return nil
	}
	v, _ := m.Get(key)
	return v
}

// getOrOrderedMap retrieves a value as *OrderedMap, returning empty if not found/not dict.
func getOrOrderedMap(m *OrderedMap, key string) *OrderedMap {
	if m == nil {
		return NewOrderedMap()
	}
	v, ok := m.Get(key)
	if !ok || v == nil {
		return NewOrderedMap()
	}
	if om, ok := v.(*OrderedMap); ok {
		return om
	}
	return NewOrderedMap()
}

// getOrList retrieves a value as []interface{}, returning empty if not found/not list.
func getOrList(m *OrderedMap, key string) []interface{} {
	if m == nil {
		return []interface{}{}
	}
	v, ok := m.Get(key)
	if !ok || v == nil {
		return []interface{}{}
	}
	if l, ok := v.([]interface{}); ok {
		return l
	}
	return []interface{}{}
}

// getOrString retrieves a value as string.
func getOrString(m *OrderedMap, key string, defaultVal string) string {
	if m == nil {
		return defaultVal
	}
	v, ok := m.Get(key)
	if !ok || v == nil {
		return defaultVal
	}
	return toString(v)
}

// asOrderedMap converts interface{} to *OrderedMap, returns nil if not a dict.
func asOrderedMap(v interface{}) (*OrderedMap, bool) {
	if v == nil {
		return nil, false
	}
	if om, ok := v.(*OrderedMap); ok {
		return om, true
	}
	return nil, false
}

// asList converts interface{} to []interface{}, returns nil if not a list.
func asList(v interface{}) ([]interface{}, bool) {
	if v == nil {
		return nil, false
	}
	if l, ok := v.([]interface{}); ok {
		return l, true
	}
	return nil, false
}

// normalizedControlText extracts alphanumeric and CJK characters, lowercased.
var controlTextRe = regexp.MustCompile(`[A-Za-z0-9]+|[\x{4e00}-\x{9fff}]+`)

func normalizedControlText(text string) string {
	matches := controlTextRe.FindAllString(text, -1)
	return strings.ToLower(strings.Join(matches, ""))
}

// normalizedControlWords extracts words (len>=2) from text.
var cjkFullMatchRe = regexp.MustCompile(`^[\x{4e00}-\x{9fff}]+$`)

func normalizedControlWords(text string) map[string]struct{} {
	words := map[string]struct{}{}
	for _, raw := range controlTextRe.FindAllString(text, -1) {
		word := strings.ToLower(raw)
		if len([]rune(word)) < 2 {
			continue
		}
		words[word] = struct{}{}
		if cjkFullMatchRe.MatchString(word) {
			runes := []rune(word)
			maxSize := 5
			if len(runes) < maxSize {
				maxSize = len(runes)
			}
			for size := 2; size <= maxSize; size++ {
				for start := 0; start <= len(runes)-size; start++ {
					words[string(runes[start:start+size])] = struct{}{}
				}
			}
		}
	}
	return words
}

// dedupeEvents removes duplicate events (by type+key), preserving order.
func dedupeEvents(events []*OrderedMap) []*OrderedMap {
	result := []*OrderedMap{}
	seen := map[string]struct{}{}
	for _, event := range events {
		eventType := toString(getOr(event, "type", ""))
		key := toString(getOr(event, "key", ""))
		k := eventType + "\x00" + key
		if _, ok := seen[k]; !ok {
			seen[k] = struct{}{}
			result = append(result, event)
		}
	}
	return result
}
