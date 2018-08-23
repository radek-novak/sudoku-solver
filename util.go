package main

import (
	"fmt"
	"strings"
)

// PrettyPrint prints the board in the following format:
//
// 8 6 5 | 7 2 4 | 1 3 9
// 9 4 2 | 8 3 1 | 7 6 5
// 7 1 3 | 9 5 6 | 8 4 2
// ---------------------
// 2 5 8 | 1 9 3 | 4 7 6
// 3 7 4 | 5 6 2 | 9 1 8
// 1 9 6 | 4 7 8 | 5 2 3
// ---------------------
// 4 3 9 | 6 8 7 | 2 5 1
// 5 2 7 | 3 1 9 | 6 8 4
// 6 8 1 | 2 4 5 | 3 9 7
//
//
func PrettyPrint(board Board) {
	for i, line := range board {
		for j := range line {
			fmt.Print(board[i][j], " ")
			if (j+1)%3 == 0 && j != 8 {
				fmt.Print("| ")
			}
		}
		fmt.Println()
		if (i+1)%3 == 0 && i != 8 {
			fmt.Print(strings.Repeat("-", 21))
			fmt.Println()
		}
	}
	fmt.Println()
	fmt.Println()
}
