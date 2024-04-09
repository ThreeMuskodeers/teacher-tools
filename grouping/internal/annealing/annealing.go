package annealing

import (
	"maps"
	"math"
	"math/rand"

	"github.com/charmbracelet/log"
)

type Student struct {
	Id StudentId `json:"id"`
	// FirstName string
	// LastName  string
}

type RelationshipPair struct {
	FirstStudentId  StudentId `json:"firstStudentId"`
	SecondStudentId StudentId `json:"secondStudentId"`
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

	log.Debug("Objective Result", "solution", solution, "numViolations", numViolations, "variance", variance)

	return variance + numViolations // Lower variance is better, indicating more evenly sized groups.
}

func SimulatedAnnealing(students []Student, numGroups int, restrictions []RelationshipPair, maxTemp, minTemp float64, steps int) (Solution, float64) {
	log.Debug("Started SimulatedAnnealing", "students", students, "numGroups", numGroups, "restrictions", restrictions, "maxTemp", maxTemp, "minTemp", minTemp, "steps", steps)
	solution := make(Solution)
	for _, student := range students {
		solution[student.Id] = GroupId(rand.Intn(numGroups))
	}

	bestSolution := maps.Clone(solution)
	bestScore := objective(solution, numGroups, restrictions)

	for step := 0; step < steps; step++ {
		temp := changeTemp(maxTemp, minTemp, steps, step)

		newSolution, err := makeMove(maps.Clone(solution), numGroups)
		if err != nil {
			log.Error("Move failed", "error", err)
			continue
		}
		newScore := objective(newSolution, numGroups, restrictions)

		if newScore < bestScore || rand.Float64() < math.Exp((bestScore-newScore)/temp) {
			solution = newSolution
			if newScore < bestScore {
				bestSolution = maps.Clone(newSolution)
				bestScore = newScore
			}
		}
	}

	if bestScore > 0 {
		log.Warn("SimulatedAnnealing failed to find a perfect solution", "bestScore", bestScore)
	}
	return bestSolution, bestScore
}
