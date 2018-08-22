package main

func main() {
	board := ReadInput()
	queue := []Board{}
	queue = append(queue, board)

	for {
		lastIx := len(queue) - 1
		currentBoard := queue[lastIx]
		isValid, isSolved := CheckBoard(currentBoard)

		if isSolved {
			PrettyPrint(currentBoard)
			return
		}

		queue = queue[:lastIx]

		if isValid {
			queue = append(queue, GetNextBoards(currentBoard)...)
		}
	}
}
