package main

import (
	"bytes"
	"encoding/json"
	"sort"
)

// OrderedMap is a map that preserves insertion order, matching Python dict semantics.
// It implements json.Marshaler so that JSON output preserves key order.
type OrderedMap struct {
	keys   []string
	values map[string]interface{}
}

// NewOrderedMap creates an empty OrderedMap.
func NewOrderedMap() *OrderedMap {
	return &OrderedMap{
		keys:   nil,
		values: make(map[string]interface{}),
	}
}

// NewOrderedMapFromPairs creates an OrderedMap from key-value pairs.
func NewOrderedMapFromPairs(pairs ...interface{}) *OrderedMap {
	om := NewOrderedMap()
	for i := 0; i+1 < len(pairs); i += 2 {
		key, ok := pairs[i].(string)
		if !ok {
			panic("OrderedMap key must be string")
		}
		om.Set(key, pairs[i+1])
	}
	return om
}

// Set sets a key-value pair, preserving insertion order for new keys.
func (om *OrderedMap) Set(key string, value interface{}) *OrderedMap {
	if _, exists := om.values[key]; !exists {
		om.keys = append(om.keys, key)
	}
	om.values[key] = value
	return om
}

// SetIfAbsent sets a key-value pair only if the key does not exist (like Python setdefault).
func (om *OrderedMap) SetIfAbsent(key string, value interface{}) interface{} {
	if existing, exists := om.values[key]; exists {
		return existing
	}
	om.keys = append(om.keys, key)
	om.values[key] = value
	return value
}

// Get retrieves a value by key.
func (om *OrderedMap) Get(key string) (interface{}, bool) {
	if om == nil {
		return nil, false
	}
	v, ok := om.values[key]
	return v, ok
}

// GetMust retrieves a value by key, panicking if not found.
func (om *OrderedMap) GetMust(key string) interface{} {
	v, ok := om.Get(key)
	if !ok {
		panic("key not found: " + key)
	}
	return v
}

// Has reports whether the key exists.
func (om *OrderedMap) Has(key string) bool {
	if om == nil {
		return false
	}
	_, ok := om.values[key]
	return ok
}

// Delete removes a key (does not preserve slot, but key list is rebuilt lazily).
func (om *OrderedMap) Delete(key string) {
	if _, exists := om.values[key]; !exists {
		return
	}
	delete(om.values, key)
	newKeys := make([]string, 0, len(om.keys)-1)
	for _, k := range om.keys {
		if k != key {
			newKeys = append(newKeys, k)
		}
	}
	om.keys = newKeys
}

// Keys returns the keys in insertion order.
func (om *OrderedMap) Keys() []string {
	if om == nil {
		return nil
	}
	return om.keys
}

// Len returns the number of keys.
func (om *OrderedMap) Len() int {
	if om == nil {
		return 0
	}
	return len(om.keys)
}

// MarshalJSON implements json.Marshaler, preserving key order.
func (om OrderedMap) MarshalJSON() ([]byte, error) {
	if om.values == nil {
		return []byte("{}"), nil
	}
	var buf bytes.Buffer
	buf.WriteByte('{')
	for i, key := range om.keys {
		if i > 0 {
			buf.WriteByte(',')
		}
		keyBytes, err := json.Marshal(key)
		if err != nil {
			return nil, err
		}
		buf.Write(keyBytes)
		buf.WriteByte(':')
		valBytes, err := json.Marshal(om.values[key])
		if err != nil {
			return nil, err
		}
		buf.Write(valBytes)
	}
	buf.WriteByte('}')
	return buf.Bytes(), nil
}

// UnmarshalJSON implements json.Unmarshaler for OrderedMap.
func (om *OrderedMap) UnmarshalJSON(data []byte) error {
	if om.values == nil {
		om.values = make(map[string]interface{})
	}
	om.keys = om.keys[:0]
	dec := json.NewDecoder(bytes.NewReader(data))
	dec.UseNumber()
	tok, err := dec.Token()
	if err != nil {
		return err
	}
	delim, ok := tok.(json.Delim)
	if !ok || delim != '{' {
		return &json.UnmarshalTypeError{Value: "non-object", Type: nil}
	}
	for dec.More() {
		tok, err := dec.Token()
		if err != nil {
			return err
		}
		key, ok := tok.(string)
		if !ok {
			return &json.UnmarshalTypeError{Value: "non-string key", Type: nil}
		}
		var val interface{}
		if err := dec.Decode(&val); err != nil {
			return err
		}
		om.Set(key, val)
	}
	return nil
}

// SortedKeys returns keys in sorted order (for deterministic iteration when order doesn't matter).
func (om *OrderedMap) SortedKeys() []string {
	keys := make([]string, len(om.keys))
	copy(keys, om.keys)
	sort.Strings(keys)
	return keys
}
