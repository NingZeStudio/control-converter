package main

import (
	"fmt"
	"math"
	"os"
	"path/filepath"
	"strconv"
)

// osReadFile reads a file's contents.
func osReadFile(path string) ([]byte, error) {
	return os.ReadFile(path)
}

// osWriteFile writes data to a file, creating parent directories if needed.
func osWriteFile(path string, data []byte, perm os.FileMode) error {
	dir := filepath.Dir(path)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(path, data, perm)
}

// osStderr returns os.Stderr.
func osStderr() *os.File {
	return os.Stderr
}

// parseAspect parses an aspect ratio string like "16/9" or "1.778".
func parseAspect(s string) (float64, error) {
	// Try parsing as a fraction first
	if idx := indexByte(s, '/'); idx >= 0 {
		num, err := strconv.ParseFloat(s[:idx], 64)
		if err != nil {
			return 0, err
		}
		den, err := strconv.ParseFloat(s[idx+1:], 64)
		if err != nil {
			return 0, err
		}
		if den == 0 {
			return 0, fmt.Errorf("denominator is zero")
		}
		return num / den, nil
	}
	return strconv.ParseFloat(s, 64)
}

func indexByte(s string, c byte) int {
	for i := 0; i < len(s); i++ {
		if s[i] == c {
			return i
		}
	}
	return -1
}

// CLIArgs holds parsed command-line arguments.
type CLIArgs struct {
	mode                 string
	input                string
	output               string
	includeDirections    bool
	lossless             bool
	absoluteAsPercentage bool
	strict               bool
	compact              bool
	stripMeta            bool
	aspect               float64
}

func printUsage() {
	fmt.Fprintf(os.Stderr, `usage: cc fcl2zl input output [options]

Convert control-layout JSON from FCL (FoldCraftLauncher) to ZL (ZalithLauncher2).

positional arguments:
  input                 input JSON file path
  output                output JSON file path

options:
  --include-directions     approximate FCL direction controls as ZL button grids
  --lossless               substitute unsupported controls instead of dropping them
  --absolute-as-percentage convert FCL absolute dp sizes to ZL percentage sizes
  --strict                 fail instead of warning on unsupported fields/events
  --compact                write compact JSON instead of pretty JSON
  --strip-meta             remove converter metadata from output JSON
  --aspect RATIO           screen aspect ratio (e.g. 16/9, default: 1.7778)
  -h, --help               show this help message and exit
`)
}

func parseArgs(args []string) (*CLIArgs, error) {
	result := &CLIArgs{
		aspect: 16.0 / 9.0,
	}

	if len(args) == 0 {
		return nil, fmt.Errorf("missing mode argument")
	}

	result.mode = args[0]
	if result.mode != "fcl2zl" {
		return nil, fmt.Errorf("unsupported mode: %s (only fcl2zl is supported)", result.mode)
	}

	rest := args[1:]
	var positionals []string
	i := 0
	for i < len(rest) {
		arg := rest[i]
		switch arg {
		case "-h", "--help":
			printUsage()
			os.Exit(0)
		case "--include-directions":
			result.includeDirections = true
		case "--lossless", "--no-drop":
			result.lossless = true
		case "--absolute-as-percentage":
			result.absoluteAsPercentage = true
		case "--strict":
			result.strict = true
		case "--compact":
			result.compact = true
		case "--strip-meta":
			result.stripMeta = true
		case "--aspect":
			if i+1 >= len(rest) {
				return nil, fmt.Errorf("--aspect requires a value")
			}
			i++
			aspect, err := parseAspect(rest[i])
			if err != nil {
				return nil, fmt.Errorf("invalid --aspect value: %v", err)
			}
			result.aspect = aspect
		default:
			if len(arg) > 0 && arg[0] == '-' {
				return nil, fmt.Errorf("unknown option: %s", arg)
			}
			positionals = append(positionals, arg)
		}
		i++
	}

	if len(positionals) < 2 {
		return nil, fmt.Errorf("input and output file paths are required")
	}
	result.input = positionals[0]
	result.output = positionals[1]

	if math.IsNaN(result.aspect) || math.IsInf(result.aspect, 0) || result.aspect <= 0 {
		return nil, fmt.Errorf("--aspect must be a positive finite number")
	}

	return result, nil
}

func main() {
	args, err := parseArgs(os.Args[1:])
	if err != nil {
		fmt.Fprintf(os.Stderr, "error: %v\n", err)
		fmt.Fprintf(os.Stderr, "Use --help for usage information.\n")
		os.Exit(2)
	}

	source, err := loadJSONFile(args.input)
	if err != nil {
		fmt.Fprintf(os.Stderr, "error: failed to read input file: %v\n", err)
		os.Exit(1)
	}

	// Reset global state for each run
	warnedMessages = map[string]struct{}{}
	substitutionCounts = map[string]int{"keys": 0, "events": 0, "layers": 0, "directions": 0}

	var result interface{}
	result = convertFCLToZL(
		source,
		args.includeDirections,
		args.strict,
		args.aspect,
		args.lossless,
		args.absoluteAsPercentage,
	)

	if args.stripMeta {
		result = stripConverterMeta(result)
	}

	if err := writeJSONFile(args.output, result, args.compact); err != nil {
		fmt.Fprintf(os.Stderr, "error: failed to write output file: %v\n", err)
		os.Exit(1)
	}

	printSubstitutionSummary()
}
