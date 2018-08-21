package main

import "fmt"

// Board ..
type Board [9][9]int8

// CheckNine (isValid, isSolved)
func CheckNine(nine [9]int8) (bool, bool) {
	var counter [10]bool

	for i := 0; i < 9; i++ {
		currentN := nine[i]
		if currentN > 0 && counter[currentN] {
			return false, false
		}
		counter[currentN] = true
	}

	return true, !counter[0]
}

// CheckLines (isValid, isSolved)
func CheckLines(board Board) (bool, bool) {
	solvedCount := 0
	for _, line := range board {
		isValid, isSolved := CheckNine(line)
		if !isValid {
			return false, false
		}

		if isSolved {
			solvedCount++
		}
	}

	if solvedCount == 9 {
		return true, true
	}

	return true, false
}

func main() {
	board := ReadInput()
	fmt.Println(board)

	fmt.Println(CheckLines(board))

}
