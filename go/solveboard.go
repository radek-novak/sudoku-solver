package main

// FindNextZero ..
func FindNextZero(board Board) (ret [2]int8) {
	for i, line := range board {
		for j := range line {
			if board[i][j] == 0 {
				ret[0] = int8(i)
				ret[1] = int8(j)
				return
			}
		}
	}
	panic("Didn't find zero element")
}

// GetNextBoards ..
func GetNextBoards(board Board) (ret []Board) {
	coords := FindNextZero(board)

	for i := int8(0); i < 9; i++ {
		newBoard := board
		newBoard[coords[0]][coords[1]] = i + 1
		ret = append(ret, newBoard)
	}

	return
}
