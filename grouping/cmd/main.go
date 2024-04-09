package main

import (
	"encoding/json"
	"net/http"
	"teachertools/grouping/internal/annealing"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
)

func main() {
	r := chi.NewRouter()
	r.Use(middleware.Logger)
	r.Get("/", func(w http.ResponseWriter, r *http.Request) {
		w.Write([]byte("welcome"))
	})
	r.Get("/group", getNewGroup)
	http.ListenAndServe(":3000", r)
}

func getNewGroup(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("content-type", "application/json")
	var request GroupRequest
	err := json.NewDecoder(r.Body).Decode(&request)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	groups, score := annealing.SimulatedAnnealing(request.Students, request.NumGroups, request.Relationships, 10, 0.1, 1000)
	response := GroupResponse{
		Groups: groups,
		Score:  score,
	}
	json.NewEncoder(w).Encode(response)
}

type GroupRequest struct {
	Students      []annealing.Student          `json:"students"`
	Relationships []annealing.RelationshipPair `json:"relationships"`
	NumGroups     int                          `json:"numGroups"`
}

type GroupResponse struct {
	Groups map[annealing.StudentId]annealing.GroupId `json:"groups"`
	Score  float64                                   `json:"score"`
}
