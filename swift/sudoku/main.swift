import Foundation

let validboardstring = """
0 4 0 0 0 0 1 7 9
0 0 2 0 0 8 0 5 4
0 0 6 0 0 5 0 0 8
0 8 0 0 7 0 9 1 0
0 5 0 0 9 0 0 3 0
0 1 9 0 6 0 0 4 0
3 0 0 4 0 0 7 0 0
5 7 0 1 0 0 2 0 0
9 2 8 0 0 0 0 6 0
"""

enum InvalidState: Error {
    case runtimeError(String)
}

typealias Row = [Int]
typealias Board = [Row]

let validboard:  Board = [
    [0,4,0,0,0,0,1,7,9],
    [0,0,2,0,0,8,0,5,4],
    [0,0,6,0,0,5,0,0,8],
    [0,8,0,0,7,0,9,1,0],
    [0,5,0,0,9,0,0,3,0],
    [0,1,9,0,6,0,0,4,0],
    [3,0,0,4,0,0,7,0,0],
    [5,7,0,1,0,0,2,0,0],
    [9,2,8,0,0,0,0,6,0]
]
let harder = [[0,0,2,0,0,4,0,0,0],
              [0,0,8,6,0,0,0,0,0],
              [0,9,0,0,0,0,0,3,0],
              [8,0,0,0,6,2,0,0,0],
              [1,4,0,0,3,0,5,0,9],
              [0,7,0,0,0,0,0,0,0],
              [0,3,5,0,0,8,6,0,7],
              [0,0,0,0,0,0,0,4,2],
              [0,0,0,9,0,1,0,0,3],

              ]
let invalidboard:  Board = [
    [4,4,0,0,0,0,1,7,9],
    [0,0,2,0,0,8,0,5,4],
    [0,0,6,0,0,5,0,0,8],
    [0,8,0,0,7,0,9,1,0],
    [0,5,0,0,9,0,0,3,0],
    [0,1,9,0,6,0,0,4,0],
    [3,0,0,4,0,0,7,0,0],
    [5,7,0,1,0,0,2,0,0],
    [9,2,8,0,0,0,0,6,0]
]


func getRow (board: Board, idx: Int ) -> Row {
    let copy = board[idx]
    
    return copy;
}

func getCol (board: Board, idx: Int ) -> Row {
    var col: Row = []
    
    for row in board {
        col.append(row[idx])
    }
    
    return col;
}

let FIRST_ROW = [0, 1, 2, 9, 10, 11, 18, 19, 20];
func getSquare(input: Board, sq_idx: Int) -> Row {
    var sq: [Int] = []
    
    let xStart = (sq_idx % 3) * 3;
    let yStart = (sq_idx / 3) * 3;
    let xEnd = xStart + 2;
    let yEnd = yStart + 2;
    //print(xStart, xEnd, yStart,yEnd)

    for j in yStart...yEnd {
        for i in xStart...xEnd {
            sq.append(input[j][i])
        }
    }
    return sq;
}


func validNine (nine: Row) -> Bool {
    var res = Array(repeating: false, count: 10)
    
    for num in nine {
        if num == 0 {
            continue
        }
        if res[num] {
            return false
        }
        res[num] = true
    }
    
    return true
}

func isBoardValid (board: Board) -> Bool {
    for i in 0...8 {
        if !validNine(nine: getSquare(input: board, sq_idx: i)) ||
            !validNine(nine: getCol(board: board, idx: i)) ||
            !validNine(nine: getRow(board: board, idx: i)) {
            return false
        }
    }
    
    return true
}

func getNextBoards (board: Board) -> [Board] {
    var newboards: [Board] = []
    var x = 0
    var y = 0

done: for i in 0...8 {
        
        for j in 0...8 {
            if board[j][i] == 0 {
                y = j
                x = i
                break done
            }
        }
    }
    

    
    for i in 1...9 {
        var newboard = board
        newboard[y][x] = i
        
        newboards.append(newboard)
    }

    return newboards
}

func isSolved (board: Board) -> Bool {
    for i in 0...8 {
        for j in 0...8 {
            if board[j][i] == 0 {
                return false
            }
        }
    }
    
    return true
}

func solveBoard(board: Board)throws -> Board{
    var queue: [Board] = [board]

    while true {
        let last = queue.popLast()
        if let unwrapped = last {
            if (isBoardValid(board: unwrapped)) {
                if isSolved(board: unwrapped) {
                    return unwrapped
                }
                let nextboards = getNextBoards(board: unwrapped)
                queue.append(contentsOf: nextboards.filter { isBoardValid(board: $0) })
            }
        } else {
            throw InvalidState.runtimeError("invalid state")
        }
    }
}

func parseBoardString (boardString: String) -> Board {
    var newboard: Board = []
    var row: Row = []

    for char in boardString {
        if char.isNumber {
            let num = Int(String(char))
            
            if let newnumber = num {
                row.append(newnumber)
                
                if row.count == 9 {
                    newboard.append(row)
                    row = []
                }
            }
        }
    }

    return newboard
}

func printBoard (board: Board) {
    for row in board {
        print(row.map { String($0) } .joined(separator: " "))
    }
}


//
//print(getRow(board: validboard, idx: 2))
//print(getCol(board: validboard, idx: 2))
//
//print(validNine(nine: getRow(board: validboard, idx: 2)))
//print(validNine(nine: [1,0,0,0,0,0,0,4,1]))
//print(getSquare(input: validboard, sq_idx: 1))
//print(getSquare(input: validboard, sq_idx: 3))
//
//print(getSquare(input: validboard, sq_idx: 8))
//print(isBoardValid(board: validboard))
//print(isBoardValid(board: invalidboard))
//print(getNextBoards(board: validboard))

//try printBoard(board: solveBoard(board: validboard))

func readStdin() -> String {
    var s = ""

    while let line = readLine() {
       s += line
    }

    return s
}

func main() {
    let boardstring = readStdin()
    let board = parseBoardString(boardString: boardstring)

    do {
        try printBoard(board: solveBoard(board: board))
    } catch {
        print("failed to solve")
    }
    
}

main()
