
(defn field [] (apply hash-set (take 9 (iterate inc 1))))
(defn row [] (apply vector (repeatedly 9 field)))
(defn board [] (apply vector (repeatedly 9 row)))
