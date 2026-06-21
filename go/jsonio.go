package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"strings"
)

// decodeJSON decodes JSON bytes into interface{} where all objects are *OrderedMap
// and all numbers are json.Number (to preserve precision).
func decodeJSON(data []byte) (interface{}, error) {
	dec := json.NewDecoder(bytes.NewReader(data))
	dec.UseNumber()
	return decodeValue(dec)
}

func decodeValue(dec *json.Decoder) (interface{}, error) {
	tok, err := dec.Token()
	if err != nil {
		return nil, err
	}
	return decodeValueWithToken(dec, tok)
}

func decodeValueWithToken(dec *json.Decoder, tok json.Token) (interface{}, error) {
	if delim, ok := tok.(json.Delim); ok {
		switch delim {
		case '{':
			return decodeObject(dec)
		case '[':
			return decodeArray(dec)
		}
		return nil, fmt.Errorf("unexpected delim: %v", delim)
	}
	// Simple token: string, json.Number, bool, nil
	return tok, nil
}

func decodeObject(dec *json.Decoder) (*OrderedMap, error) {
	om := NewOrderedMap()
	for dec.More() {
		// Read key
		tok, err := dec.Token()
		if err != nil {
			return nil, err
		}
		key, ok := tok.(string)
		if !ok {
			return nil, fmt.Errorf("expected string key, got %v", tok)
		}
		// Read value
		val, err := decodeValue(dec)
		if err != nil {
			return nil, err
		}
		om.Set(key, val)
	}
	// Read closing }
	if _, err := dec.Token(); err != nil {
		return nil, err
	}
	return om, nil
}

func decodeArray(dec *json.Decoder) ([]interface{}, error) {
	var arr []interface{}
	for dec.More() {
		val, err := decodeValue(dec)
		if err != nil {
			return nil, err
		}
		arr = append(arr, val)
	}
	// Read closing ]
	if _, err := dec.Token(); err != nil {
		return nil, err
	}
	if arr == nil {
		arr = []interface{}{}
	}
	return arr, nil
}

// encodeJSON encodes a value to JSON with the given indent.
// If indent is empty, produces compact JSON.
func encodeJSON(value interface{}, indent string) ([]byte, error) {
	if indent == "" {
		return json.Marshal(value)
	}
	var buf bytes.Buffer
	enc := json.NewEncoder(&buf)
	enc.SetEscapeHTML(false)
	enc.SetIndent("", indent)
	if err := enc.Encode(value); err != nil {
		return nil, err
	}
	// json.Encoder.Encode adds a trailing newline, which we want for non-compact mode
	return buf.Bytes(), nil
}

// stripJSONComments removes // and /* */ comments from JSON text.
func stripJSONComments(text string) string {
	var result strings.Builder
	inString := false
	escape := false
	index := 0
	for index < len(text) {
		char := text[index]
		nextChar := byte(0)
		if index+1 < len(text) {
			nextChar = text[index+1]
		}
		if inString {
			result.WriteByte(char)
			if escape {
				escape = false
			} else if char == '\\' {
				escape = true
			} else if char == '"' {
				inString = false
			}
			index++
		} else if char == '"' {
			inString = true
			result.WriteByte(char)
			index++
		} else if char == '/' && nextChar == '/' {
			index += 2
			for index < len(text) && text[index] != '\r' && text[index] != '\n' {
				index++
			}
		} else if char == '/' && nextChar == '*' {
			index += 2
			for index+1 < len(text) && !(text[index] == '*' && text[index+1] == '/') {
				index++
			}
			index += 2
		} else {
			result.WriteByte(char)
			index++
		}
	}
	return result.String()
}

// loadJSONFile reads and parses a JSON file, returning *OrderedMap.
func loadJSONFile(path string) (*OrderedMap, error) {
	data, err := readFile(path)
	if err != nil {
		return nil, err
	}
	result, err := decodeJSON(data)
	if err != nil {
		// Try stripping comments
		stripped := stripJSONComments(string(data))
		result, err = decodeJSON([]byte(stripped))
		if err != nil {
			return nil, err
		}
	}
	if om, ok := result.(*OrderedMap); ok {
		return om, nil
	}
	return nil, fmt.Errorf("expected JSON object at root")
}

// readFile reads a file's contents.
func readFile(path string) ([]byte, error) {
	return osReadFile(path)
}

// writeJSONFile writes a value as JSON to a file.
func writeJSONFile(path string, value interface{}, compact bool) error {
	var data []byte
	var err error
	if compact {
		data, err = encodeJSON(value, "")
		if err != nil {
			return err
		}
	} else {
		data, err = encodeJSON(value, "  ")
		if err != nil {
			return err
		}
	}
	return osWriteFile(path, data, 0644)
}

// printSubstitutionSummary prints the substitution counts to stderr.
func printSubstitutionSummary() {
	total := 0
	for _, count := range substitutionCounts {
		total += count
	}
	if total == 0 {
		return
	}
	fmt.Fprintf(osStderr(), "conversion substitutions: keys=%d, events=%d, layers=%d, directions=%d\n",
		substitutionCounts["keys"], substitutionCounts["events"],
		substitutionCounts["layers"], substitutionCounts["directions"])
}

// Ensure io is used
var _ = io.EOF
