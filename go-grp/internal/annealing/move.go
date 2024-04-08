package annealing

import (
	"errors"
	"fmt"
	"math/rand"
)

func makeMove(solution Solution, numGroups int) (Solution, error) {
	return moveFromLargeGroup(solution, numGroups)
}

// moveFromLargeGroup moves a student from the largest to the smallest group.
func moveFromLargeGroup(solution Solution, numGroups int) (Solution, error) {
	if numGroups <= 0 {
		return nil, errors.New("numGroups must be positive")
	}
	groupSizes := make([]int, numGroups)
	for _, group := range solution {
		if group < 0 || group >= GroupId(numGroups) {
			return nil, errors.New("solution contains invalid group numbers")
		}
		groupSizes[group]++
	}

	// Identifying the largest and smallest groups.
	largestGroup, smallestGroup := GroupId(0), GroupId(0)
	for i, size := range groupSizes {
		if size >= groupSizes[largestGroup] {
			largestGroup = GroupId(i)
		}
		if size < groupSizes[smallestGroup] || groupSizes[smallestGroup] == 0 {
			smallestGroup = GroupId(i)
		}
	}

	// Move a random student from the largest to the smallest group.
	studentsInLargestGroup := []StudentId{}
	for student, group := range solution {
		if group == largestGroup {
			studentsInLargestGroup = append(studentsInLargestGroup, student)
		}
	}

	if len(studentsInLargestGroup) == 0 {
		return nil, errors.New("no students found in the largest group")
	}

	randomStudent := studentsInLargestGroup[rand.Intn(len(studentsInLargestGroup))]
	fmt.Println("Moving student", randomStudent, "from group", largestGroup, "to group", smallestGroup)
	solution[randomStudent] = smallestGroup

	return solution, nil
}

// randomMove attempts to randomly reassign a student to a different group.
func randomMove(solution []int, numGroups int) ([]int, error) {
	if len(solution) == 0 {
		return nil, errors.New("solution cannot be empty")
	}

	index := rand.Intn(len(solution))
	newGroup := rand.Intn(numGroups)
	solution[index] = newGroup

	return solution, nil
}
