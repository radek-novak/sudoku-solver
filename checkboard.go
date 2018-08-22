package main

import "fmt"

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

// FlipBoard ...
func FlipBoard(board Board) (rotatedBoard Board) {
	for lineIx, line := range board {
		for colIx, digit := range line {
			rotatedBoard[colIx][lineIx] = digit
		}
	}
	return
}

// CheckCols ...
func CheckCols(board Board) (bool, bool) {
	return CheckLines(FlipBoard(board))
}

func unwrapMiniSquares(board Board) (res [9][9]int8) {
	currentIndex := 0

	for i := 0; i < 9; i += 3 {
		for j := 0; j < 9; j += 3 {
			var line [9]int8
			currentLineIndex := 0

			for k := 0; k < 3; k++ {
				for l := 0; l < 3; l++ {
					line[currentLineIndex] = board[i+k][j+l]
					currentLineIndex++
				}
			}

			res[currentIndex] = line
			currentIndex++
			currentLineIndex = 0
		}
	}

	return
}

// CheckMiniSquares ..
func CheckMiniSquares(board Board) (bool, bool) {
	return CheckLines(unwrapMiniSquares(board))
}

// CheckBoard ..
func CheckBoard(board Board) (bool, bool) {
	isLinesValid, isLinesSolved := CheckLines(board)
	if !isLinesValid {
		fmt.Println("isLinesVali not")
		return false, false
	}

	isColsValid, isColsSolved := CheckCols(board)
	if !isColsValid {
		fmt.Println("isColsValid not")
		return false, false
	}

	isSquaresValid, isSquaresSolved := CheckMiniSquares(board)
	if !isSquaresValid {
		return false, false
	}

	return true, isLinesSolved && isColsSolved && isSquaresSolved
}
