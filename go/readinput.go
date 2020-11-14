package main

import (
	"bufio"
	"io"
	"os"
	"strconv"
)

// ReadInput fn
func ReadInput() [9][9]int8 {
	stdin, err := os.Stdin.Stat()

	if err != nil {
		panic(err)
	}

	if stdin.Mode()&os.ModeCharDevice != 0 || stdin.Size() <= 0 {
		panic("The command is intended to work with pipes.")
	}

	reader := bufio.NewReader(os.Stdin)
	var board [9][9]int8
	var lines []string

	for {
		input, err := reader.ReadString('\n')
		if err != nil && err == io.EOF {
			break
		}
		lines = append(lines, input)
	}

	for lineIx, line := range lines {
		strLine := string(line)
		carIx := 0
		for _, c := range strLine {
			digit, err := strconv.Atoi(string(c))

			if err != nil {
				continue
			}

			board[lineIx][carIx] = int8(digit)
			carIx++
		}

		if len(board[lineIx]) != 9 {
			panic("Wrong number of digits in a line")
		}

		carIx = 0
	}

	if len(board) != 9 {
		panic("Wrong number of lines")
	}

	return board
}
