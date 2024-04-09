package annealing

import (
	"math"
)

func linearChangeTemp(maxTemp, minTemp float64, steps, step int) float64 {
	delta := maxTemp - minTemp
	return maxTemp - delta*float64(step)/float64(steps)
}

// logarithmicChangeTemp decreases the temperature using a logarithmic scale based on the current step.
func logarithmicChangeTemp(maxTemp, minTemp float64, steps, step int) float64 {
	// Ensuring we don't divide by zero or take the log of a non-positive number.
	if step == 0 {
		return maxTemp
	}
	scale := math.Log(float64(step + 1)) // +1 to adjust for zero-based step index.
	normalizedScale := scale / math.Log(float64(steps))
	return maxTemp - (maxTemp-minTemp)*normalizedScale
}

// changeTemp decides which temperature change function to use.
// This function would typically involve some logic or condition to choose between methods.
// For demonstration, let's assume it always uses the linear method.
func changeTemp(maxTemp, minTemp float64, steps, step int) float64 {
	return linearChangeTemp(maxTemp, minTemp, steps, step)
}
