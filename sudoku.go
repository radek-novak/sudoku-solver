package main

import (
	"os"
	"sync"
	"time"
)

func worker(queue *[]Board, mutex *sync.Mutex, done chan<- Board) {
	for {

		mutex.Lock()
		lastIx := len(*queue) - 1
		// fmt.Println("lastIx", lastIx)
		board := (*queue)[lastIx]
		*queue = (*queue)[:lastIx]
		mutex.Unlock()

		isValid, isSolved := CheckBoard(board)

		if isSolved {
			PrettyPrint(board)
			os.Exit(0)
			// done <- board
		}

		if isValid {
			boards := GetNextBoards(board)
			mutex.Lock()
			*queue = append(*queue, boards...)
			mutex.Unlock()
		}
	}
}

func main() {
	var mutex = sync.Mutex{}
	board := ReadInput()
	done := make(chan Board)
	queue := make([]Board, 1)
	queue = append(queue, board)

	for i := 0; i < 4; i++ {
		go worker(&queue, &mutex, done)

		time.Sleep(10 * time.Millisecond)
	}
	// PrettyPrint(<-done)
}
