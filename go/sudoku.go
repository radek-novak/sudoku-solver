package main

import (
	"fmt"
	"os"
	"sync/atomic"
)

func worker(boardToSolve <-chan Board, nextBoardsChan chan<- []Board) {
	for board := range boardToSolve {
		isValid, isSolved := CheckBoard(board)
		if isSolved {
			PrettyPrint(board)
			os.Exit(0)
		}
		if isValid {
			nextBoardsChan <- GetNextBoards(board)
		}
	}
}

func main() {
	var ops int32

	board := ReadInput()
	boardToSolve := make(chan Board, 49900)
	nextBoardsChan := make(chan []Board, 300)

	for w := 0; w < 4; w++ {
		go worker(boardToSolve, nextBoardsChan)
	}

	boardToSolve <- board

	for nextBoards := range nextBoardsChan {
		atomic.AddInt32(&ops, 1)
		i := atomic.LoadInt32(&ops)
		if i%100 == 0 {
			fmt.Println(i)
		}

		for _, board := range nextBoards {
			boardToSolve <- board
		}
	}
}
