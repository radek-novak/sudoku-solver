(ns sudoku)

(def one-to-nine (take 9 (iterate inc 1)))

(defn row-complete? [row]
  (= 9 (count (set (concat one-to-nine row)))))

(defn row-valid? [nine]
  (let [filtered (filter #(not= 0 %) nine)]
    (= (count filtered) (count (set filtered)))))

(defn get-col [matrix col]
  (into (vector) (map #(get % col) matrix)))

(defn rotate-matrix [matrix]
  (loop [i 0 ret []]
    (if
     (> i 8) ret
     (recur (inc i) (conj ret (get-col matrix i))))))

(defn get-diagonal [matrix]
  (map #(get (get matrix %) %) (take 9 (iterate inc 0))))

(defn get-three [row off] (subvec row off (+ off 3)))

(defn get-square [matrix x y]
  (let [rows (subvec matrix y (+ y 3))]
    (vec (flatten (map #(get-three % x) rows)))))

(defn get-squares [matrix]
  (map
   (fn [x] (get-square matrix (* 3 (quot x 3)) (* 3 (mod x 3))))
   (take 9 (iterate inc 0))))


(defn get-all-rows [matrix]
  (let [columns (rotate-matrix matrix)]
    (concat matrix
            columns
            (get-squares matrix)
            ; [(get-diagonal matrix)]
            ; [(get-diagonal columns)]
            )))

(defn sudoku-valid? [matrix]
  (every? row-valid? (get-all-rows matrix)))

(defn sudoku-solved? [matrix]
  (and
   (sudoku-valid? matrix)
   (every? row-complete? matrix)))

(defn find-next-zero [matrix]
  (first
   (for [[x row] (map-indexed vector matrix)
         [y val] (map-indexed vector row)
         :when (= 0 val)]
     [x y])))

(defn generate-boards [matrix]
  (map #(assoc-in matrix (find-next-zero matrix) %)
       one-to-nine))

(defn solve-sudoku [matrix]
  (loop [queue (list matrix) current matrix]
    (cond
      (nil? current) nil
      (sudoku-solved? current) current
      (sudoku-valid? current)
      (recur
       (concat (rest queue) (generate-boards current))
       (first queue))
      :else (recur (rest queue) (first queue)))))

(defn load-sudoku-file [path]
  (->> path
       slurp
       clojure.string/split-lines
       (map clojure.string/trim)
       (map #(clojure.string/split % #" "))
       (map #(map read-string %))
       (map #(into (vector) %))
       (into (vector))))

(defn print-board [matrix]
  (->> matrix
       (map #(clojure.string/join " " %))
       (clojure.string/join "\n")
       println))

(->> (first *command-line-args*)
     load-sudoku-file
     solve-sudoku
     print-board)