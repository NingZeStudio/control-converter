package main

const (
	FCLControllerVersion = 21
	ZLEditorVersion      = 11
	META_KEY             = "_control_byIQge报错别找我"
	METASchemaVersion    = 1
)

// ZLKeyAliases maps various key aliases to canonical GLFW names.
var ZLKeyAliases = buildZLKeyAliases()

func buildZLKeyAliases() map[string]string {
	m := map[string]string{
		"GLFW_MOUSE_BUTTON_1":       "GLFW_MOUSE_BUTTON_LEFT",
		"GLFW_MOUSE_BUTTON_2":       "GLFW_MOUSE_BUTTON_RIGHT",
		"GLFW_MOUSE_BUTTON_3":       "GLFW_MOUSE_BUTTON_MIDDLE",
		"MOUSE_SCROLL_UP":           "launcher.event.scroll_up.single",
		"MOUSE_SCROLL_DOWN":         "launcher.event.scroll_down.single",
		"key.mouse.left":            "GLFW_MOUSE_BUTTON_LEFT",
		"key.mouse.right":           "GLFW_MOUSE_BUTTON_RIGHT",
		"key.mouse.middle":          "GLFW_MOUSE_BUTTON_MIDDLE",
		"key.mouse.4":               "GLFW_MOUSE_BUTTON_4",
		"key.mouse.5":               "GLFW_MOUSE_BUTTON_5",
		"key.mouse.6":               "GLFW_MOUSE_BUTTON_6",
		"key.mouse.7":               "GLFW_MOUSE_BUTTON_7",
		"key.mouse.8":               "GLFW_MOUSE_BUTTON_8",
		"key.keyboard.unknown":      "GLFW_KEY_UNKNOWN",
		"key.keyboard.num.lock":     "GLFW_KEY_NUM_LOCK",
		"key.keyboard.keypad.0":     "GLFW_KEY_KP_0",
		"key.keyboard.keypad.1":     "GLFW_KEY_KP_1",
		"key.keyboard.keypad.2":     "GLFW_KEY_KP_2",
		"key.keyboard.keypad.3":     "GLFW_KEY_KP_3",
		"key.keyboard.keypad.4":     "GLFW_KEY_KP_4",
		"key.keyboard.keypad.5":     "GLFW_KEY_KP_5",
		"key.keyboard.keypad.6":     "GLFW_KEY_KP_6",
		"key.keyboard.keypad.7":     "GLFW_KEY_KP_7",
		"key.keyboard.keypad.8":     "GLFW_KEY_KP_8",
		"key.keyboard.keypad.9":     "GLFW_KEY_KP_9",
		"key.keyboard.keypad.add":     "GLFW_KEY_KP_ADD",
		"key.keyboard.keypad.decimal": "GLFW_KEY_KP_DECIMAL",
		"key.keyboard.keypad.enter":   "GLFW_KEY_KP_ENTER",
		"key.keyboard.keypad.equal":   "GLFW_KEY_KP_EQUAL",
		"key.keyboard.keypad.multiply":"GLFW_KEY_KP_MULTIPLY",
		"key.keyboard.keypad.divide":  "GLFW_KEY_KP_DIVIDE",
		"key.keyboard.keypad.subtract":"GLFW_KEY_KP_SUBTRACT",
		"key.keyboard.down":           "GLFW_KEY_DOWN",
		"key.keyboard.left":           "GLFW_KEY_LEFT",
		"key.keyboard.right":          "GLFW_KEY_RIGHT",
		"key.keyboard.up":             "GLFW_KEY_UP",
		"key.keyboard.apostrophe":     "GLFW_KEY_APOSTROPHE",
		"key.keyboard.backslash":      "GLFW_KEY_BACKSLASH",
		"key.keyboard.comma":          "GLFW_KEY_COMMA",
		"key.keyboard.equal":          "GLFW_KEY_EQUAL",
		"key.keyboard.grave.accent":   "GLFW_KEY_GRAVE_ACCENT",
		"key.keyboard.left.bracket":   "GLFW_KEY_LEFT_BRACKET",
		"key.keyboard.minus":          "GLFW_KEY_MINUS",
		"key.keyboard.period":         "GLFW_KEY_PERIOD",
		"key.keyboard.right.bracket":  "GLFW_KEY_RIGHT_BRACKET",
		"key.keyboard.semicolon":      "GLFW_KEY_SEMICOLON",
		"key.keyboard.slash":          "GLFW_KEY_SLASH",
		"key.keyboard.space":          "GLFW_KEY_SPACE",
		"key.keyboard.tab":            "GLFW_KEY_TAB",
		"key.keyboard.left.alt":       "GLFW_KEY_LEFT_ALT",
		"key.keyboard.left.control":   "GLFW_KEY_LEFT_CONTROL",
		"key.keyboard.left.shift":     "GLFW_KEY_LEFT_SHIFT",
		"key.keyboard.left.win":       "GLFW_KEY_LEFT_SUPER",
		"key.keyboard.left.super":     "GLFW_KEY_LEFT_SUPER",
		"key.keyboard.left.meta":      "GLFW_KEY_LEFT_SUPER",
		"key.keyboard.right.alt":      "GLFW_KEY_RIGHT_ALT",
		"key.keyboard.right.control":  "GLFW_KEY_RIGHT_CONTROL",
		"key.keyboard.right.shift":    "GLFW_KEY_RIGHT_SHIFT",
		"key.keyboard.right.win":      "GLFW_KEY_RIGHT_SUPER",
		"key.keyboard.right.super":    "GLFW_KEY_RIGHT_SUPER",
		"key.keyboard.right.meta":     "GLFW_KEY_RIGHT_SUPER",
		"key.keyboard.enter":          "GLFW_KEY_ENTER",
		"key.keyboard.escape":         "GLFW_KEY_ESCAPE",
		"key.keyboard.backspace":      "GLFW_KEY_BACKSPACE",
		"key.keyboard.delete":         "GLFW_KEY_DELETE",
		"key.keyboard.end":            "GLFW_KEY_END",
		"key.keyboard.home":           "GLFW_KEY_HOME",
		"key.keyboard.insert":         "GLFW_KEY_INSERT",
		"key.keyboard.page.down":      "GLFW_KEY_PAGE_DOWN",
		"key.keyboard.page.up":        "GLFW_KEY_PAGE_UP",
		"key.keyboard.caps.lock":      "GLFW_KEY_CAPS_LOCK",
		"key.keyboard.pause":          "GLFW_KEY_PAUSE",
		"key.keyboard.scroll.lock":    "GLFW_KEY_SCROLL_LOCK",
		"key.keyboard.menu":           "GLFW_KEY_MENU",
		"key.keyboard.print.screen":   "GLFW_KEY_PRINT_SCREEN",
		"key.keyboard.world.1":        "GLFW_KEY_WORLD_1",
		"key.keyboard.world.2":        "GLFW_KEY_WORLD_2",
		"key.keyboard.keypad.separator": "GLFW_KEY_KP_DECIMAL",
	}
	for i := 0; i < 10; i++ {
		key := "key.keyboard." + string(rune('0'+i))
		val := "GLFW_KEY_" + string(rune('0'+i))
		m[key] = val
	}
	for code := 'a'; code <= 'z'; code++ {
		key := "key.keyboard." + string(code)
		val := "GLFW_KEY_" + string(code-32)
		m[key] = val
	}
	for i := 1; i <= 25; i++ {
		key := "key.keyboard.f" + itoa(i)
		val := "GLFW_KEY_F" + itoa(i)
		m[key] = val
	}
	return m
}

var FCLMouse = map[string]int{
	"GLFW_MOUSE_BUTTON_LEFT":   1000,
	"GLFW_MOUSE_BUTTON_MIDDLE": 1001,
	"GLFW_MOUSE_BUTTON_RIGHT":  1002,
}

var FCLMouseReverse = map[int]string{
	1000: "GLFW_MOUSE_BUTTON_LEFT",
	1001: "GLFW_MOUSE_BUTTON_MIDDLE",
	1002: "GLFW_MOUSE_BUTTON_RIGHT",
}

// FCLScrollReverse maps scroll keycodes to (single_event, long_event).
var FCLScrollReverse = map[int][2]string{
	1003: {"launcher.event.scroll_up.single", "launcher.event.scroll_up"},
	1004: {"launcher.event.scroll_down.single", "launcher.event.scroll_down"},
}

var ZLOnlyKeys = map[string]struct{}{
	"GLFW_KEY_WORLD_1":       {},
	"GLFW_KEY_WORLD_2":       {},
	"GLFW_KEY_F25":           {},
	"GLFW_KEY_MENU":          {},
	"GLFW_KEY_LAST":          {},
	"GLFW_MOD_SHIFT":         {},
	"GLFW_MOD_CONTROL":       {},
	"GLFW_MOD_ALT":           {},
	"GLFW_MOD_SUPER":         {},
	"GLFW_MOD_CAPS_LOCK":     {},
	"GLFW_MOD_NUM_LOCK":      {},
	"GLFW_MOUSE_BUTTON_4":    {},
	"GLFW_MOUSE_BUTTON_5":    {},
	"GLFW_MOUSE_BUTTON_6":    {},
	"GLFW_MOUSE_BUTTON_7":    {},
	"GLFW_MOUSE_BUTTON_8":    {},
	"GLFW_MOUSE_BUTTON_LAST": {},
}

var UnsupportedZLKeyReasons = buildUnsupportedZLKeyReasons()

func buildUnsupportedZLKeyReasons() map[string]string {
	m := make(map[string]string, len(ZLOnlyKeys))
	for key := range ZLOnlyKeys {
		m[key] = "FCL controls do not define an exact matching keycode"
	}
	return m
}

var UnsupportedFCLKeyReasons = map[int]string{
	0:   "FCL KEY_RESERVED is not a real input key",
	121: "FCL KEY_KPCOMMA has no exact GLFW/ZL control event equivalent",
}

var GLFWToFCL = map[string]int{
	"GLFW_KEY_UNKNOWN":        240,
	"GLFW_KEY_SPACE":          57,
	"GLFW_KEY_APOSTROPHE":     40,
	"GLFW_KEY_COMMA":          51,
	"GLFW_KEY_MINUS":          12,
	"GLFW_KEY_PERIOD":         52,
	"GLFW_KEY_SLASH":          53,
	"GLFW_KEY_0":              11,
	"GLFW_KEY_1":              2,
	"GLFW_KEY_2":              3,
	"GLFW_KEY_3":              4,
	"GLFW_KEY_4":              5,
	"GLFW_KEY_5":              6,
	"GLFW_KEY_6":              7,
	"GLFW_KEY_7":              8,
	"GLFW_KEY_8":              9,
	"GLFW_KEY_9":              10,
	"GLFW_KEY_SEMICOLON":      39,
	"GLFW_KEY_EQUAL":          13,
	"GLFW_KEY_A":              30,
	"GLFW_KEY_B":              48,
	"GLFW_KEY_C":              46,
	"GLFW_KEY_D":              32,
	"GLFW_KEY_E":              18,
	"GLFW_KEY_F":              33,
	"GLFW_KEY_G":              34,
	"GLFW_KEY_H":              35,
	"GLFW_KEY_I":              23,
	"GLFW_KEY_J":              36,
	"GLFW_KEY_K":              37,
	"GLFW_KEY_L":              38,
	"GLFW_KEY_M":              50,
	"GLFW_KEY_N":              49,
	"GLFW_KEY_O":              24,
	"GLFW_KEY_P":              25,
	"GLFW_KEY_Q":              16,
	"GLFW_KEY_R":              19,
	"GLFW_KEY_S":              31,
	"GLFW_KEY_T":              20,
	"GLFW_KEY_U":              22,
	"GLFW_KEY_V":              47,
	"GLFW_KEY_W":              17,
	"GLFW_KEY_X":              45,
	"GLFW_KEY_Y":              21,
	"GLFW_KEY_Z":              44,
	"GLFW_KEY_LEFT_BRACKET":   26,
	"GLFW_KEY_RIGHT_BRACKET":  27,
	"GLFW_KEY_BACKSLASH":      43,
	"GLFW_KEY_GRAVE_ACCENT":   41,
	"GLFW_KEY_ESCAPE":         1,
	"GLFW_KEY_ENTER":          28,
	"GLFW_KEY_TAB":            15,
	"GLFW_KEY_BACKSPACE":      14,
	"GLFW_KEY_INSERT":         110,
	"GLFW_KEY_DELETE":         111,
	"GLFW_KEY_RIGHT":          106,
	"GLFW_KEY_LEFT":           105,
	"GLFW_KEY_DOWN":           108,
	"GLFW_KEY_UP":             103,
	"GLFW_KEY_PAGE_UP":        104,
	"GLFW_KEY_PAGE_DOWN":      109,
	"GLFW_KEY_HOME":           102,
	"GLFW_KEY_END":            107,
	"GLFW_KEY_CAPS_LOCK":      58,
	"GLFW_KEY_SCROLL_LOCK":    70,
	"GLFW_KEY_NUM_LOCK":       69,
	"GLFW_KEY_PRINT_SCREEN":   99,
	"GLFW_KEY_PAUSE":          119,
	"GLFW_KEY_F1":             59,
	"GLFW_KEY_F2":             60,
	"GLFW_KEY_F3":             61,
	"GLFW_KEY_F4":             62,
	"GLFW_KEY_F5":             63,
	"GLFW_KEY_F6":             64,
	"GLFW_KEY_F7":             65,
	"GLFW_KEY_F8":             66,
	"GLFW_KEY_F9":             67,
	"GLFW_KEY_F10":            68,
	"GLFW_KEY_F11":            87,
	"GLFW_KEY_F12":            88,
	"GLFW_KEY_F13":            183,
	"GLFW_KEY_F14":            184,
	"GLFW_KEY_F15":            185,
	"GLFW_KEY_F16":            186,
	"GLFW_KEY_F17":            187,
	"GLFW_KEY_F18":            188,
	"GLFW_KEY_F19":            189,
	"GLFW_KEY_F20":            190,
	"GLFW_KEY_F21":            191,
	"GLFW_KEY_F22":            192,
	"GLFW_KEY_F23":            193,
	"GLFW_KEY_F24":            194,
	"GLFW_KEY_KP_0":           82,
	"GLFW_KEY_KP_1":           79,
	"GLFW_KEY_KP_2":           80,
	"GLFW_KEY_KP_3":           81,
	"GLFW_KEY_KP_4":           75,
	"GLFW_KEY_KP_5":           76,
	"GLFW_KEY_KP_6":           77,
	"GLFW_KEY_KP_7":           71,
	"GLFW_KEY_KP_8":           72,
	"GLFW_KEY_KP_9":           73,
	"GLFW_KEY_KP_DECIMAL":     83,
	"GLFW_KEY_KP_DIVIDE":      98,
	"GLFW_KEY_KP_MULTIPLY":    55,
	"GLFW_KEY_KP_SUBTRACT":    74,
	"GLFW_KEY_KP_ADD":         78,
	"GLFW_KEY_KP_ENTER":       96,
	"GLFW_KEY_KP_EQUAL":       117,
	"GLFW_KEY_LEFT_SHIFT":     42,
	"GLFW_KEY_LEFT_CONTROL":   29,
	"GLFW_KEY_LEFT_ALT":       56,
	"GLFW_KEY_LEFT_SUPER":     125,
	"GLFW_KEY_RIGHT_SHIFT":    54,
	"GLFW_KEY_RIGHT_CONTROL":  97,
	"GLFW_KEY_RIGHT_ALT":      100,
	"GLFW_KEY_RIGHT_SUPER":    126,
}

var FCLToGLFW = buildFCLToGLFW()

func buildFCLToGLFW() map[int]string {
	m := make(map[int]string, len(GLFWToFCL))
	for k, v := range GLFWToFCL {
		m[v] = k
	}
	return m
}

// ZLToFCLFallbacks maps unsupported ZL keys to (fcl_keycode, reason).
type zlToFCLFallback struct {
	keycode int
	reason  string
}

var ZLToFCLFallbacks = map[string]zlToFCLFallback{
	"GLFW_MOUSE_BUTTON_4": {1003, "FCL has no side mouse button 4; substituted with scroll up"},
	"GLFW_MOUSE_BUTTON_5": {1004, "FCL has no side mouse button 5; substituted with scroll down"},
	"GLFW_MOUSE_BUTTON_6": {1003, "FCL has no side mouse button 6; substituted with scroll up"},
	"GLFW_MOUSE_BUTTON_7": {1004, "FCL has no side mouse button 7; substituted with scroll down"},
	"GLFW_MOUSE_BUTTON_8": {1004, "FCL has no side mouse button 8; substituted with scroll down"},
	"GLFW_KEY_F25":        {GLFWToFCL["GLFW_KEY_F24"], "FCL has no F25; substituted with F24"},
	"GLFW_KEY_WORLD_1":    {GLFWToFCL["GLFW_KEY_UNKNOWN"], "FCL has no WORLD_1; substituted with UNKNOWN"},
	"GLFW_KEY_WORLD_2":    {GLFWToFCL["GLFW_KEY_UNKNOWN"], "FCL has no WORLD_2; substituted with UNKNOWN"},
	"GLFW_KEY_MENU":       {GLFWToFCL["GLFW_KEY_UNKNOWN"], "FCL has no menu key; substituted with UNKNOWN"},
	"GLFW_KEY_LAST":       {GLFWToFCL["GLFW_KEY_UNKNOWN"], "FCL has no LAST sentinel key; substituted with UNKNOWN"},
	"GLFW_MOD_SHIFT":      {GLFWToFCL["GLFW_KEY_LEFT_SHIFT"], "FCL has no modifier event; substituted with left shift"},
	"GLFW_MOD_CONTROL":    {GLFWToFCL["GLFW_KEY_LEFT_CONTROL"], "FCL has no modifier event; substituted with left control"},
	"GLFW_MOD_ALT":        {GLFWToFCL["GLFW_KEY_LEFT_ALT"], "FCL has no modifier event; substituted with left alt"},
	"GLFW_MOD_SUPER":      {GLFWToFCL["GLFW_KEY_LEFT_SUPER"], "FCL has no modifier event; substituted with left super"},
	"GLFW_MOD_CAPS_LOCK":  {GLFWToFCL["GLFW_KEY_CAPS_LOCK"], "FCL has no modifier event; substituted with caps lock"},
	"GLFW_MOD_NUM_LOCK":   {GLFWToFCL["GLFW_KEY_NUM_LOCK"], "FCL has no modifier event; substituted with num lock"},
	"GLFW_MOUSE_BUTTON_LAST": {1004, "FCL has no LAST mouse sentinel; substituted with scroll down"},
}

// FCLToZLFallbacks maps unsupported FCL keycodes to ((type, key), reason).
type fclToZLFallback struct {
	eventType string
	key       string
	reason    string
}

var FCLToZLFallbacks = map[int]fclToZLFallback{
	0:   {"key", "GLFW_KEY_UNKNOWN", "FCL KEY_RESERVED is not a real input key; substituted with GLFW_KEY_UNKNOWN"},
	121: {"key", "GLFW_KEY_KP_DECIMAL", "FCL KEY_KPCOMMA has no exact GLFW key; substituted with keypad decimal"},
}

// Global mutable state (matches Python module-level globals).
var (
	warnedMessages       = map[string]struct{}{}
	substitutionCounts   = map[string]int{"keys": 0, "events": 0, "layers": 0, "directions": 0}
)
