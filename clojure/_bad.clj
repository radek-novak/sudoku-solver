
; (defn row-valid? [nine]
;   (loop [i 0 occur #{}]
;     (let [current (get nine i)]
;       (cond
;         (contains? occur current) false
;         (> i 8) true
;         (= current 0) (recur (inc i) occur)
;         :else (recur (inc i) (conj occur current))))))


; (defn get-squares [matrix]
;   (map
;    (fn [x]
;      (vec (map
;            (fn [y] (get-square sudoku x y)) (range 0 8 3))))
;    (range 0 8 3)))

; (defn get-diagonal [matrix]
;   (loop [i 0 ret (get matrix 0)]
;     (if
;      (> i 8) ret
;      (recur (inc i) (assoc ret i (get (get matrix i) i))))))

(def valid-sudoku
  [[0 4 0 0 0 0 1 7 9]
   [0 0 2 0 0 8 0 5 4]
   [0 0 6 0 0 5 0 0 8]
   [0 8 0 0 7 0 9 1 0]
   [0 5 0 0 9 0 0 3 0]
   [0 1 9 0 6 0 0 4 0]
   [3 0 0 4 0 0 7 0 0]
   [5 7 0 1 0 0 2 0 0]
   [9 2 8 0 0 0 0 6 0]])
(def solved-sudoku
  [[2 3 7 9 5 8 6 1 4]
   [5 1 9 6 4 3 7 8 2]
   [8 4 6 1 2 7 5 9 3]
   [1 8 3 7 9 5 2 4 6]
   [9 2 5 8 6 4 3 7 1]
   [6 7 4 2 3 1 9 5 8]
   [4 6 8 3 7 9 1 2 5]
   [3 9 1 5 8 2 4 6 7]
   [7 5 2 4 1 6 8 3 9]])
(def almost-solved
  [[2 3 7 9 5 8 6 1 4]
   [5 1 9 6 4 3 7 8 2]
   [8 4 6 1 2 7 5 9 3]
   [1 8 3 7 9 5 2 4 6]
   [9 2 5 8 6 4 3 7 1]
   [6 7 4 2 3 1 9 5 8]
   [4 6 8 3 7 9 1 2 5]
   [3 9 1 5 8 2 4 6 7]
   [7 5 2 4 1 6 8 0 0]])

(def valid-row [1 4 0 0 0 5 0 2 3])
(def solved-row [1 4 7 6 9 5 8 2 3])
(def invalid-row [1 4 4 0 0 5 0 2 3])
