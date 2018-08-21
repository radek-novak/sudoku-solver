package main

import (
	"testing"
)

func TestCheckNine(t *testing.T) {
	t.Run("check valid", func(t *testing.T) {
		isValid, isSolved := CheckNine([]int8{0, 0, 1, 2, 3, 4, 0, 5, 6})
		if !isValid || isSolved {
			t.Errorf("Should've been invalid and not solved but was %t and %t", isValid, isSolved)
		}
	})

	t.Run("check solved", func(t *testing.T) {
		isValid, isSolved := CheckNine([]int8{1, 2, 3, 4, 7, 8, 9, 5, 6})

		if !isValid || !isSolved {
			t.Errorf("Should've been valid and solved but was %t and %t", isValid, isSolved)
		}
	})

	t.Run("check false", func(t *testing.T) {
		isValid, isSolved := CheckNine([]int8{1, 2, 3, 4, 7, 8, 9, 5, 4})

		if isValid || isSolved {
			t.Errorf("Should've been not valid and not solved but was %t and %t", isValid, isSolved)
		}
	})
}
