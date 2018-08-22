package main

import "fmt"

// Board ..
type Board [9][9]int8

func main() {
	board := ReadInput()

	fmt.Println(CheckBoard(board))
}
