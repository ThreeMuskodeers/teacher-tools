package annealing

import (
	"fmt"
	"maps"
	"math"
	"math/rand"
)

type Student struct {
	Id        StudentId
	FirstName string
	LastName  string
}

type RelationshipPair struct {
	FirstStudentId  StudentId
	SecondStudentId StudentId
}
type (
	StudentId int
	GroupId   int
	Solution  map[StudentId]GroupId
)

func objective(solution Solution, numGroups int, restrictions []RelationshipPair) float64 {
	// Check if the solution violates any restrictions.
	var numViolations float64 = 0
	for _, restriction := range restrictions {
		if solution[restriction.FirstStudentId] == solution[restriction.SecondStudentId] {
			numViolations++
		}
	}

	// Compute the size of each group.
	groupSizes := make([]int, numGroups)
	for _, group := range solution {
		groupSizes[group]++
	}

	// Compute the variance of the group sizes.
	var meanSize float64
	for _, size := range groupSizes {
		meanSize += float64(size)
	}
	meanSize /= float64(numGroups)

	var variance float64
	for _, size := range groupSizes {
		variance += math.Pow(float64(size)-meanSize, 2)
	}
	variance /= float64(numGroups)

	return variance + (2 * numViolations) // Lower variance is better, indicating more evenly sized groups.
}

func SimulatedAnnealing(students []Student, numGroups int, restrictions []RelationshipPair, maxTemp, minTemp float64, steps int) (Solution, float64) {
	solution := make(Solution)
	for _, student := range students {
		solution[student.Id] = GroupId(rand.Intn(numGroups))
	}

	bestSolution := maps.Clone(solution)
	bestScore := objective(solution, numGroups, restrictions)

	for step := 0; step < steps; step++ {
		temp := changeTemp(maxTemp, minTemp, steps, step)

		newSolution, err := makeMove(solution, numGroups)
		if err != nil {
			fmt.Println("Move failed:", err)
			continue
		}
		newScore := objective(newSolution, numGroups, restrictions)

		if newScore < bestScore || rand.Float64() < math.Exp((bestScore-newScore)/temp) {
			solution = newSolution
			if newScore < bestScore {
				bestSolution = newSolution
				bestScore = newScore
			}
		}
	}

	return bestSolution, bestScore
}
