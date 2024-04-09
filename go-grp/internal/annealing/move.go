package annealing

import (
	"errors"
	"math/rand"
)

func makeMove(solution Solution, numGroups int) (Solution, error) {
	if numGroups == 1 {
		return solution, nil
	}
	if numGroups == 0 {
		return nil, errors.New("numGroups must be positive")
	}

	groupSizes := make([]int, numGroups)
	for _, group := range solution {
		if group < 0 || group >= GroupId(numGroups) {
			return nil, errors.New("solution contains invalid group numbers")
		}
		groupSizes[group]++
	}

	// if the groups sizes are all equal (or as close as possible given the number of students and groups)
	// then we can just swap two students
	if allEqual(numGroups, len(solution), groupSizes) {
		return swapStudents(solution, numGroups)
	}

	return moveFromLargeGroup(solution, numGroups)
}

func allEqual(numGroups int, numStudents int, groupSizes []int) bool {
	// check if all elements are as close as possible due to the number of students and groups
	studentsPerGroup := numStudents / numGroups
	remainder := numStudents % numGroups

	extraAllowance := remainder

	for _, size := range groupSizes {
		if size > studentsPerGroup+1 {
			return false
		}
		if size < studentsPerGroup {
			return false
		}
		if size == studentsPerGroup+1 {
			if extraAllowance == 0 {
				return false
			}
			extraAllowance--
		}
	}
	return true
}

func swapStudents(solution Solution, numGroups int) (Solution, error) {
	// Select two random groups.
	group1 := GroupId(rand.Intn(numGroups))
	group2 := GroupId(rand.Intn(numGroups))
	for group1 == group2 {
		group2 = GroupId(rand.Intn(numGroups))
	}

	// Select a random student from each group.
	studentsInGroup1 := []StudentId{}
	studentsInGroup2 := []StudentId{}
	for student, group := range solution {
		if group == group1 {
			studentsInGroup1 = append(studentsInGroup1, student)
		} else if group == group2 {
			studentsInGroup2 = append(studentsInGroup2, student)
		}
	}

	if len(studentsInGroup1) == 0 || len(studentsInGroup2) == 0 {
		return nil, errors.New("no students found in one of the groups")
	}

	student1 := studentsInGroup1[rand.Intn(len(studentsInGroup1))]
	student2 := studentsInGroup2[rand.Intn(len(studentsInGroup2))]

	// Swap the students.
	solution[student1], solution[student2] = solution[student2], solution[student1]

	return solution, nil
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
		if size < groupSizes[smallestGroup] {
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
