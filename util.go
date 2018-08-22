package main

import (
	"fmt"
	"strings"
)

// PrettyPrint ..
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
