package annealing

import (
	"fmt"
	"math"
	"testing"
)

func TestObjectiveFunction(t *testing.T) {
	// Setup test cases
	tests := []struct {
		solution     Solution
		students     []Student
		numGroups    int
		restrictions []RelationshipPair
		expected     float64
	}{
		{
			Solution{
				1: 0,
				2: 0,
				3: 1,
				4: 1,
			},
			[]Student{{1, "Alice", "Last"}, {2, "Bob", "Last"}, {3, "Charlie", "Last"}, {4, "Dana", "Last"}}, // Students
			2,                                  // Number of groups
			[]RelationshipPair{{1, 2}, {3, 4}}, // Restrictions
			2,                                  // Expected result: invalid solution due to restriction violation
		},
		{
			Solution{
				1: 0,
				2: 1,
				3: 0,
				4: 1,
			},
			[]Student{{1, "Alice", "Last"}, {2, "Bob", "Last"}, {3, "Charlie", "Last"}, {4, "Dana", "Last"}},
			2,
			[]RelationshipPair{{1, 2}, {3, 4}},
			0.0, // Expected result: perfect variance, evenly sized groups
		},
	}

	for _, tc := range tests {
		result := objective(tc.solution, tc.numGroups, tc.restrictions)
		if result != tc.expected {
			t.Errorf("Expected variance %v, got %v", tc.expected, result)
		}
	}
}

func TestAnnealing(t *testing.T) {
	// Setup test cases
	tests := []struct {
		students     []Student
		numGroups    int
		restrictions []RelationshipPair
		maxTemp      float64
		minTemp      float64
		steps        int
	}{
		{
			[]Student{{1, "Alice", "Last"}, {2, "Bob", "Last"}, {3, "Charlie", "Last"}, {4, "Dana", "Last"}},
			2,
			[]RelationshipPair{{1, 2}, {3, 4}},
			10.0,
			0.1,
			1000,
		},
		{
			[]Student{
				{1, "Alice", "Last"},
				{2, "Bob", "Last"},
				{3, "Charlie", "Last"},
				{4, "Dana", "Last"},
				{5, "Eve", "Last"},
				{6, "Frank", "Last"},
				{7, "Grace", "Last"},
				{8, "Hank", "Last"},
				{9, "Ivy", "Last"},
				{10, "Jack", "Last"},
				{11, "Karl", "Last"},
				{12, "Liam", "Last"},
				{13, "Mia", "Last"},
				{14, "Nina", "Last"},
				{15, "Oscar", "Last"},
				{16, "Pam", "Last"},
				{17, "Quinn", "Last"},
				{18, "Ralph", "Last"},
				{19, "Sara", "Last"},
				{20, "Tina", "Last"},
				{21, "Ursula", "Last"},
			},
			3,
			[]RelationshipPair{{1, 2}, {3, 4}, {6, 3}, {10, 2}},
			10.0,
			0.1,
			1000,
		},
	}
	for _, tc := range tests {
		for range 1000 {
			solution, bestScore := SimulatedAnnealing(tc.students, tc.numGroups, tc.restrictions, tc.maxTemp, tc.minTemp, tc.steps)
			if math.IsInf(bestScore, 1) {
				t.Errorf("Invalid solution found")
			}
			if bestScore < 0 {
				t.Errorf("Negative score found")
			}
			if bestScore > 0 {
				fmt.Println("TESTING")
				fmt.Println(createGroupList(solution, tc.numGroups))
				t.Errorf("Test should be simple enough for perfect score of 0 variance")
			}

		}
	}
}

func createGroupList(solution Solution, numGroups int) [][]StudentId {
	groups := make([][]StudentId, numGroups)
	for student, group := range solution {
		groups[group] = append(groups[group], student)
	}
	return groups
}
